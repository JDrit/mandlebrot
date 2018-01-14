extern crate crossbeam;
extern crate num;
extern crate num_cpus;

use num::Complex;
use render;

pub fn render(pixels: &mut [u8], bounds: (usize, usize), upper_left: Complex<f64>,
              lower_right: Complex<f64>) {
    let threads = num_cpus::get() * 4;
    let rows_per_band = bounds.1 / threads + 1;
    let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();

    crossbeam::scope(|spawner| {
        for (i, band) in bands.into_iter().enumerate() {
            let top = rows_per_band * i;
            let height = band.len() / bounds.0;
            let band_bounds = (bounds.0, height);
            let band_upper_left = render::pixel_to_point(bounds, (0, top), upper_left,
                                                         lower_right);
            let band_lower_right = render::pixel_to_point(bounds, (bounds.0, top + height),
                                                           upper_left, lower_right);
            println!("spawning work item: {}", i);            
            spawner.spawn(move || {
                render::render(band, band_bounds, band_upper_left, band_lower_right);
            });
        }
    });
}
