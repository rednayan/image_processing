use image_processing::Image;
use std::fs;

fn main() {
    let image_bytes = fs::read("example2.jpg").unwrap();
    let image = Image::new(image_bytes.clone());
    println!("{:?}", &image_bytes[0..100]);
    println!("{:?}", image.application_data());
    // let mut i = 0;
    // while i < 5 {
    //     println!("{:?}", &image_bytes[i]);
    //     i = i + 1;
    // }
    // println!("{:?}", image.application_data());
}
