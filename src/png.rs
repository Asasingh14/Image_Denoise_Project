use image::{ImageBuffer, ColorType, Rgb};
use rayon::prelude::*;
use crossbeam::scope;

pub fn to_png(pixels: &Vec<Vec<[u16; 3]>>, filename: &str) {
    let height = pixels.len();
    let width = pixels[0].len();

    let mut image_buffer = ImageBuffer::new(width as u32, height as u32);

    pixels.par_iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, pixel)| {
            let rgb = Rgb([pixel[0] as u8, pixel[1] as u8, pixel[2] as u8]);
            image_buffer.put_pixel(x as u32, y as u32, rgb);
        });
    });

    let _ = image_buffer.save(filename);
}