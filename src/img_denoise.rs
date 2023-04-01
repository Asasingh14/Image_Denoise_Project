use image::{GenericImageView, ImageBuffer, Rgb};
use rayon::prelude::*;
use std::path::Path;

pub fn denoise_image(input_path: &str, output_path: &str) {
    let image = image::open(&Path::new(input_path)).unwrap();
    let (width, height) = image.dimensions();

    let denoised_image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(width, height, |x, y| {
        let neighbors = get_neighbors(&image, x as i32, y as i32);
        let avg_color = average_color(&neighbors);
        Rgb([avg_color[0], avg_color[1], avg_color[2]])
    });

    denoised_image.save(output_path).unwrap();
}

fn get_neighbors(image: &image::DynamicImage, x: i32, y: i32) -> Vec<[u8; 3]> {
    let width = image.width() as i32;
    let height = image.height() as i32;

    let dx = [-1, 0, 1, -1, 1, -1, 0, 1];
    let dy = [-1, -1, -1, 0, 0, 1, 1, 1];

    let mut neighbors = Vec::new();
    for i in 0..8 {
        let nx = x + dx[i];
        let ny = y + dy[i];

        if nx >= 0 && ny >= 0 && nx < width && ny < height {
            let pixel = image.get_pixel(nx as u32, ny as u32);
            neighbors.push([pixel[0], pixel[1], pixel[2]]);
        }
    }

    neighbors
}

fn average_color(neighbors: &Vec<[u8; 3]>) -> [u8; 3] {
    let count = neighbors.len() as u32;
    let mut r_sum = 0;
    let mut g_sum = 0;
    let mut b_sum = 0;

    for color in neighbors {
        r_sum += color[0] as u32;
        g_sum += color[1] as u32;
        b_sum += color[2] as u32;
    }

    [
        (r_sum / count) as u8,
        (g_sum / count) as u8,
        (b_sum / count) as u8,
    ]
}
