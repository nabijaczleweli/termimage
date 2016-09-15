//! Display images in your terminal, kind of
//!
//! ![DS3 SS example](https://cdn.rawgit.com/nabijaczleweli/termimage/master/assets/DS3-result.jpg)
//!
//! # Library doc
//!
//! This library is used by `termimage` itself for all its function and is therefore contains all necessary functions.
//!
//! ## Data flow
//!
//! ```plaintext
//! Options::parse()
//! |> guess_format()
//! |> load_image()
//! |> resize_image()
//! |> write_[no_]ansi()
//! ```
//!
//! # Executable manpage
//!
//! Exit values and possible errors:
//!
//! ```text
//! 1 - failed to guess the file's format
//! 2 - failed to open the image file
//! ```
//!
//! ## SYNOPSIS
//!
//! `termimage` [OPTIONS] &lt;IMAGE&gt;
//!
//! ## DESCRIPTION
//!
//! Show images in your terminal.
//!
//! The images are automatically downscaled to the terminal's size and their
//! colours are approximated to match the terminal's display colours.
//!
//! With ANSI output this means a 3-bit colour resolution, with WinAPI - 4-bit.
//!
//! With WinAPI output the output colours are acquired from the console itself,
//! with ANSI output a sane default is assumed.
//!
//! ## OPTIONS
//!
//! &lt;IMAGE&gt;
//!
//! ```text
//! Image to display, must end in a recognisable image format extension.
//! ```
//!
//! -s --size &lt;size&gt;
//!
//! ```text
//! Output image resolution.
//!
//! By default this is autodetected to match the output terminal's resolution,
//! but is required when outputting to a file.
//!
//! Format: NxM
//! ```
//!
//! -a --ansi &lt;ANSI_type&gt;
//!
//! ```text
//! Force ANSI output of the specified kind,
//!
//! The accepted values are "simple" and "truecolor", truecolor is the default
//! on non-Windows.
//!
//! Simple ANSI output uses 3-bit background colours, while truecolor supports
//! the whole 24-bit pallette.
//! ```
//!
//! -f --force
//!
//! ```text
//! By default the image's aspect ratio will be preserved when downscaling,
//! use this option to override that behaviour.
//! ```
//!
//! ## EXAMPLES
//!
//! `termimage` [`-s` *NxM*] [`-f`] *assets/image.png*
//!
//! ```text
//! Display assets/image.png in the terminal using the default output type,
//! optionally not preserving the aspect ratio.
//! ```
//!
//! `termimage` [`-s` *NxM*] [`-f`] [`-a` *simple*] *assets/image.png*
//!
//! ```text
//! Display assets/image.png in the terminal using the simple ANSI output type,
//! optionally not preserving the aspect ratio.
//! ```

#[macro_use]
extern crate lazy_static;
extern crate term_size;
#[cfg(target_os = "windows")]
extern crate kernel32;
#[cfg(target_os = "windows")]
extern crate winapi;
extern crate image;
extern crate regex;
#[macro_use]
extern crate clap;

mod options;
mod outcome;

pub mod ops;
pub mod util;

pub use options::Options;
pub use outcome::Outcome;
