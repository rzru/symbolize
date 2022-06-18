use image::imageops::FilterType;
use std::{error::Error, process};
use symbolize::{symbolize, Colorize};
fn main() -> Result<(), Box<dyn Error>> {
    let result = symbolize(
        "./image.png",
        0.1,
        // Function will use these symbols for the main average colors.
        // If the image has the most white pixels, the first character from the vector will be used for them, and so on.
        &vec!['*', '#', '@', ' '],
        // Filter type that will be used for scaling.
        FilterType::Nearest,
        Colorize::Bw,
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
