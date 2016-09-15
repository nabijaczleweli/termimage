//! Main functions doing actual work.
//!
//! Use `guess_format()` to get teh image format from a path,
//! then read the image using `load_image()`,
//! resize it to terminal size with `resize_image()`
//! and display it with `write_[no_]ansi()`.

use self::super::util::{ANSI_BG_COLOUR_ESCAPES, ANSI_COLOUR_ESCAPES, ANSI_BG_COLOURS, ANSI_COLOURS, JPEG_MAGIC, BMP_MAGIC, ICO_MAGIC, GIF_MAGIC, PNG_MAGIC,
                        closest_colour};
use image::{self, GenericImage, DynamicImage, ImageFormat, FilterType, Pixel};
use std::io::{BufReader, Write, Read};
use self::super::Outcome;
use std::path::PathBuf;
use std::fs::File;

mod no_ansi;

pub use self::no_ansi::write_no_ansi;


/// Guess the image format from its extension or magic.
///
/// # Examples
///
/// Correct:
///
/// ```
/// # extern crate image;
/// # extern crate termimage;
/// # use image::ImageFormat;
/// # use std::path::PathBuf;
/// # use termimage::ops::guess_format;
/// # fn main() {
/// assert_eq!(guess_format(&(String::new(), PathBuf::from("img.png"))), Ok(ImageFormat::PNG));
/// assert_eq!(guess_format(&(String::new(), PathBuf::from("img.jpg"))), Ok(ImageFormat::JPEG));
/// assert_eq!(guess_format(&(String::new(), PathBuf::from("img.gif"))), Ok(ImageFormat::GIF));
/// assert_eq!(guess_format(&(String::new(), PathBuf::from("img.bmp"))), Ok(ImageFormat::BMP));
/// assert_eq!(guess_format(&(String::new(), PathBuf::from("img.ico"))), Ok(ImageFormat::ICO));
/// # }
/// ```
///
/// Incorrect:
///
/// ```
/// # use std::path::PathBuf;
/// # use termimage::Outcome;
/// # use termimage::ops::guess_format;
/// assert_eq!(guess_format(&("src/ops.rs".to_string(), PathBuf::from("src/ops/mod.rs"))),
/// Err(Outcome::GuessingFormatFailed("src/ops.rs".to_string())));
/// ```
pub fn guess_format(file: &(String, PathBuf)) -> Result<ImageFormat, Outcome> {
    file.1
        .extension()
        .and_then(|ext| match &ext.to_str().unwrap().to_lowercase()[..] {
            "png" => Some(Ok(ImageFormat::PNG)),
            "jpg" | "jpeg" | "jpe" | "jif" | "jfif" | "jfi" => Some(Ok(ImageFormat::JPEG)),
            "gif" => Some(Ok(ImageFormat::GIF)),
            "webp" => Some(Ok(ImageFormat::WEBP)),
            "ppm" => Some(Ok(ImageFormat::PPM)),
            "tiff" | "tif" => Some(Ok(ImageFormat::TIFF)),
            "tga" => Some(Ok(ImageFormat::TGA)),
            "bmp" | "dib" => Some(Ok(ImageFormat::BMP)),
            "ico" => Some(Ok(ImageFormat::ICO)),
            "hdr" => Some(Ok(ImageFormat::HDR)),
            _ => None,
        })
        .unwrap_or_else(|| {
            let mut buf = [0; 32];
            let read = try!(File::open(&file.1).map_err(|_| Outcome::OpeningImageFailed(file.0.clone()))).read(&mut buf).unwrap();
            let buf = &buf[..read];

            if buf.len() >= PNG_MAGIC.len() && &buf[..PNG_MAGIC.len()] == PNG_MAGIC {
                Ok(ImageFormat::PNG)
            } else if buf.len() >= JPEG_MAGIC.len() && &buf[..JPEG_MAGIC.len()] == JPEG_MAGIC {
                Ok(ImageFormat::JPEG)
            } else if buf.len() >= GIF_MAGIC.len() && &buf[..GIF_MAGIC.len()] == GIF_MAGIC {
                Ok(ImageFormat::GIF)
            } else if buf.len() >= BMP_MAGIC.len() && &buf[..BMP_MAGIC.len()] == BMP_MAGIC {
                Ok(ImageFormat::BMP)
            } else if buf.len() >= ICO_MAGIC.len() && &buf[..ICO_MAGIC.len()] == ICO_MAGIC {
                Ok(ImageFormat::ICO)
            } else {
                Err(Outcome::GuessingFormatFailed(file.0.clone()))
            }
        })
}

/// Load an image from the specified file as the specified format.
///
/// Get the image fromat with `guess_format()`.
pub fn load_image(file: &(String, PathBuf), format: ImageFormat) -> Result<DynamicImage, Outcome> {
    Ok(image::load(BufReader::new(try!(File::open(&file.1).map_err(|_| Outcome::OpeningImageFailed(file.0.clone())))),
                   format)
        .unwrap())
}

/// Get the image size to downscale to, given its size, the terminal's size and whether to preserve its aspect.
///
/// The resulting image size is twice as tall as the terminal size because we print two pixels per cell (height-wise).
pub fn image_resized_size(size: (u32, u32), term_size: (u32, u32), preserve_aspect: bool) -> (u32, u32) {
    if !preserve_aspect {
        return (term_size.0, term_size.1 * 2);
    }

    let nwidth = term_size.0;
    let nheight = term_size.1 * 2;
    let (width, height) = size;

    let ratio = width as f32 / height as f32;
    let nratio = nwidth as f32 / nheight as f32;

    let scale = if nratio > ratio {
        nheight as f32 / height as f32
    } else {
        nwidth as f32 / width as f32
    };

    ((width as f32 * scale) as u32, (height as f32 * scale) as u32)
}

/// Resize the specified image to the specified size.
pub fn resize_image(img: &DynamicImage, size: (u32, u32)) -> DynamicImage {
    img.resize_exact(size.0, size.1, FilterType::Nearest)
}

/// Display the specified image in the default console using ANSI escape codes.
pub fn write_ansi<W: Write>(out: &mut W, img: &DynamicImage) {
    let (width, height) = img.dimensions();
    let term_h = height / 2;

    for y in 0..term_h {
        let upper_y = y * 2;
        let lower_y = upper_y + 1;

        for x in 0..width {
            let closest_upper_clr = closest_colour(img.get_pixel(x, upper_y).to_rgb(), ANSI_COLOURS);
            let closest_lower_clr = closest_colour(img.get_pixel(x, lower_y).to_rgb(), ANSI_BG_COLOURS);

            write!(out,
                   "{}{}\u{2580}", // ▀
                   ANSI_COLOUR_ESCAPES[closest_upper_clr],
                   ANSI_BG_COLOUR_ESCAPES[closest_lower_clr])
                .unwrap();
        }
        writeln!(out, "{}{}", ANSI_COLOUR_ESCAPES[15], ANSI_BG_COLOUR_ESCAPES[0]).unwrap();
    }
}

/// Display the specified image in the default console using ANSI 24-bit escape colour codes.
pub fn write_ansi_truecolor<W: Write>(out: &mut W, img: &DynamicImage) {
    let (width, height) = img.dimensions();
    let term_h = height / 2;

    for y in 0..term_h {
        let upper_y = y * 2;
        let lower_y = upper_y + 1;

        for x in 0..width {
            let upper_pixel = img.get_pixel(x, upper_y).to_rgb();
            let lower_pixel = img.get_pixel(x, lower_y).to_rgb();

            write!(out,
                   "\x1B[38;2;{};{};{}m\
                    \x1B[48;2;{};{};{}m\u{2580}", // ▀
                   upper_pixel[0],
                   upper_pixel[1],
                   upper_pixel[2],
                   lower_pixel[0],
                   lower_pixel[1],
                   lower_pixel[2])
                .unwrap();
        }
        writeln!(out, "{}", ANSI_BG_COLOUR_ESCAPES[0]).unwrap();
    }
}
