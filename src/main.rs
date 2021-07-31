extern crate image;
// extern crate imageproc;

use image::{DynamicImage, GenericImageView, GrayImage, RgbImage};

use riprs::object_detection::rgb_threshold;
use riprs::riprs_cv_lib::file;

fn main() {
    let input_image_path = "data/ball_1.jpg";
    // let path_image = "data/ball_2.jpg";

    let output_path = file::FilePath::new("data/result/", "result.png");

    let input_image = image::open(input_image_path).unwrap().to_rgb8();

    let rgb_threshold = rgb_threshold::RGBThreshold {
        r_high: 255,
        r_low: 0,
        g_high: 255,
        g_low: 0,
        b_high: 40,
        b_low: 0,
    };
    let binarized_image = rgb_threshold::binalize_rgb(&input_image, &rgb_threshold);
    binarized_image.save(output_path.get_file_path()).unwrap();
    let (r_image, g_image, b_image) = rgb_threshold::disassemble_rgb(&input_image);

    let output_path = file::FilePath::new("data/result/", "result_r.png");
    r_image.save(output_path.get_file_path()).unwrap();
    let output_path = file::FilePath::new("data/result/", "result_g.png");
    g_image.save(output_path.get_file_path()).unwrap();
    let output_path = file::FilePath::new("data/result/", "result_b.png");
    b_image.save(output_path.get_file_path()).unwrap();
}

fn print_image_info(image: RgbImage) -> () {
    println!("dimensions {:?}", image.dimensions());
    //println!("{:?}", self.image.color());
}
