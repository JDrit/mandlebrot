extern crate num;

mod output;
mod render;
mod threading;

use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        writeln!(std::io::stderr(), "Usage: mandlelbrot FILE PIXELS UPPERLEFT LOWERRIGHT")
            .unwrap();
        writeln!(std::io::stderr(), "Example: {} mandel.png 1000x750 -1.20,0.35 -1.0,0.20", args[0])
            .unwrap();
        std::process::exit(1);
    } else {
        let bounds = render::parse_pair(&args[2], 'x')
            .expect("error parsing image dimensions");
        let upper_left = render::parse_complex(&args[3])
            .expect("error parsing upper left corner point");
        let lower_right = render::parse_complex(&args[4])
            .expect("error parsing lower right corner point");
        let mut pixels = vec![0 ; bounds.0 * bounds.1];

        //render::render(&mut pixels, bounds, upper_left, lower_right);
        threading::render(&mut pixels, bounds, upper_left, lower_right);

        output::write_image(&args[1], &pixels, bounds)
            .expect("error writing PNG file");
    }
}


