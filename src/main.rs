extern crate image;
extern crate imageproc;

use image::imageops;
use image::RgbImage;
use imageproc::point::Point;

use riprs::object_detection::threshold_detection;
use riprs::riprs_cv_lib::convertor::rgb_to_binary;
use riprs::riprs_cv_lib::debug;
use riprs::riprs_cv_lib::logger;

fn threshold_detection(input_image: &RgbImage, logger: &logger::Logger) -> () {
    let rgb_threshold = rgb_to_binary::RGBThreshold {
        high_threshold: image::Rgb([255, 255, 80]),
        low_threshold: image::Rgb([100, 0, 0]),
    };
    let binarized_image =
        rgb_to_binary::convert_to_binary_image_by_threshold(&input_image, &rgb_threshold);
    let cg: Point<f32> = threshold_detection::get_cg_from_binary(&binarized_image);
    debug::print_point_info(&cg, "cg");
}

fn threshold_detection_with_debug(input_image: &RgbImage, logger: &logger::Logger) -> () {
    let rgb_threshold = rgb_to_binary::RGBThreshold {
        high_threshold: image::Rgb([255, 255, 80]),
        low_threshold: image::Rgb([100, 0, 0]),
    };

    let benchmark = debug::Benchmark::set_start_time();
    let binarized_image =
        rgb_to_binary::convert_to_binary_image_by_threshold(&input_image, &rgb_threshold);
    let cg: Point<f32> = threshold_detection::get_cg_from_binary(&binarized_image);
    benchmark.print_bench_time();

    let cg_image = threshold_detection::get_cg_debug_image(&input_image, &cg);
    debug::print_image_info(&input_image);
    debug::print_point_info(&cg, "cg");
    debug::print_pixel_from_point(&input_image, cg);
    cg_image
        .save(logger.get_time_path("threshold_detection_", "png"))
        .unwrap();

    let rgb_disassembled_image =
        rgb_to_binary::get_rgb_threshold_debug_image(&input_image, &rgb_threshold);
    rgb_disassembled_image
        .save(logger.get_full_path("result_rgb_layer.png"))
        .unwrap();
}

fn main() {
    let logger = logger::Logger::new("data/result/");

    let input_image_path = "data/ball_1.jpg";
    // let path_image = "data/ball_2.jpg";
    let input_image_ = image::open(input_image_path).unwrap().to_rgb8();
    let input_image = imageops::resize(&input_image_, 400, 320, imageops::Gaussian);

    threshold_detection(&input_image, &logger);
    threshold_detection_with_debug(&input_image, &logger);
}
