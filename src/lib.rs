#[macro_use]
extern crate lazy_static;
extern crate term_size;
extern crate image;
extern crate regex;
#[macro_use]
extern crate clap;

mod options;
mod outcome;

pub mod ops;

pub use options::Options;
pub use outcome::Outcome;
