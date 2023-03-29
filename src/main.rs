use crate::cr3_img_mat::*;
use crate::to_matrix::*;
use crate::png::*;

mod cr3_img_mat;
mod to_matrix;
mod compress;
mod to_jpeg;
mod png;

fn main() {

    println!("Hello, world!");

    // let cr3_file_path = "/Users/asa/Desktop/rustprojects/Cr3_image_transform/test_file/EOSR5hSLI000050NR0.CR3";
    //
    // let cr3_mat = extract_rgb_values_in_parallel_cr3(cr3_file_path);
    //
    // println!("Matrix: {:?}", cr3_mat);

    let file_path = "/Users/asa/Desktop/rustprojects/Cr3_image_transform/test_file/0EC2BB31-D11C-4828-BB18-FBE507F5272A_1_105_c.jpeg";

    let pre_matrix = img_to_matrix_color(file_path);

    println!("Matrix: {:?}", pre_matrix);

    to_png(&pre_matrix, "/Users/asa/Desktop/rustprojects/Cr3_image_transform/res_dump/image.jpg");

    //save_compressed_image_to_file(&compressed_image, "/Users/asa/Desktop/rustprojects/Cr3_image_transform/res_dump/image.jpg").unwrap();

}