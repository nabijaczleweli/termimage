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


use clap::{Arg, AppSettings};
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
        let szarg_def;
        let mut szarg = Arg::from_usage("-s --size [size] 'Output image resolution'").validator(Options::size_validator);
        let have_dimms = if let Some((w, h)) = term_size::dimensions() {
            szarg_def = format!("{}x{}", w, h - 1);
            szarg = szarg.default_value(&szarg_def);
            true
        } else {
            szarg = szarg.required(true);
            false
        };

        let matches = app_from_crate!("\n")
            .setting(AppSettings::ColoredHelp)
            .arg(Arg::from_usage("<IMAGE> 'Image file to display'").validator(Options::image_file_validator))
            .arg(szarg)
            .arg(Arg::from_usage("-f --force 'Don't preserve the image's aspect ratio'"))
            .arg(Arg::from_usage("-a --ansi [ANSI] 'Force output ANSI escapes'").possible_values(&["truecolor", "simple-black", "simple-white"]))
            .get_matches();

        let image = matches.value_of("IMAGE").unwrap();
        Options {
            image: (image.to_string(), fs::canonicalize(image).unwrap()),
            size: Options::parse_size(matches.value_of("size").unwrap()).unwrap(),
            preserve_aspect: !matches.is_present("force"),
            ansi_out: if cfg!(not(target_os = "windows")) || !have_dimms || matches.is_present("ansi") {
                match matches.value_of("ansi").unwrap_or("truecolor") {
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

    fn image_file_validator(s: String) -> Result<(), String> {
        fs::canonicalize(&s).map(|_| ()).map_err(|_| format!("Image file \"{}\" not found", s))
    }

    fn size_validator(s: String) -> Result<(), String> {
        match Options::parse_size(&s) {
            None => Err(format!("\"{}\" is not a valid size (in format \"NNNxMMM\")", s)),
            Some((0, _)) | Some((_, 0)) => Err(format!("Can't resize image to size 0")),
            Some(_) => Ok(()),
        }
    }
}
