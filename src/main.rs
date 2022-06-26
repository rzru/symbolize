use clap::Parser;
use image::{imageops::FilterType, open, ImageError};
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
    about = "converts bitmap images into text art",
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
    #[clap(short, long, value_parser)]
    palette: String,

    /// Filter type. One of: nearest, triangle, catmull_rom, gaussian, lanczos3.
    /// More about differences: https://docs.rs/image/latest/image/imageops/enum.FilterType.html
    #[clap(short, long, value_parser, default_value = "nearest")]
    filter: String,

    /// Flag that shows should output be colorized for a terminal or not.
    /// Not recommended to use it with anything but terminals with rgb support
    #[clap(short, long, action, default_value_t = false)]
    colorize: bool,
}

fn main() -> Result<(), ImageError> {
    let args = Args::parse();
    let filter_type_wrapper: FilterTypeWrapper = args.filter.try_into()?;
    let palette: Vec<char> = args.palette.chars().collect();

    let result = symbolize(
        open(args.path)?,
        args.scale,
        &palette,
        filter_type_wrapper.0,
        args.colorize,
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
        match self.as_str() {
            "nearest" => Ok(FilterTypeWrapper(FilterType::Nearest)),
            "triangle" => Ok(FilterTypeWrapper(FilterType::Triangle)),
            "catmull_rom" => Ok(FilterTypeWrapper(FilterType::CatmullRom)),
            "gaussian" => Ok(FilterTypeWrapper(FilterType::Gaussian)),
            "lanczos3" => Ok(FilterTypeWrapper(FilterType::Lanczos3)),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "unknown filter type, aborting",
            )),
        }
    }
}
