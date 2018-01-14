extern crate image;

use self::image::ColorType;
use self::image::png::PNGEncoder;
use std::fs::File;
use std::io::Error;

pub fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::RGB(8))?;
    return Ok(());
}

