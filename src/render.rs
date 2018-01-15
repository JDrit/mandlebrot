extern crate crossbeam;
extern crate image;
extern crate num;
extern crate num_cpus;

use self::image::ImageBuffer;
use self::image::ImageError;
use self::image::Rgb;
use num::Complex;

use std::fs::File;
use std::str::FromStr;

pub fn escape_time(c: Complex<f64>, limit: u8) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..limit {
        z = z*z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i as u32 * 8);
        }        
    }
    return None;
}

pub fn parse_pair<T: FromStr>(s: &str, sep: char) -> Option<(T, T)> {
    let result = s.find(sep).map(|index| {
        (T::from_str(&s[..index]), T::from_str(&s[index + 1..]))
    });

    return match result {
        Some((Ok(l), Ok(r))) => Some((l, r)),
        _ => None,
    };
}

pub fn parse_complex(s: &str) -> Option<Complex<f64>> {
    return parse_pair(s, ',').map(|pair| Complex { re: pair.0, im: pair.1 });
}

pub fn pixel_to_point(bounds: (usize, usize), pixel: (usize, usize), upper_left: Complex<f64>,
                      lower_right: Complex<f64>) -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re,
                           upper_left.im - lower_right.im);
    let re = upper_left.re + pixel.0 as f64 * width / bounds.0 as f64;
    let im = upper_left.im - pixel.1 as f64 * height / bounds.1 as f64;
    return Complex { re: re, im: im };
}

pub fn render_img(file_name: &str, bounds: (usize, usize), upper_left: Complex<f64>,
                  lower_right: Complex<f64>) -> Result<(), ImageError> {
    let mut img_buf: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::new(bounds.0 as u32, bounds.1 as u32);
    let threads = num_cpus::get() * 4;
    let rows_per_band = (bounds.1 as usize) / threads + 1;
    let chunk_size: usize = rows_per_band * bounds.0 as usize;

    {
        let mut pixels: Vec<&mut Rgb<u8>> = img_buf.pixels_mut().collect();
        let bands: Vec<&mut [&mut Rgb<u8>]> = pixels.chunks_mut(chunk_size).collect();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left,
                                                     lower_right);
                let band_lower_right = pixel_to_point(bounds, (bounds.0, top + height),
                                                      upper_left, lower_right);
                spawner.spawn(move || {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }       
        });
    }

    let ref mut output = File::create(file_name)?;
    return image::ImageRgb8(img_buf).save(output, image::PNG);
}

fn to_color(color_num: u32) -> [u8 ; 3] {
    let red = (color_num & 0xFF) as u8;
    let green = ((color_num >> 8) & 0xFF) as u8;
    let blue = ((color_num >> 16) & 0xFF) as u8;
    return [0, 0, red];
}

fn render(pixels: &mut [&mut Rgb<u8>], bounds: (usize, usize), upper_left: Complex<f64>,
          lower_right: Complex<f64>) {
    
    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            let offset = row * bounds.0 + column;
            match escape_time(point, 255) {
                Some(i) => *pixels[offset] = Rgb(to_color(i)),
                None => *pixels[offset] = Rgb([0, 0, 0]),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn do_parse() {
        assert_eq!(Some((100, 200)), parse_pair::<i32>("100x200", 'x'));
    }

    #[test]
    fn do_not_parse() {
        assert_eq!(None, parse_pair::<i32>("100x200", '%'));
    }

    #[test]
    fn parse_complex_number() {
        assert_eq!(Some(Complex { re: 500f64, im: 600f64 }), parse_complex("500,600"));
    }

    #[test]
    fn fail_to_parse_complex() {
        assert_eq!(None, parse_complex("500x600"));
    }

    #[test]
    fn pixel_point() {
        let upper_left = Complex { re: -1.0, im: 1.0 };
        let lower_right = Complex { re: 1.0, im: -1.0 };
        assert_eq!(Complex { re: -0.5, im: -0.5 }, pixel_to_point((100, 100), (25, 75),
                                                                  upper_left, lower_right));
    }
}
