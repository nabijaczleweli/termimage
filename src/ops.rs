use image::{self, DynamicImage, ImageFormat, FilterType};
use self::super::Outcome;
use std::path::PathBuf;
use std::io::BufReader;
use std::fs::File;


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

pub fn load_image(file: &(String, PathBuf), format: ImageFormat) -> DynamicImage {
    image::load(BufReader::new(File::open(&file.1).unwrap()), format).unwrap()
}

pub fn resize_image(img: &DynamicImage, size: (u32, u32), preserve_aspect: bool) -> DynamicImage {
    if preserve_aspect {
        img.resize(size.0, size.1, FilterType::Nearest)
    } else {
        img.resize_exact(size.0, size.1, FilterType::Nearest)
    }
}
