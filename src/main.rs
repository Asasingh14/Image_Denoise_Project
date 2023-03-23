use crate::cr3_img_mat::*;
use crate::to_matrix::*;

mod cr3_img_mat;
mod to_matrix;
mod compress;

fn main() {

    println!("Hello, world!");

    let file_path = "/Users/asa/Desktop/rustprojects/Cr3_image_transform/test_file/0EC2BB31-D11C-4828-BB18-FBE507F5272A_1_105_c.jpeg";

    let pre_matrix = img_to_matrix(file_path);

    println!("Matrix: {:?}", pre_matrix);

}
