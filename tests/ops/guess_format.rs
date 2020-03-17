extern crate image;
extern crate termimage;

use std::path::PathBuf;
use self::termimage::Error;
use self::image::ImageFormat;
use self::termimage::ops::guess_format;


#[test]
fn png() {
    test_correct("png", ImageFormat::Png);
}

#[test]
fn jpeg() {
    test_correct("jpg", ImageFormat::Jpeg);
    test_correct("jpeg", ImageFormat::Jpeg);
    test_correct("jpe", ImageFormat::Jpeg);
    test_correct("jif", ImageFormat::Jpeg);
    test_correct("jfif", ImageFormat::Jpeg);
    test_correct("jfi", ImageFormat::Jpeg);
}

#[test]
fn gif() {
    test_correct("gif", ImageFormat::Gif);
}

#[test]
fn webp() {
    test_correct("webp", ImageFormat::WebP);
}

#[test]
fn ppm() {
    test_correct("ppm", ImageFormat::Pnm);
}

#[test]
fn tiff() {
    test_correct("tiff", ImageFormat::Tiff);
    test_correct("tif", ImageFormat::Tiff);
}

#[test]
fn tga() {
    test_correct("tga", ImageFormat::Tga);
}

#[test]
fn bmp() {
    test_correct("bmp", ImageFormat::Bmp);
    test_correct("dib", ImageFormat::Bmp);
}

#[test]
fn ico() {
    test_correct("ico", ImageFormat::Ico);
}

#[test]
fn hdr() {
    test_correct("hdr", ImageFormat::Hdr);
}

#[test]
fn unknown() {
    let p = "tests/ops/guess_format.rs".to_string();
    assert_eq!(guess_format(&(p.clone(), PathBuf::from(&p))), Err(Error::GuessingFormatFailed(p)));
}


fn test_correct(ext: &str, f: ImageFormat) {
    assert_eq!(guess_format(&(String::new(), PathBuf::from(format!("img.{}", ext)))), Ok(f));
}
