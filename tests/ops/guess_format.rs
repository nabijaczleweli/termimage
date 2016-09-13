extern crate image;
extern crate termimage;

use std::path::PathBuf;
use self::image::ImageFormat;
use self::termimage::Outcome;
use self::termimage::ops::guess_format;


#[test]
fn png() {
    test_correct("png", ImageFormat::PNG);
}

#[test]
fn jpeg() {
    test_correct("jpg", ImageFormat::JPEG);
    test_correct("jpeg", ImageFormat::JPEG);
    test_correct("jpe", ImageFormat::JPEG);
    test_correct("jif", ImageFormat::JPEG);
    test_correct("jfif", ImageFormat::JPEG);
    test_correct("jfi", ImageFormat::JPEG);
}

#[test]
fn gif() {
    test_correct("gif", ImageFormat::GIF);
}

#[test]
fn webp() {
    test_correct("webp", ImageFormat::WEBP);
}

#[test]
fn ppm() {
    test_correct("ppm", ImageFormat::PPM);
}

#[test]
fn tiff() {
    test_correct("tiff", ImageFormat::TIFF);
    test_correct("tif", ImageFormat::TIFF);
}

#[test]
fn tga() {
    test_correct("tga", ImageFormat::TGA);
}

#[test]
fn bmp() {
    test_correct("bmp", ImageFormat::BMP);
    test_correct("dib", ImageFormat::BMP);
}

#[test]
fn ico() {
    test_correct("ico", ImageFormat::ICO);
}

#[test]
fn hdr() {
    test_correct("hdr", ImageFormat::HDR);
}

#[test]
fn unknown() {
    let p = "../tests/ops/guess_format.rs".to_string();
    assert_eq!(guess_format(&(p.clone(), PathBuf::from(&p))), Err(Outcome::GuessingFormatFailed(p)));
}


fn test_correct(ext: &str, f: ImageFormat) {
    assert_eq!(guess_format(&(String::new(), PathBuf::from(format!("img.{}", ext)))), Ok(f));
}
