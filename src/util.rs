//! Module containing various utility functions.


use std::iter;
use image::Rgb;
use std::ops::Index;


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
