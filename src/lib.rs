//! This crate provides [`symbolize`] function that allows you to convert raster images to their symbolic versions.
//! It supports scaling the [`symbolize`]d images as well as coloring them for terminals with RGB-support.
//!
//! # Example usage:
//!
//! ```
//! use image::{imageops::FilterType};
//! use std::{process, error::Error};
//! use symbolize::{symbolize, Colorize};
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let result = symbolize(
//!         "./image.png",
//!         0.1,
//!         // Function will use these symbols for the main average colors.
//!         // If the image has the most white pixels, the first character from the vector will be used for them, and so on.
//!         &vec!['*', '#', '@', ' '],
//!         FilterType::Nearest,
//!         Colorize::Bw,
//!     );
//!
//!     match result {
//!         Err(e) => {
//!             eprintln!("{}", e);
//!             process::exit(1);
//!         }
//!         Ok(result) => {
//!             for line in Into::<Vec<String>>::into(result) {
//!                 println!("{}", line)
//!             }
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! [`SymbolizeResult`] is a wrapper that allows you to easy convert a result to [`Vec<String>`], [`Vec<u8>`] or [`String`]
//!
//! # Example output:
//!
//! ```
//! @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
//! @@@@@@@@@@@@@@@@@@@@@@@@^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^@@@@@@@@@@@@@@@@@@@@@@@@
//! @@@@@@@@@@@@@@@@@@@@@@^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^@@@@@@@@@@@@@@@@@@@@@@
//! @@@@@@@@@@@@@@@@@@^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^@@@@@@@@@@@@@@@@@@
//! @@@@@@@@@@@@@@@@^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^@@@@@@@@@@@@@@@@
//! @@@@@@@@@@@@^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^&&&&^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^@@@@@@@@@@@@
//! @@@@@@@@@@^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^&&&&^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^@@@@@@@@@@
//! @@@@@@@@^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^&&&&&&^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^@@@@@@@@
//! @@@@@@@@^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^&&&&&&&&^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^@@@@@@@@
//! @@@@@@^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^&&&&&&&&^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^@@@@@@
//! @@@@^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^&&&&&&&&&&&&^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^@@@@
//! @@^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^&&&&&&&&&&&&^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^@@
//! @@^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^&&&&&&&&&&&&&&&&^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^@@
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^&&&&&&&&&&&&&&&&^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^&&&&&&&&&&&&&&&&^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^&&&&^^&&&&&&&&&&&&&&^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^&&&&&&&&&&&&&&&&&&&&&&&&^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^&&&&&&&&&&&&&&&&&&&&&&&&^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^&&&&&&&&&&&&&&&&&&&&&&&&&&&&^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^&&&&&&&&&&&&&&&&&&&&&&&&&&&&^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^&&&&&&&&&&&&&&&&^^^^^^^^&&&&&&&&&&&&&&&&^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^&&&&&&&&&&&&&&^^^^^^^^^^^^&&&&&&&&&&&&&&^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^&&&&&&&&&&&&&&&&^^^^^^^^^^^^&&&&&&&&&&&&&&&&^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^&&&&&&&&&&&&&&^^^^^^^^^^^^^^^^&&&&&&&&&&&&&&^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^&&&&&&&&&&&&&&&&^^^^^^^^^^^^^^^^&&&&&&&&&&&&&&&&^^^^^^^^^^^^^^^^^^^^^^^^^^
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^&&&&&&&&&&&&&&&&^^^^^^^^^^^^^^^^&&&&&&&&&&&&&&^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//! ^^^^^^^^^^^^^^^^^^^^^^^^&&&&&&&&&&&&&&&&&&^^^^^^^^^^^^^^^^&&&&&&&&&&&&&&&&^^^^^^^^^^^^^^^^^^^^^^^^^^
//! ^^^^^^^^^^^^^^^^^^^^^^^^&&&&&&&&&&&&&&&&&&^^^^^^^^^^^^^^^^&&&&&&&&&&&&&&&&&&^^^^^^^^^^^^^^^^^^^^^^^^
//! ^^^^^^^^^^^^^^^^^^^^^^&&&&&&&&&&&&&&&&^^^^^^^^^^^^^^^^^^^^^^^^&&&&&&&&&&&&&&&&^^^^^^^^^^^^^^^^^^^^^^
//! ^^^^^^^^^^^^^^^^^^^^&&&&&&&&&&&&^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^&&&&&&&&&&&&^^^^^^^^^^^^^^^^^^^^
//! ^^^^^^^^^^^^^^^^^^^^&&&&&&&&^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^&&&&&&&&^^^^^^^^^^^^^^^^^^^^
//! ^^^^^^^^^^^^^^^^^^&&&&&&^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^&&&&&&^^^^^^^^^^^^^^^^^^
//! ^^^^^^^^^^^^^^^^^^&&^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^&&^^^^^^^^^^^^^^^^^^
//! @@^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^@@
//! @@^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^@@
//! @@@@^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^@@@@
//! @@@@@@^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^@@@@@@
//! @@@@@@@@^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^@@@@@@@@
//! @@@@@@@@^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^@@@@@@@@
//! @@@@@@@@@@^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^@@@@@@@@@@
//! @@@@@@@@@@@@^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^@@@@@@@@@@@@
//! @@@@@@@@@@@@@@@@^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^@@@@@@@@@@@@@@@@
//! @@@@@@@@@@@@@@@@@@^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^@@@@@@@@@@@@@@@@@@
//! @@@@@@@@@@@@@@@@@@@@@@^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^@@@@@@@@@@@@@@@@@@@@@@
//! @@@@@@@@@@@@@@@@@@@@@@@@^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^@@@@@@@@@@@@@@@@@@@@@@@@
//! @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
//! ```

use std::{
    collections::{hash_map::Entry, HashMap},
    error::Error,
    io,
};

use crossterm::style::{style, Color, Stylize};
use image::{
    imageops::{resize, FilterType},
    open, ImageBuffer, Rgb,
};

/// Helper wrapper struct that provides some [`Into`] implementations for easier convertation
pub struct SymbolizeResult(Vec<String>);

impl Into<String> for SymbolizeResult {
    fn into(self) -> String {
        self.0.join("\n")
    }
}

impl Into<Vec<u8>> for SymbolizeResult {
    fn into(self) -> Vec<u8> {
        self.0.join("\n").into_bytes()
    }
}

impl Into<Vec<String>> for SymbolizeResult {
    fn into(self) -> Vec<String> {
        self.0
    }
}

/// Determines whether the image should use colors for the rgb terminal.
#[derive(Debug)]
pub enum Colorize {
    Bw,
    Rgb,
}

impl Into<Colorize> for bool {
    fn into(self) -> Colorize {
        if self == true {
            Colorize::Rgb
        } else {
            Colorize::Bw
        }
    }
}

/// Main function of this crate. Turns your raster image into it's symbolic representation.
pub fn symbolize(
    path: &str,
    scale: f32,
    symbols: &[char],
    filter_type: FilterType,
    colorize: Colorize,
) -> Result<SymbolizeResult, Box<dyn Error>> {
    let mut image = open(path)?.into_rgb8();
    image = resize(
        &image,
        (image.width() as f32 * scale) as u32,
        (image.height() as f32 * scale) as u32,
        filter_type,
    );
    let colors_to_use = get_most_used_colours_with_symbols(&image, symbols);

    let mut result = vec![];
    for row in image.rows() {
        let mut result_row = vec![];
        for pixel in row {
            let (symbol, average_pixel) = get_symbol_by_pixel(&colors_to_use, pixel)?;
            let str_symbol = match colorize {
                Colorize::Bw => symbol.to_string(),
                Colorize::Rgb => {
                    format!(
                        "{}",
                        style(symbol.to_string()).with(Color::from((
                            average_pixel.0[0],
                            average_pixel.0[1],
                            average_pixel.0[2]
                        )))
                    )
                }
            };
            result_row.push(str_symbol.clone());
            result_row.push(str_symbol);
        }

        result.push(result_row.join(""))
    }

    Ok(SymbolizeResult(result))
}

#[derive(Debug)]
struct PixelWithSymbol {
    pixel: Rgb<u8>,
    symbol: char,
}

impl PixelWithSymbol {
    fn new(pixel: Rgb<u8>, symbol: char) -> Self {
        Self { pixel, symbol }
    }
}

fn get_most_used_colours_with_symbols(
    image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    symbols: &[char],
) -> Vec<PixelWithSymbol> {
    let mut colors_uses: HashMap<&image::Rgb<u8>, usize> = HashMap::new();
    for pixel in image.pixels() {
        match colors_uses.entry(pixel) {
            Entry::Vacant(entry) => {
                entry.insert(1);
            }
            Entry::Occupied(mut entry) => {
                *entry.get_mut() += 1;
            }
        }
    }
    let mut colours_uses_vec: Vec<(&Rgb<u8>, usize)> = colors_uses.into_iter().collect();
    colours_uses_vec.sort_by_key(|(_, count)| *count);

    let (start, end) = (
        colours_uses_vec.len() - symbols.len(),
        colours_uses_vec.len(),
    );

    let mut symbol_idx = symbols.len() - 1;
    colours_uses_vec
        .drain(start..end)
        .map(|(pixel, _)| {
            let pixel_with_symbol = PixelWithSymbol::new(*pixel, symbols[symbol_idx]);
            symbol_idx = symbol_idx.checked_sub(1).unwrap_or(0);

            pixel_with_symbol
        })
        .collect()
}

fn get_symbol_by_pixel(
    pixels_with_info: &[PixelWithSymbol],
    pixel_to_compare: &Rgb<u8>,
) -> Result<(char, Rgb<u8>), io::Error> {
    let mut char = None;
    let mut rgb_pixel = None;
    let mut comparison = None;

    for PixelWithSymbol { pixel, symbol } in pixels_with_info {
        let pretendent_comparison = get_pixel_comparison(pixel_to_compare, pixel);
        if comparison.is_none() || pretendent_comparison < comparison.unwrap() {
            char = Some(*symbol);
            comparison = Some(pretendent_comparison);
            rgb_pixel = Some(*pixel);
        }
    }

    if let (Some(char), Some(rgb_pixel)) = (char, rgb_pixel) {
        return Ok((char, rgb_pixel));
    }

    Err(io::Error::new(
        io::ErrorKind::Other,
        "unexpected error, can't find matching char for a pixel. aborting",
    ))
}

fn get_pixel_comparison(first: &Rgb<u8>, second: &Rgb<u8>) -> usize {
    ((first.0[0] as i16 - second.0[0] as i16).abs()
        + (first.0[1] as i16 - second.0[1] as i16).abs()
        + (first.0[2] as i16 - second.0[2] as i16).abs()) as usize
}
