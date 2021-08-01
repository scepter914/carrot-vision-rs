extern crate image;

use image::RgbImage;

// RGB
pub fn print_image_info(image: RgbImage) -> () {
    println!("dimensions {:?}", image.dimensions());
    //println!("{:?}", self.image.color());
}

pub fn print_pixel(pixel: &image::Rgb<u8>) -> () {
    println!("{}, {}, {}", pixel[0], pixel[1], pixel[2]);
}
