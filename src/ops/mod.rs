//! Main functions doing actual work.
//!
//! Use `guess_format()` to get teh image format from a path,
//! then read the image using `load_image()`,
//! resize it to terminal size with `resize_image()`
//! and display it with `write_[no_]ansi()`.

use image::{self, GenericImage, DynamicImage, ImageFormat, FilterType, Pixel};
use self::super::util::{ANSI_BG_COLOUR_ESCAPES, ANSI_COLOURS, closest_colour};
use std::io::{BufReader, Write};
use self::super::Outcome;
use std::path::PathBuf;
use std::fs::File;

mod no_ansi;

pub use self::no_ansi::write_no_ansi;


/// Guess the image format from its extension.
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
/// assert_eq!(guess_format(&("../ops.rs".to_string(), PathBuf::from("ops.rs"))),
/// Err(Outcome::GuessingFormatFailed("../ops.rs".to_string())));
/// ```
pub fn guess_format(file: &(String, PathBuf)) -> Result<ImageFormat, Outcome> {
    file.1
        .extension()
        .and_then(|ext| match &ext.to_str().unwrap().to_lowercase()[..] {
            "png" => Some(ImageFormat::PNG),
            "jpg" | "jpeg" | "jpe" | "jif" | "jfif" | "jfi" => Some(ImageFormat::JPEG),
            "gif" => Some(ImageFormat::GIF),
            "webp" => Some(ImageFormat::WEBP),
            "ppm" => Some(ImageFormat::PPM),
            "tiff" | "tif" => Some(ImageFormat::TIFF),
            "tga" => Some(ImageFormat::TGA),
            "bmp" | "dib" => Some(ImageFormat::BMP),
            "ico" => Some(ImageFormat::ICO),
            "hdr" => Some(ImageFormat::HDR),
            _ => None,
        })
        .ok_or_else(|| Outcome::GuessingFormatFailed(file.0.clone()))
}

/// Load an image from the specified file as the specified format.
///
/// Get the image fromat with `guess_format()`.
pub fn load_image(file: &(String, PathBuf), format: ImageFormat) -> DynamicImage {
    image::load(BufReader::new(File::open(&file.1).unwrap()), format).unwrap()
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
