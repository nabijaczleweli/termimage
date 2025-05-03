//! This module contains the configuration of the application.
//!
//! All options are passed individually to each function and are not bundled together.
//!
//! # Examples
//!
//! ```no_run
//! # use termimage::Options;
//! let options = Options::parse();
//! println!("Image to display: {}", options.image.0);
//! ```

use std::ffi::OsString;
use std::path::PathBuf;
use std::str::FromStr;
use term_size;
use std::fs;


/// Supported ANSI output formats
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnsiOutputFormat {
    /// Truecolor ANSI 24-bit colour
    Truecolor,
    /// Dumb ANSI 3-bit colour, for black backgrounds
    SimpleBlack,
    /// Dumb ANSI 3-bit colour, for white backgrounds
    SimpleWhite,
}


/// Representation of the application's all configurable values.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Options {
    /// Image file to display.
    ///
    /// This tuple contains the plaintext name (user-friendly) and a normalised path (programmer-friendly).
    pub image: (String, PathBuf),
    /// Output size. Default: detected from terminal size or no default.
    pub size: (u32, u32),
    /// Whether to preserve the image's aspect ratio when resizing. Default: `true`.
    pub preserve_aspect: bool,
    /// Whether to output ANSI escapes and in which format. Default: `None` on Windooze when not writing to a file.
    pub ansi_out: Option<AnsiOutputFormat>,
}

impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        let mut szarg = arg!(-s --size [size] "Output image resolution").value_parser(Options::size_validator);
        let have_dimms = if let Some((w, h)) = term_size::dimensions() {
            szarg = szarg.default_value(OsString::from(format!("{}x{}", w, h - 1)));
            true
        } else {
            szarg = szarg.required(true);
            false
        };

        let matches = command!("\n")
            .arg(arg!(<IMAGE> "Image file to display").value_parser(Options::image_file_validator))
            .arg(szarg)
            .arg(arg!(-f --force "Don't preserve the image's aspect ratio"))
            .arg(arg!(-a --ansi [ANSI] "Force output ANSI escapes").value_parser(["truecolor", "simple-black", "simple-white"]))
            .get_matches();

        let image = matches.get_one::<PathBuf>("IMAGE").unwrap();
        Options {
            image: (image.to_string_lossy().to_string(), image.to_path_buf()),
            size: matches.get_one::<(u32, u32)>("size").unwrap().clone(),
            preserve_aspect: !matches.get_flag("force"),
            ansi_out: if cfg!(not(target_os = "windows")) || !have_dimms || matches.get_flag("ansi") {
                match matches.get_one::<String>("ansi").map_or("truecolor", |v| v.as_str()) {
                    "truecolor" => Some(AnsiOutputFormat::Truecolor),
                    "simple-black" => Some(AnsiOutputFormat::SimpleBlack),
                    "simple-white" => Some(AnsiOutputFormat::SimpleWhite),
                    _ => unreachable!(),
                }
            } else {
                None
            },
        }
    }

    fn parse_size(s: &str) -> Option<(u32, u32)> {
        let mut parts = s.splitn(2, |c| c == 'x' || c == 'X');
        Some((u32::from_str(parts.next()?).ok()?, u32::from_str(parts.next()?).ok()?))
    }

    fn image_file_validator(s: &str) -> Result<PathBuf, String> {
        fs::canonicalize(&s).map_err(|_| format!("Image file \"{}\" not found", s))
    }

    fn size_validator(s: &str) -> Result<(u32, u32), String> {
        match Options::parse_size(s) {
            None => Err(format!("\"{}\" is not a valid size (in format \"NNNxMMM\")", s)),
            Some((0, _)) | Some((_, 0)) => Err(format!("Can't resize image to size 0")),
            Some(other) => Ok(other),
        }
    }
}
