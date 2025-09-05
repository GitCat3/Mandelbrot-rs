use image::{GrayImage, Luma};
use num_complex::Complex;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

fn cartesiantocomplex(x: u32, y: u32, max_x: u32, max_y: u32) -> Complex<f64> {
    let x = x as f64;
    let y = y as f64;
    let max_x = max_x as f64;
    let max_y = max_y as f64;
    
    // Map x from [0, max_x] to [-2, 0.5]
    let a = (x / max_x) * 2.5 - 2.0;
    // Map y from [0, max_y] to [-1.25, 1.25]
    let b = (y / max_y) * 2.5 - 1.25;

    Complex::new(a, b)
}

fn mandelbrot(c: Complex<f64>, max_iter: i32) -> bool{
    let mut z = Complex::new(0.0, 0.0);
    let bound = Complex::new(2.0, 0.0);
    for _ in 1..max_iter {
        z = z.powi(2) + c;
        if z.norm() >= bound.norm() {
            return false;
        }
    }
    return true;
}

fn main() {
    let max_x = 500;
    let max_y = 500;
    let max_iter = 100;
    let image = Arc::new(Mutex::new(GrayImage::new(max_x, max_y)));

    for i in 0..max_y-1 {
        let image = Arc::clone(&image);
        (0..max_x-1).into_par_iter().for_each(move |j| {
            let c = cartesiantocomplex(j, i, max_x, max_y);
            let color = if mandelbrot(c, max_iter) {
                Luma([0u8])
            } else {
                Luma([255u8])
            };
            image.lock().unwrap().put_pixel(j, i, color);
        });
    }
    image.lock().unwrap().save("mandelbrot.png").unwrap();
}
