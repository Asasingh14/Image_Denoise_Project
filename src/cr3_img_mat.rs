use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

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