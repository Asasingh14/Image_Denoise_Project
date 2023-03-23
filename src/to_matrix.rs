use image::{ImageBuffer, Rgb};
use rayon::prelude::*;

pub fn img_to_matrix(file_path: &str) -> Vec<Vec<[u16; 3]>> {
    // Load the Cr3 image
    let img = image::open(file_path).unwrap();

    // Convert the image to an RGB image buffer
    let rgb_img = img.into_rgb16();

    // Get the dimensions of the image
    let (width, height) = rgb_img.dimensions();

    // Convert the RGB image buffer to a matrix in parallel
    let matrix: Vec<Vec<[u16; 3]>> = (0..height).into_par_iter().map(|y| {
        (0..width).into_par_iter().map(|x| {
            let pixel = rgb_img.get_pixel(x, y);
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];
            [r, g, b] // Store the RGB values in the result
        }).collect()
    }).collect();

    matrix
}