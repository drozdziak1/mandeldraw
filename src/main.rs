/**
 * This program is based on this example from Rosetta Code:
 * https://rosettacode.org/wiki/Mandelbrot_set#Rust
 */
extern crate image;
extern crate num_complex;
extern crate rayon;

use std::collections::BTreeSet;
use std::fs::File;

use num_complex::Complex;
use rayon::prelude::*;

const MAX_LUMA: u8 = 255;
const BOUNDARY: f32 = 100f32;

fn main() {
    let max_iterations = 40u16;
    let img_side = 200u32;
    let cxmin = -2.2f32;
    let cxmax = 1.0f32;
    let cymin = -1.6f32;
    let cymax = 1.6f32;
    let scalex = (cxmax - cxmin) / img_side as f32;
    let scaley = (cymax - cymin) / img_side as f32;

    let mut imgbuf = image::ImageBuffer::new(img_side, img_side);

    let mut lumaset = BTreeSet::new();

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let cx = cxmin + x as f32 * scalex;
        let cy = cymin + y as f32 * scaley;

        let c = Complex::new(cx, cy);
        let mut z = Complex::new(0f32, 0f32);

        let mut i = 0f32;

        for t in 0..max_iterations {
            if z.norm() > BOUNDARY {
                break;
            }
            z = z * z + c;
            i = t as f32 / max_iterations as f32 * MAX_LUMA as f32;
        }
        *pixel = image::Luma([MAX_LUMA - i as u8]);

        lumaset.insert(MAX_LUMA - i as u8);
    }

    let min = lumaset.iter().min().unwrap();
    let max = lumaset.iter().max().unwrap();

    for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut normalized = (pixel.data[0] - min) as f32 / *max as f32 * MAX_LUMA as f32;
        pixel.data = [normalized as u8];
    }

    let fout = &mut File::create("fractal.png").unwrap();
    image::ImageLuma8(imgbuf).save(fout, image::PNG).unwrap();
}
