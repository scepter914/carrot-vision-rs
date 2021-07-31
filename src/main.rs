extern crate image;
// extern crate imageproc;

use image::{DynamicImage, GenericImageView, GrayImage, RgbImage};
use imageproc::contrast::stretch_contrast;

use riprs::object_detection::rgb_threshold;
use riprs::riprs_cv_lib::file;

fn main() {
    let input_image_path = "data/ball_1.jpg";
    // let path_image = "data/ball_2.jpg";
    let output_path = riprs::riprs_cv_lib::file::FilePath::new("data/result/", "result.png");
    let input_image = image::open(input_image_path).unwrap().to_rgb8();

    let rgb_threshold = rgb_threshold::RGBThreshold {
        r_high: 255,
        r_low: 80,
        g_high: 255,
        g_low: 0,
        b_high: 255,
        b_low: 0,
    };
    let binarized_image = rgb_threshold::binalize_rgb(&input_image, &rgb_threshold);
    binarized_image.save(output_path.get_file_path()).unwrap();
}

fn print_image_info(image: RgbImage) -> () {
    println!("dimensions {:?}", image.dimensions());
    //println!("{:?}", self.image.color());
}
