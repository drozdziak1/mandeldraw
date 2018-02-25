extern crate docopt;
extern crate image;
extern crate num_complex;

// Stdlib imports
use std::collections::BTreeSet;
use std::fs::File;

// External imports
use docopt::Docopt;
use num_complex::Complex;

const MAX_LUMA: u8 = 255;

/// A helper struct for handling coordinates
struct Coord<T> {
    pub x: T,
    pub y: T,
}

const USAGE: &'static str = "
Usage: mandeldraw [options] [<output_filename>]

Options:
    -h, --help                  Display this help message
    -z ZOOM, --zoom ZOOM        How much we should zoom (float)
    --centerx X                 Where to center the render (horizontal, float)
    --centery Y                 Where to center the render (vertical, float)
    --lenx X                    How wide should the render span around the center (float)
    --leny Y                    How tall should the render span around the center (float)
    -r RAD, --radius RAD        How big a norm should we treat as infinity (float)
    -n COUNT, --max-iter COUNT  Iteration count before a point belongs in the set (float)
    -s PIXELS, --size PIXELS    How many pixels wide shouold the image take (int)
    -x, --crosshairs            Draw crosshairs at (X, Y)
";


fn main() {

    let args = Docopt::new(USAGE)
        .and_then(|d| d.parse())
        .unwrap_or_else(|e| e.exit());

    // CLI argument parsing and defaults
    let zoom: f64 = args.get_str("--zoom").parse().unwrap_or(1f64);
    if zoom <= 0f64 {
        panic!("Invalid zoom value!");
    }

    let center = Coord {
        x: args.get_str("--centerx").parse::<f64>().unwrap_or(0.746999f64),
        y: args.get_str("--centery").parse::<f64>().unwrap_or(0.249991f64),
    };
    let len = Coord {
        x: args.get_str("--lenx").parse::<f64>().unwrap_or(2f64) / zoom,
        y: args.get_str("--leny").parse::<f64>().unwrap_or(2f64) / zoom,
    };
    if len.x <= 0f64 || len.y <= 0f64 {
        panic!("Invalid viewport dimensions")
    }
    let radius: f64 = args.get_str("--radius").parse().unwrap_or(2f64);
    let img_size: u32 = args.get_str("--size").parse().unwrap_or(400u32);
    let max_iterations: u16 = args.get_str("--max-iter").parse().unwrap_or(40u16);
    let output_filename: &str = match args.get_str("<output_filename>") {
        "" => "fractal.png",
        s => s,
    };
    let draw_crosshairs: bool = args.get_bool("--crosshairs");

    println!("Zoom: {}x", zoom);
    println!("Center: {}, {}", center.x, center.y);
    println!("Viewport dimensions: {}x{}", len.x, len.y);
    println!("Escape radius: {}", radius);
    println!("Image size: {}x{} pixels", img_size, img_size);
    println!("Max iterations: {}", max_iterations);
    println!("Output file: {}", output_filename);
    println!("Crosshairs: {}", draw_crosshairs);

    let cmin = Coord {
        x: -len.x + center.x / 2f64,
        y: -len.y + center.y / 2f64
    };
    let cmax = Coord {
        x: len.x + center.x / 2f64,
        y: len.y + center.y / 2f64
    };

    let scale = Coord {
        x: (cmax.x - cmin.x) / img_size as f64,
        y: (cmax.y - cmin.y) / img_size as f64
    };

    let mut imgbuf = image::ImageBuffer::new(img_size, img_size);

    // A set for normalization of pixels after calculation
    let mut lumaset = BTreeSet::new();

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let cx = cmin.x + x as f64 * scale.x;
        let cy = cmin.y + y as f64 * scale.y;

        let c = Complex::new(cx, cy);
        let mut z = Complex::new(0f64, 0f64);

        let mut i = 0f64;

        for t in 0..max_iterations {
            if z.norm() > radius {
                break;
            }
            z = z * z + c;
            i = t as f64;
        }
        i = i / max_iterations as f64 * MAX_LUMA as f64;

        *pixel = image::Rgb([MAX_LUMA - i as u8; 3]);

        lumaset.insert(MAX_LUMA - i as u8);
    }

    let min = lumaset.iter().min().unwrap();
    let max = lumaset.iter().max().unwrap();

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut normalized = (pixel.data[0] - *min) as f64 / (*max - *min) as f64 * MAX_LUMA as f64;
        pixel.data = if draw_crosshairs && (2 * x == img_size || 2 * y == img_size) {
            [0, normalized as u8, MAX_LUMA - normalized as u8]
        } else {
            [normalized as u8; 3]
        }
    }


    let fout = &mut File::create("fractal.png").unwrap();
    image::ImageRgb8(imgbuf).save(fout, image::PNG).unwrap();
}
