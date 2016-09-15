use std::io::Write;


/// Enum representing all possible values the application can fail.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Outcome {
    /// No errors occured, everything executed correctly.
    NoError,
    /// Failed to guess the image format.
    GuessingFormatFailed(String),
    /// Failed to open image file.
    OpeningImageFailed(String),
}

impl Outcome {
    /// Get the executable exit value from an `Outcome` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use termimage::Outcome;
    /// # use std::iter::FromIterator;
    /// let mut out = Vec::new();
    /// Outcome::GuessingFormatFailed("not_image.rs".to_string()).print_error(&mut out);
    /// assert_eq!(String::from_iter(out.iter().map(|&i| i as char)),
    ///            "Failed to guess format of \"not_image.rs\".\n".to_string());
    /// ```
    pub fn print_error<W: Write>(&self, err_out: &mut W) {
        match *self {
            Outcome::NoError => (),
            Outcome::GuessingFormatFailed(ref fname) => writeln!(err_out, "Failed to guess format of \"{}\".", fname).unwrap(),
            Outcome::OpeningImageFailed(ref fname) => writeln!(err_out, "Failed to open image file \"{}\".", fname).unwrap(),
        }
    }

    /// Get the executable exit value from an `Outcome` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::process::exit;
    /// # use termimage::Outcome;
    /// exit(Outcome::NoError.exit_value());
    /// ```
    pub fn exit_value(&self) -> i32 {
        match *self {
            Outcome::NoError => 0,
            Outcome::GuessingFormatFailed(_) => 1,
            Outcome::OpeningImageFailed(_) => 2,
        }
    }
}
