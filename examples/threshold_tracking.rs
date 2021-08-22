extern crate image;
extern crate imageproc;

use image::imageops;
use image::RgbImage;
use imageproc::point::Point;
use log::{log_enabled, Level};
use std::env;

use realtime_cv::object_detection::threshold_detection;
use realtime_cv_lib::convertor::rgb_to_binary;
use realtime_cv_lib::{debug, image_util, logger};

fn threshold_detection(input_image: &RgbImage, logger: &logger::Logger) {
    let rgb_threshold = rgb_to_binary::RGBThreshold {
        high_threshold: image::Rgb([255, 255, 80]),
        low_threshold: image::Rgb([100, 0, 0]),
    };

    let binarized_image: image::GrayImage;
    let cg: Point<f32>;
    if log_enabled!(Level::Info) {
        let benchmark = debug::Benchmark::set_start_time();
        binarized_image =
            rgb_to_binary::convert_to_binary_image_by_threshold(&input_image, &rgb_threshold);
        cg = threshold_detection::get_cg_from_binary(&binarized_image);
        benchmark.print_bench_time();
        debug::print_point_info("cg", &cg);
        if log_enabled!(Level::Debug) {
            debug::print_pixel_from_point("cg", &input_image, cg);
        }
    } else {
        binarized_image =
            rgb_to_binary::convert_to_binary_image_by_threshold(&input_image, &rgb_threshold);
        cg = threshold_detection::get_cg_from_binary(&binarized_image);
    }

    // If log level is Debug or Trace, then save the image of center gravity debug image
    if log_enabled!(Level::Debug) {
        let cg_image = threshold_detection::get_cg_debug_image(&input_image, &cg);
        // cg_image
        //     .save(logger.get_time_path("threshold_detection_", "png"))
        //     .unwrap();
        image_util::save_ppm_file(
            &cg_image,
            std::path::Path::new(&logger.get_time_path("threshold_detection_", "ppm")),
        );
    }

    // If log level is Trace, then save the image of threshold rgb debug image
    if log_enabled!(Level::Trace) {
        let rgb_disassembled_image =
            rgb_to_binary::get_rgb_threshold_debug_image(&input_image, &rgb_threshold);
        rgb_disassembled_image
            .save(logger.get_full_path("result_rgb_layer.png"))
            .unwrap();
    }
}

fn main() {
    let log_level;
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        log_level = "Error";
    } else {
        log_level = &args[1];
    }

    let logger = logger::Logger::init(log_level, "data/result");

    let input_image_path = "data/ball_2.jpg";
    let input_image_ = image::open(input_image_path).unwrap().to_rgb8();
    let input_image = imageops::resize(&input_image_, 640, 360, imageops::Gaussian);
    debug::print_image_info(&input_image);

    threshold_detection(&input_image, &logger);
}
