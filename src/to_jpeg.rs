use std::fs::File;
use std::io::prelude::*;

fn mat_to_jpeg(compressed_image: &[u8], file_path: &str) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(compressed_image)?;
    Ok(())
}