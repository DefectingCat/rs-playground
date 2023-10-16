use num::Complex;
use rayon::prelude::*;

fn mandelbrot_pixel(x: u32, y: u32, width: u32) -> u8 {
    let scale = 3.0 / width as f64;
    let x = x as f64 * scale - 2.0;
    let y = y as f64 * scale - 1.5;
    let c = Complex::new(x, y);
    let mut z = Complex::new(0.0, 0.0);
    let max_iterations = 255;

    for i in 0..max_iterations {
        if z.norm() > 2.0 {
            return i as u8;
        }
        z = z * z + c
    }

    max_iterations as u8
}

fn main() {
    let width = 800;
    let height = 800;

    let mut buffer = image::ImageBuffer::new(width, height);
    buffer
        .enumerate_pixels_mut()
        .par_bridge()
        .into_par_iter()
        .for_each(|(x, y, pixel)| {
            let r = (0.3 * x as f32) as u8;
            let g = (0.3 * y as f32) as u8;
            let color_b = mandelbrot_pixel(x, y, width);
            *pixel = image::Rgb([r, g, color_b]);
        });

    buffer
        .save("mandelbrot.png")
        .expect("Failed to create output file");
}
