//! Display images in your terminal, kind of
//!
//! ![DS3 SS example WinAPI](https://cdn.rawgit.com/nabijaczleweli/termimage/master/assets/DS3-winapi.jpg)
//! ![DS3 SS example truecolor](https://cdn.rawgit.com/nabijaczleweli/termimage/master/assets/DS3-truecolor.png)
//! ![rust logo example](https://cdn.rawgit.com/nabijaczleweli/termimage/master/assets/rust-logo-truecolor.png)
//! ![playing dice example](https://cdn.rawgit.com/nabijaczleweli/termimage/master/assets/playing-dice-truecolor.png)
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
//! |> image_resized_size()
//! |> resize_image()
//! |> write_[no_]ansi[_truecolor]()
//! ```
//!
//! ### Prose explanation
//!
//! First, get an `Options` instance, be it via a struct-literal or `Options::parse()`;
//! or don't and just create the individual arguments manually.
//!
//! Then, use `ops::load_image()`. If you know your image's format, great. If you don't, get it via `ops::guess_format()`.
//!
//! After that resize the image to an output-ready size provided by `ops::image_resized_size()` with `resize_image()`.
//! `ops::image_resized_size()` takes into consideration using two pixels per cell in the output functions,
//! so the size it returns is twice as tall as the terminal output size passed to it.
//!
//! Finally, call `ops::write_ansi()`/`ops::write_ansi_truecolor()`/`ops::write_no_ansi()` depending on your liking with the
//! resulting image.
//!
//! Or, if you want to display images manually, use `ops::create_colourtable()` to create an approximate colours table and
//! display it, for example, with `ncurses`.
//!
//! ### Example
//!
//! This is a complete example, from parsing the commandline to displaying the result.
//!
//! ```no_run
//! # extern crate termimage;
//! # extern crate image;
//! # use image::GenericImage;
//! # use std::io::stdout;
//! # use termimage::*;
//! # fn main() {
//! #   not_main();
//! # }
//! # fn not_main() -> Result<(), Error> {
//! let opts = Options::parse();
//!
//! let format = try!(ops::guess_format(&opts.image));
//! let img = try!(ops::load_image(&opts.image, format));
//!
//! let img_s = ops::image_resized_size(img.dimensions(), opts.size, opts.preserve_aspect);
//! let resized = ops::resize_image(&img, img_s);
//!
//! match opts.ansi_out {
//!     Some(true) => ops::write_ansi_truecolor(&mut stdout(), &resized),
//!     Some(false) => ops::write_ansi(&mut stdout(), &resized),
//!     None => ops::write_no_ansi(&resized),
//! }
//! # Ok(())
//! # }
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
//!
//! (for f in $(find *image_dir* -type f); do `termimage -s` *NxM* [`-f`] [`-a` *ANSI_type*] $f; done) > *out_file*
//!
//! ```text
//! Print all images in image_dir to out_file.
//!
//! Note the --size option being specified, since it's required when outputting to a file.
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

mod error;
mod options;

pub mod ops;
pub mod util;

pub use error::Error;
pub use options::Options;
