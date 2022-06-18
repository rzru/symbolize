use clap::Parser;
use image::{imageops::FilterType, ImageError};
use std::{
    io::{Error, ErrorKind},
    process,
};
use symbolize::symbolize;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(name = "symbolize!")]
#[clap(author = "rzru <rzzzzru@gmail.com>")]
#[clap(
    about = "Converts raster images into their symbolic view",
    long_about = None
)]
struct Args {
    /// Path to the original picture
    #[clap(value_parser)]
    path: String,

    /// Defines scale of symbolized picture relatively to the original
    #[clap(long, value_parser, default_value_t = 1.0)]
    scale: f32,

    /// Defines symbols that will be used to fill the picture (in priority order)
    #[clap(short, long, value_parser, default_value_t = String::from("*@#&"))]
    symbols: String,

    /// Filter type. One of: nearest, triangle, catmull_rom, gaussian, lanczos3.
    /// More about differences: https://docs.rs/image/latest/image/imageops/enum.FilterType.html
    #[clap(short, long, value_parser, default_value_t = String::from("triangle"))]
    filter: String,

    /// Flag that shows should output be colorized for a terminal or not.
    /// Not recommended to use it with anything but terminals with rgb support
    #[clap(short, long, action, default_value_t = false)]
    colorize: bool,
}

fn main() -> Result<(), ImageError> {
    let args = Args::parse();
    let filter_type_wrapper: FilterTypeWrapper = args.filter.try_into()?;
    let symbols: Vec<char> = args.symbols.chars().collect();

    let result = symbolize(
        &args.path,
        args.scale,
        &symbols,
        filter_type_wrapper.0,
        args.colorize.into(),
    );

    match result {
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
        Ok(result) => {
            for line in Into::<Vec<String>>::into(result) {
                println!("{}", line)
            }
        }
    }

    Ok(())
}

struct FilterTypeWrapper(FilterType);

impl TryInto<FilterTypeWrapper> for String {
    type Error = Error;

    fn try_into(self) -> Result<FilterTypeWrapper, Self::Error> {
        let self_str = self.as_str();

        if self_str == "nearest" {
            return Ok(FilterTypeWrapper(FilterType::Nearest));
        }

        if self_str == "triangle" {
            return Ok(FilterTypeWrapper(FilterType::Triangle));
        }

        if self_str == "catmull_rom" {
            return Ok(FilterTypeWrapper(FilterType::CatmullRom));
        }

        if self_str == "gaussian" {
            return Ok(FilterTypeWrapper(FilterType::Gaussian));
        }

        if self_str == "lanczos3" {
            return Ok(FilterTypeWrapper(FilterType::Lanczos3));
        }

        Err(Error::new(
            ErrorKind::InvalidData,
            "unknown filter type, aborting",
        ))
    }
}
