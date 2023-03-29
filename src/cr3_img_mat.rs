use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use rayon::prelude::*;
use rayon::iter::*;

pub fn cr3_to_matrix(file_path: &str) -> Option<Vec<Vec<[u16; 3]>>> {
    // Open the CR3 file
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => return None,
    };

    // Seek to the offset of the raw image data in the CR3 file
    let mut offset = None;
    let mut buffer = [0; 4];
    while let Ok(n) = file.read(&mut buffer) {
        if n == 0 {
            break;
        }
        if buffer == [0xFF, 0xD8, 0xFF, 0xE1] {
            // Found the start of the raw image data
            offset = Some(file.seek(SeekFrom::Current(-4)).unwrap());
            break;
        }
    }
    let offset = match offset {
        Some(offset) => offset,
        None => return None,
    };

    // Read the raw image data into a vector
    let mut raw_data = Vec::new();
    if let Err(_) = file.seek(SeekFrom::Start(offset)) {
        return None;
    }
    if let Err(_) = file.read_to_end(&mut raw_data) {
        return None;
    }

    // Parse the raw image data to extract the RGB values
    let width = 6000; // Replace with actual width of the CR3 image
    let height = 4000; // Replace with actual height of the CR3 image
    let matrix: Vec<Vec<[u16; 3]>> = (0..height).map(|y| {
        (0..width).map(|x| {
            let start = (y * width + x) * 6;
            let r = (raw_data[start + 1] as u16) << 8 | raw_data[start + 0] as u16;
            let g = (raw_data[start + 3] as u16) << 8 | raw_data[start + 2] as u16;
            let b = (raw_data[start + 5] as u16) << 8 | raw_data[start + 4] as u16;
            [r, g, b] // Store the RGB values in the result
        }).collect()
    }).collect();

    Some(matrix)
}

pub fn extract_rgb_values_in_parallel_cr3(filename: &str) -> Vec<Vec<[u16; 3]>> {
    // Load the CR3 file and find the start of the preview image
    let mut file = std::fs::File::open(filename).unwrap();
    let mut buf = vec![0; 12];
    file.read_exact(&mut buf).unwrap();
    let preview_offset = u32::from_le_bytes([buf[8], buf[9], buf[10], buf[11]]);

    // Seek to the start of the preview image
    file.seek(std::io::SeekFrom::Start(preview_offset as u64)).unwrap();

    // Parse the JPEG header to get the image dimensions
    let mut buf = vec![0; 9];
    file.read_exact(&mut buf).unwrap();
    let width = u16::from_be_bytes([buf[6], buf[7]]) as usize;
    let height = u16::from_be_bytes([buf[4], buf[5]]) as usize;

    // Allocate a buffer for the image pixels
    let mut pixels = vec![0; width * height * 3];

    // Read the JPEG image data into the buffer
    let mut cursor = std::io::Cursor::new(&mut pixels[..]);
    std::io::copy(&mut file, &mut cursor).unwrap();

    // Extract the RGB values of each pixel in parallel using Rayon
    let pixels: Vec<[u16; 3]> = pixels
        .par_chunks(3)
        .map(|chunk| {
            let r = chunk[0] as u16;
            let g = chunk[1] as u16;
            let b = chunk[2] as u16;
            [r, g, b]
        })
        .collect();

    // Convert the flattened list of pixels into a 2D vector
    pixels.chunks(width).map(|row| row.to_vec()).collect()
}