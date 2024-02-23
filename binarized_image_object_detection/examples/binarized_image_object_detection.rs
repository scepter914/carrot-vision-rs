use ::binarized_image_object_detection::binarized_image_object_detection::Circle;
use carrot_image_utils::rgb_image::convert;
use carrot_image_utils::rgb_image::convert::CarrotRgbImage;
use carrot_image_utils::rgb_image::rgb_threshold::RGBThreshold;
use image::imageops;
use image::GrayImage;
use imageproc::drawing;
use imageproc::rect::Rect;
use std::path::PathBuf;

use carrot_image_utils::geometry::rect;
use carrot_image_utils::rgb_image::draw;
use carrot_image_utils::rgb_image::file;
use carrot_utils::benchmark::Benchmark;

use binarized_image_object_detection::binarized_image_object_detection;

fn main() {
    let mut benchmark: Benchmark = Benchmark::new();

    // Set input image
    // let input_image_path = "data/ball.jpg";
    let input_image_path = "object-detection-rs/binarized_image_object_detection/data/ball.jpg";
    std::fs::create_dir_all("results").unwrap();

    let input_image_ = image::open(input_image_path).unwrap().to_rgb8();
    let input_image = imageops::resize(&input_image_, 1280, 720, imageops::Gaussian);
    let input_image_2 = imageops::resize(&input_image_, 1280, 300, imageops::Gaussian);

    // Threshold detection
    let rgb_threshold = RGBThreshold {
        high_threshold: image::Rgb([255, 255, 80]),
        low_threshold: image::Rgb([100, 0, 0]),
    };
    benchmark.start();
    let binarized_image: GrayImage = input_image.to_binary(&rgb_threshold);
    let circle: Circle = binarized_image_object_detection::get_center_of_gravity(&binarized_image);
    benchmark.stop();
    benchmark.print();

    // Debug image
    let mut cg_image = draw::draw_point(&input_image, circle.position, image::Rgb([0, 0, 255]));

    let rect_: Rect = rect::make_rect_from_chw(
        circle.position,
        (2.0 * circle.radius) as u32,
        (2.0 * circle.radius) as u32,
    );
    cg_image = drawing::draw_hollow_rect(&cg_image, rect_, image::Rgb([0, 0, 255]));
    let file_path = PathBuf::from("results/threshold_detection.ppm");
    file::save_ppm_file(&cg_image, file_path.as_path()).unwrap();

    // Detail debug image
    let debug_image =
        convert::get_rgb_threshold_debug_image(&input_image, &rgb_threshold, 640, 360);
    let file_path = PathBuf::from("results/debug_image.ppm");
    file::save_ppm_file(&debug_image, file_path.as_path()).unwrap();
}
