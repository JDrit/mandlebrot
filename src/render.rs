extern crate num;

use num::Complex;
use std::str::FromStr;

pub fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..limit {
        z = z*z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
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

pub fn render(pixels: &mut [u8], bounds: (usize, usize), upper_left: Complex<f64>,
              lower_right: Complex<f64>) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
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
