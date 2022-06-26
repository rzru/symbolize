//! This crate provides [`symbolize`] function that allows you to convert bitmap images into fine text art.
//! It supports scaling the [`symbolize`]d images as well as coloring them for terminals with RGB-support.
//!
//! [`SymbolizeResult`] is a wrapper that allows you to easy convert a result to [`Vec<String>`], [`Vec<u8>`] or [`String`]
//!
//! The "original_image" parameter provides an original image as a [`DynamicImage`]
//!
//! The "palette" parameter determines which characters will be used when converting the image.
//! The symbols are arranged in descending order of the frequency of their appearance on the image.
//!
//! The "scale" parameter determines the size of the output image relative to the size of the original.
//!
//! The "filter_type" parameter defines what type of filtering will be used when scaling the image. For more info read [`FilterType`] docs.
//!
//! The "colorize" parameter determines whether the output should be colorized for RGB-terminals or not.
//!
//! # Example usage:
//!
//! ```ignore
//! use image::{imageops::FilterType, open};
//! use std::{process, error::Error};
//! use symbolize::symbolize;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let result = symbolize(
//!         open("./path/to/image.png")?
//!         &vec!['*', '#', '@', ' '],
//!         0.1,
//!         FilterType::Nearest,
//!         false,
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
//! # Example output:
//!
//! ```ignore
//!                               @@  @@@@  @@                              
//!                           @@  @@@@@@@@@@@@@@@@@@                        
//!                     @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@                    
//!                     @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@            @@      
//!   @@  @@          @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@      @@@@      
//!   @@  @@@@        @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@      @@@@  @@@@
//! @@@@  @@@@    @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  @@@@@@@@  
//!   @@@@@@@@    @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@    @@@@@@@@  
//!     @@@@@@  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@    
//!       @@@@  @@@@@@@@@@@@@@@@@@&&    @@@@&&&&  @@@@@@@@@@@@@@  @@        
//!         @@@@@@@@@@@@@@@@@@@@@@&&    @@@@      @@@@@@@@@@@@@@@@@@        
//!           @@@@@@@@@@@@@@@@@@@@      @@@@      @@@@@@@@@@@@@@@@@@        
//!         @@@@@@##@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@####@@@@@@      
//!           @@@@##  ####@@@@@@@@@@@@@@    @@@@@@@@@@####    ##@@@@        
//!             @@  ##        ######################        ##  @@          
//!               @@                                            @@          
//!                                                           @@
//! ```

use std::{
    collections::{hash_map::Entry, HashMap},
    error::Error,
    io,
};

use crossterm::style::{style, Color, Stylize};
use image::{
    imageops::{resize, FilterType},
    DynamicImage, Rgb, RgbImage,
};

/// Helper wrapper struct that provides some [`Into`] implementations for easier convertation
pub struct SymbolizeResult(Vec<Vec<String>>);

impl Into<String> for SymbolizeResult {
    fn into(self) -> String {
        self.0
            .iter()
            .map(|row| row.join(""))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Into<Vec<u8>> for SymbolizeResult {
    fn into(self) -> Vec<u8> {
        self.0
            .iter()
            .map(|row| row.join(""))
            .collect::<Vec<String>>()
            .join("\n")
            .into_bytes()
    }
}

impl Into<Vec<String>> for SymbolizeResult {
    fn into(self) -> Vec<String> {
        self.0
            .iter()
            .map(|row| row.join(""))
            .collect::<Vec<String>>()
    }
}

/// Main function of this crate. Turns your bitmap image into text art.
pub fn symbolize(
    original_image: DynamicImage,
    scale: f32,
    palette: &[char],
    filter_type: FilterType,
    colorize: bool,
) -> Result<SymbolizeResult, Box<dyn Error>> {
    if palette.is_empty() {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "pallete should contain at leasst one symbol, aborting",
        )));
    }

    if scale < 0.0 {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "scale should be > 0, aborting",
        )));
    }

    let original_image_rgb = original_image.into_rgb8();
    let scaled_image = resize(
        &original_image_rgb,
        (original_image_rgb.width() as f32 * scale) as u32,
        (original_image_rgb.height() as f32 * scale) as u32,
        filter_type,
    );
    let colors_to_use = get_most_used_colours_with_symbols(&scaled_image, palette);

    let mut result = vec![];
    for row in scaled_image.rows() {
        let mut result_row = vec![];
        for pixel in row {
            let (symbol, average_pixel) = get_symbol_by_pixel(&colors_to_use, pixel)?;

            let str_symbol = if colorize {
                format!(
                    "{}",
                    style(symbol.to_string()).with(Color::from((
                        average_pixel.0[0],
                        average_pixel.0[1],
                        average_pixel.0[2]
                    )))
                )
            } else {
                symbol.to_string()
            };
            result_row.push(str_symbol.clone());
            result_row.push(str_symbol);
        }

        result.push(result_row)
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

fn get_most_used_colours_with_symbols(image: &RgbImage, symbols: &[char]) -> Vec<PixelWithSymbol> {
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
        colours_uses_vec
            .len()
            .checked_sub(symbols.len())
            .unwrap_or(0),
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
    pixels_with_symbols: &[PixelWithSymbol],
    pixel_to_compare: &Rgb<u8>,
) -> Result<(char, Rgb<u8>), io::Error> {
    let mut char = None;
    let mut rgb_pixel = None;
    let mut comparison = None;

    for PixelWithSymbol { pixel, symbol } in pixels_with_symbols {
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

#[cfg(test)]
mod tests {
    use image::{imageops::FilterType, open};

    use crate::symbolize;

    fn get_ferris() -> Vec<&'static str> {
        vec![
            "                                                                        ",
            "                                                                        ",
            "                                                                        ",
            "                              @@  @@@@  @@                              ",
            "                          @@  @@@@@@@@@@@@@@@@@@                        ",
            "                    @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@                    ",
            "                    @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@            @@      ",
            "  @@  @@          @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@      @@@@      ",
            "  @@  @@@@        @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@      @@@@  @@@@",
            "@@@@  @@@@    @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  @@@@@@@@  ",
            "  @@@@@@@@    @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@    @@@@@@@@  ",
            "    @@@@@@  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@    ",
            "      @@@@  @@@@@@@@@@@@@@@@@@&&    @@@@&&&&  @@@@@@@@@@@@@@  @@        ",
            "        @@@@@@@@@@@@@@@@@@@@@@&&    @@@@      @@@@@@@@@@@@@@@@@@        ",
            "          @@@@@@@@@@@@@@@@@@@@      @@@@      @@@@@@@@@@@@@@@@@@        ",
            "        @@@@@@$$@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@$$$$@@@@@@      ",
            "          @@@@$$  $$$$@@@@@@@@@@@@@@    @@@@@@@@@@$$$$    $$@@@@        ",
            "            @@  $$        $$$$$$$$$$$$$$$$$$$$$$        $$  @@          ",
            "              @@                                            @@          ",
            "                                                          @@            ",
            "                                                                        ",
            "                                                                        ",
            "                                                                        ",
            "                                                                        ",
        ]
    }

    fn get_colorized_ferris() -> &'static str {
        "\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\n\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\n\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\n\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\n\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\n\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;165;43;0m&\u{1b}[39m\u{1b}[38;2;165;43;0m&\u{1b}[39m\u{1b}[38;2;165;43;0m&\u{1b}[39m\u{1b}[38;2;165;43;0m&\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;165;43;0m&\u{1b}[39m\u{1b}[38;2;165;43;0m&\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;247;76;0m$\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\n\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\n\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m\u{1b}[38;2;0;0;0m@\u{1b}[39m"
    }

    #[test]
    fn renders_ferris_as_vec_of_strings() {
        let image = open("./test-data/ferris.png").unwrap();
        let result: Vec<String> = symbolize(
            image,
            0.03,
            &vec![' ', '@', '$', '&'],
            FilterType::Nearest,
            false,
        )
        .unwrap()
        .into();

        assert_eq!(result, get_ferris());
    }

    #[test]
    fn renders_colorized_ferris_as_string() {
        let image = open("./test-data/ferris.png").unwrap();
        let result: String = symbolize(
            image,
            0.01,
            &vec![' ', '@', '$', '&'],
            FilterType::Nearest,
            true,
        )
        .unwrap()
        .into();

        assert_eq!(result, get_colorized_ferris());
    }

    #[test]
    fn renders_ferris_as_string() {
        let image = open("./test-data/ferris.png").unwrap();
        let result: String = symbolize(
            image,
            0.03,
            &vec![' ', '@', '$', '&'],
            FilterType::Nearest,
            false,
        )
        .unwrap()
        .into();

        assert_eq!(result, get_ferris().join("\n"));
    }

    #[test]
    fn renders_ferris_as_byteslice() {
        let image = open("./test-data/ferris.png").unwrap();
        let result: Vec<u8> = symbolize(
            image,
            0.03,
            &vec![' ', '@', '$', '&'],
            FilterType::Nearest,
            false,
        )
        .unwrap()
        .into();

        assert_eq!(result, get_ferris().join("\n").into_bytes());
    }

    #[test]
    fn returns_error_if_no_palette_passed() {
        let image = open("./test-data/ferris.png").unwrap();
        let result = symbolize(image, 0.03, &vec![], FilterType::Nearest, false);

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "pallete should contain at leasst one symbol, aborting"
        );
    }

    #[test]
    fn returns_error_if_scale_less_than_zero() {
        let image = open("./test-data/ferris.png").unwrap();
        let result = symbolize(image, -0.03, &vec![' '], FilterType::Nearest, false);

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "scale should be > 0, aborting"
        );
    }
}
