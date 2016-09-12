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


use clap::{App, Arg, AppSettings};
use std::path::PathBuf;
use std::fs;


/// Representation of the application's all configurable values.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Options {
    /// Image file to display.
    pub image: (String, PathBuf),
}

impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        let matches = App::new("termimage")
            .version(crate_version!())
            .author(crate_authors!())
            .setting(AppSettings::ColoredHelp)
            .about("Display images in your terminal, kind of")
            .arg(Arg::from_usage("<IMAGE> 'Image file to display'").validator(Options::image_file_validator))
            .get_matches();

        let image = matches.value_of("IMAGE").unwrap();
        Options { image: (image.to_string(), fs::canonicalize(image).unwrap()) }
    }

    fn image_file_validator(s: String) -> Result<(), String> {
        fs::canonicalize(&s).map(|_| ()).map_err(|_| format!("Image file \"{}\" not found", s))
    }
}
