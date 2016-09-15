//! Main functions doing actual work.
//!
//! Use `guess_format()` to get teh image format from a path,
//! then read the image using `load_image()`,
//! resize it to terminal size with `resize_image()`
//! and display it with `write_[no_]ansi()`.

use self::super::util::{ANSI_BG_COLOUR_ESCAPES, ANSI_COLOURS, JPEG_MAGIC, BMP_MAGIC, ICO_MAGIC, GIF_MAGIC, PNG_MAGIC, closest_colour};
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

/// Resize the specified image to the specified size, optionally preserving its aspect ratio.
pub fn resize_image(img: &DynamicImage, size: (u32, u32), preserve_aspect: bool) -> DynamicImage {
    if preserve_aspect {
        img.resize(size.0, size.1, FilterType::Nearest)
    } else {
        img.resize_exact(size.0, size.1, FilterType::Nearest)
    }
}

/// Display the specified image in the default console using ANSI escape codes.
pub fn write_ansi<W: Write>(out: &mut W, img: &DynamicImage) {
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let closest_clr = closest_colour(img.get_pixel(x, y).to_rgb(), ANSI_COLOURS);
            write!(out, "{} ", ANSI_BG_COLOUR_ESCAPES[closest_clr]).unwrap();
        }
        writeln!(out, "{}", ANSI_BG_COLOUR_ESCAPES[0]).unwrap();
    }
}
