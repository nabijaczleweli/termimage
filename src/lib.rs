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
