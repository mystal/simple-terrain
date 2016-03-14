extern crate image;
extern crate rand;

use std::f64;

use image::{
    ImageBuffer,
};

type NoiseField = Box<Fn(f64, f64) -> f64>;
type NoiseGrid = Vec<Vec<f64>>;
type ColorMap = Fn(f64) -> i32;

fn sine_field(alpha: f64, offset: f64) -> NoiseField {
    Box::new(move |x, y| {
        let p = alpha.cos() * x + alpha.sin() * y;
        (p + offset).sin()
    })
}

fn scale(field: NoiseField, factor: f64) -> NoiseField {
    Box::new(move |x, y| {
        field(x / factor, y / factor) * factor
    })
}


fn noise_field(n: i32, fall: f64) -> NoiseField {
    (0..n).fold(Box::new(|x, y| 0.0), |noise, i| {
        let alpha = rand::random::<f64>() * f64::consts::PI * 2.0;
        let offset = rand::random::<f64>() * f64::consts::PI * 2.0;
        Box::new(move |x, y| noise(x, y) + scale(sine_field(alpha, offset), fall.powi(i))(x, y))
    })
}

fn grid(noise: NoiseField, width: u32, height: u32) -> NoiseGrid {
    let mut samples = vec![vec![0.0; width as usize]; height as usize];

    // Sample noise at different points to generate a grid of values.
    for (y, row) in samples.iter_mut().enumerate() {
        for (x, v) in row.iter_mut().enumerate() {
            *v = noise(x as f64 / width as f64 - 0.5, y as f64 / height as f64 - 0.5);
        }
    }

    // Find min/max values for the grid.
    let min = samples.iter()
        .map(|row| row.iter().cloned().fold(f64::NAN, f64::min))
        .fold(f64::NAN, f64::min);
    let max = samples.iter()
        .map(|row| row.iter().cloned().fold(f64::NAN, f64::max))
        .fold(f64::NAN, f64::max);

    println!("Min: {}, Max: {}", min, max);

    // Use the min/max to map all values to [0,1].
    for row in samples.iter_mut() {
        for v in row.iter_mut() {
            *v = ((*v - min) / (max - min)).max(0.0).min(1.0);
        }
    }

    samples
}

fn grayscale(v: f64) -> image::Luma<u8> {
    let channel = (v * 256.0).floor().max(0.0).min(255.0) as u8;
    image::Luma([channel])
}

fn terracolor(v: f64) -> image::Rgb<u8> {
    image::Rgb(
        if v < 0.50 {
            [0x20, 0x20, 0xFF] // ocean
        } else if v < 0.55 {
            [0x40, 0x40, 0xFF] // shallow water
        } else if v < 0.6 {
            [0x40, 0xA0, 0x40] // plains
        } else if v < 0.8 {
            [0x30, 0x80, 0x30] // forest
        } else if v < 0.85 {
            [0x80, 0x80, 0x80] // mountain
        } else if v < 0.9 {
            [0x60, 0x60, 0x60] // tall mountain
        } else {
            [0xFF, 0xFF, 0xFF] // snow
        }
    )
}

// TODO: Apply ColorMap on each NoiseGrid value to get pixel color
fn render(noise_grid: NoiseGrid, img_path: &str) {
    let height = noise_grid.len() as u32;
    let width = noise_grid[0].len() as u32;
    let img = ImageBuffer::from_fn(width, height, |x, y| {
        //image::Rgba([0u8, 0, 0, 255])
        //grayscale(noise_grid[y as usize][x as usize])
        terracolor(noise_grid[y as usize][x as usize])
    });

    // Write the contents of this image to the Writer in PNG format.
    let _ = img.save(img_path).unwrap();
}

fn main() {
    let field = scale(sine_field(0.0, 0.0), 1.0 / (f64::consts::PI * 4.0));
    let noise_grid = grid(field, 200, 200);
    render(noise_grid, "test.png");

    let field = scale(noise_field(200, 0.98), 0.3);
    let noise_grid = grid(field, 200, 200);
    render(noise_grid, "terraina.png");

    let field = scale(noise_field(200, 0.98), 0.3);
    let noise_grid = grid(field, 200, 200);
    render(noise_grid, "terrainb.png");

    let field = scale(noise_field(200, 0.98), 0.3);
    let noise_grid = grid(field, 200, 200);
    render(noise_grid, "terrainc.png");
}
