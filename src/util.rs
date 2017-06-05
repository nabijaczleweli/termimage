//! Module containing various utility functions.


use std::iter;
use image::Rgb;
use std::ops::Index;


/// Magic number used for determining whether an image is a BMP.
///
/// Source: [Wikipedia](https://en.wikipedia.org/wiki/List_of_file_signatures).
pub static BMP_MAGIC: &'static [u8] = &[0x42, 0x4D];

/// Magic number used for determining whether an image is an ICO.
///
/// Source: [Wikipedia](https://en.wikipedia.org/wiki/List_of_file_signatures).
pub static ICO_MAGIC: &'static [u8] = &[0x00, 0x00, 0x01, 0x00];

/// Magic number used for determining whether an image is a GIF.
///
/// Source: [Wikipedia](https://en.wikipedia.org/wiki/List_of_file_signatures).
pub static GIF_MAGIC: &'static [u8] = &[0x47, 0x49, 0x46, 0x38];

/// Magic number used for determining whether an image is a PNG.
///
/// Source: [Wikipedia](https://en.wikipedia.org/wiki/List_of_file_signatures).
pub static PNG_MAGIC: &'static [u8] = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

/// Magic number used for determining whether an image is a JPEG.
///
/// Source: [Wikipedia](https://en.wikipedia.org/wiki/List_of_file_signatures).
pub static JPEG_MAGIC: &'static [u8] = &[0xFF, 0xD8, 0xFF, 0xE0];

/// "Standard" ANSI background colours, in the same order as `ANSI_BG_COLOUR_ESCAPES`.
///
/// Acquired from screenshot provided by [@Ell](https://github.com/elliotpotts):
///
/// ![Terminal screenshot](https://cloud.githubusercontent.com/assets/6709544/18532811/e7e87a6e-7ade-11e6-868f-f6d2f9faec27.png)
///
/// Might not be representative due to white bg though...
pub static ANSI_BG_COLOURS: &'static [Rgb<u8>] = &[Rgb { data: [0xEE, 0xE8, 0xD5] },
                                                   Rgb { data: [0xDC, 0x32, 0x2F] },
                                                   Rgb { data: [0x85, 0x99, 0x00] },
                                                   Rgb { data: [0xB5, 0x89, 0x00] },
                                                   Rgb { data: [0x26, 0x8B, 0xD2] },
                                                   Rgb { data: [0xD3, 0x36, 0x82] },
                                                   Rgb { data: [0x2A, 0xA1, 0x98] },
                                                   Rgb { data: [0x07, 0x36, 0x42] }];

/// "Standard" ANSI colours, in the same order as `ANSI_COLOUR_ESCAPES`.
///
/// Acquired from screenshot provided by [@Ell](https://github.com/elliotpotts):
///
/// ![Terminal screenshot](https://cloud.githubusercontent.com/assets/6709544/18532811/e7e87a6e-7ade-11e6-868f-f6d2f9faec27.png)
///
/// Might not be representative due to white bg though...
pub static ANSI_COLOURS: &'static [Rgb<u8>] = &[Rgb { data: [0xEE, 0xE8, 0xD5] },
                                                Rgb { data: [0xDC, 0x32, 0x2F] },
                                                Rgb { data: [0x85, 0x99, 0x00] },
                                                Rgb { data: [0xB5, 0x89, 0x00] },
                                                Rgb { data: [0x26, 0x8B, 0xD2] },
                                                Rgb { data: [0xD3, 0x36, 0x82] },
                                                Rgb { data: [0x2A, 0xA1, 0x98] },
                                                Rgb { data: [0x07, 0x36, 0x42] },
                                                Rgb { data: [0xFD, 0xF6, 0xE3] },
                                                Rgb { data: [0xCB, 0x4B, 0x16] },
                                                Rgb { data: [0x93, 0xA1, 0xA1] },
                                                Rgb { data: [0x83, 0x94, 0x96] },
                                                Rgb { data: [0x65, 0x7B, 0x83] },
                                                Rgb { data: [0x6C, 0x71, 0xC4] },
                                                Rgb { data: [0x58, 0x6E, 0x75] },
                                                Rgb { data: [0x00, 0x2B, 0x36] }];

/// ANSI background colour escapes.
pub static ANSI_COLOUR_ESCAPES: &'static [&'static str] = &["\x1B[0;30m",
                                                            "\x1B[0;31m",
                                                            "\x1B[0;32m",
                                                            "\x1B[0;33m",
                                                            "\x1B[0;34m",
                                                            "\x1B[0;35m",
                                                            "\x1B[0;36m",
                                                            "\x1B[0;37m",
                                                            "\x1B[1;30m",
                                                            "\x1B[1;31m",
                                                            "\x1B[1;32m",
                                                            "\x1B[1;33m",
                                                            "\x1B[1;34m",
                                                            "\x1B[1;35m",
                                                            "\x1B[1;36m",
                                                            "\x1B[1;37m"];

/// ANSI background colour escapes.
pub static ANSI_BG_COLOUR_ESCAPES: &'static [&'static str] = &["\x1B[40m", "\x1B[41m", "\x1B[42m", "\x1B[43m", "\x1B[44m", "\x1B[45m", "\x1B[46m", "\x1B[47m"];

/// Reset ANSI attributes
pub static ANSI_RESET_ATTRIBUTES: &str = "\x1B[0m";


/// Create a string consisting of `n` repetitions of `what`.
///
/// # Examples
///
/// ```
/// # use termimage::util::mul_str;
/// assert_eq!(mul_str("Го! ", 3), "Го! Го! Го! ".to_string());
/// ```
pub fn mul_str(what: &str, n: usize) -> String {
    iter::repeat(what).take(n).collect()
}

/// Get the closest colour to the provided one out of the specified list of colours and retirn its index.
///
/// The formula was taken from [this](http://stackoverflow.com/a/1847112/2851815) SO answer
/// and might not be the best, but we only ever have a VERY limited colourset, so it shouldn't really matter that much.
pub fn closest_colour<P: Index<usize, Output = u8>>(to: Rgb<u8>, out_of: &[P]) -> usize {
    let mut diffs = out_of.iter()
        .enumerate()
        .map(|(i, rgb)| {
            (((rgb[0] as f32 - to[0] as f32) * 0.30).powi(2) + ((rgb[1] as f32 - to[1] as f32) * 0.59).powi(2) +
             ((rgb[1] as f32 - to[2] as f32) * 0.11).powi(2),
             i)
        })
        .collect::<Vec<_>>();
    diffs.sort_by(|&(lhs_diff, _), &(rhs_diff, _)| lhs_diff.partial_cmp(&rhs_diff).unwrap());
    diffs[0].1
}
