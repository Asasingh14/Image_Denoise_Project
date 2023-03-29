use image::{ImageBuffer, Rgb};
use rayon::prelude::*;

pub fn img_to_matrix_color(file_path: &str) -> Vec<Vec<[u16; 3]>> {
    // Load the image
    let img = image::open(file_path).unwrap();

    let rgb_img = img.into_rgb16();

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

pub fn img_to_matrix_greyscale(file_path: &str) -> Vec<Vec<u16>> {
    // Load the image
    let img = image::open(file_path).unwrap();

    let grey_img = img.into_luma16();

    let (width, height) = grey_img.dimensions();

    // Convert the greyscale image buffer to a matrix in parallel
    let matrix: Vec<Vec<u16>> = (0..height).into_par_iter().map(|y| {
        (0..width).into_par_iter().map(|x| {
            let pixel = grey_img.get_pixel(x, y);
            let luma = pixel[0] as f32 / u16::MAX as f32; // Normalize the luminance to [0, 1]
            (luma * u16::MAX as f32) as u16 // Scale the luminance back to [0, u16::MAX]
        }).collect()
    }).collect();

    matrix
}