//! Module containing various utility functions.


use std::iter;
use image::Rgb;
use std::ops::Index;


/// "Standard" ANSI colours, in the same order as `ANSI_COLOUR_ESCAPES`.
///
/// Acquired from screenshot provided by [@Ell](https://github.com/elliotpotts):
///
/// ![Terminal screenshot](https://cloud.githubusercontent.com/assets/6709544/18532811/e7e87a6e-7ade-11e6-868f-f6d2f9faec27.png)
///
/// Might not be representative due to white bg though...
pub static ANSI_COLOURS: &'static [Rgb<u8>] = &[Rgb { data: [0xee, 0xe8, 0xd5] },
                                                Rgb { data: [0xdc, 0x32, 0x2f] },
                                                Rgb { data: [0x85, 0x99, 0x00] },
                                                Rgb { data: [0xb5, 0x89, 0x00] },
                                                Rgb { data: [0x26, 0x8b, 0xd2] },
                                                Rgb { data: [0xd3, 0x36, 0x82] },
                                                Rgb { data: [0x2a, 0xa1, 0x98] },
                                                Rgb { data: [0x07, 0x36, 0x42] }];

/// ANSI background colour escapes.
pub static ANSI_BG_COLOUR_ESCAPES: &'static [&'static str] = &["\x1B[40m", "\x1B[41m", "\x1B[42m", "\x1B[43m", "\x1B[44m", "\x1B[45m", "\x1B[46m", "\x1B[47m"];


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
