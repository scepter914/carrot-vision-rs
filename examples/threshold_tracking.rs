use image::imageops;
use image::RgbImage;
use imageproc::point::Point;
use log::LevelFilter;
use log::{log_enabled, Level};
use std::env;
use std::path::PathBuf;

use realtime_cv::object_detection::threshold_detection;
use realtime_cv_lib::convertor::rgb_to_binary;
use realtime_cv_lib::{debug, image_util};

use carrot_util_library::file;
use carrot_util_library::logger::CarrotLogger;

fn threshold_detection(input_image: &RgbImage, log_directory_path: PathBuf) {
    let rgb_threshold = rgb_to_binary::RGBThreshold {
        high_threshold: image::Rgb([255, 255, 80]),
        low_threshold: image::Rgb([100, 0, 0]),
    };

    let binarized_image: image::GrayImage;
    let cg: Point<f32>;
    if log_enabled!(Level::Info) {
        let benchmark = debug::Benchmark::set_start_time();
        binarized_image = rgb_to_binary::convert_by_threshold(&input_image, &rgb_threshold);
        cg = threshold_detection::get_cg_from_binary(&binarized_image);
        benchmark.print_bench_time();
        debug::print_point_info("cg", &cg);
        if log_enabled!(Level::Debug) {
            debug::print_pixel_from_point("cg", &input_image, cg);
        }
    } else {
        binarized_image = rgb_to_binary::convert_by_threshold(&input_image, &rgb_threshold);
        cg = threshold_detection::get_cg_from_binary(&binarized_image);
    }

    // If log level is Debug or Trace, then save the image of center gravity debug image
    if log_enabled!(Level::Debug) {
        let cg_image = threshold_detection::get_cg_debug_image(&input_image, &cg);
        let file_path =
            log_directory_path.join(file::get_time_filepath("threshold_detection_", "ppm"));
        image_util::save_ppm_file(&cg_image, file_path.as_path()).unwrap();
    }

    // If log level is Trace, then save the image of threshold rgb debug image
    if log_enabled!(Level::Trace) {
        let rgb_disassembled_image =
            rgb_to_binary::get_rgb_threshold_debug_image(&input_image, &rgb_threshold);
        let rgb_disassembled_image_path = log_directory_path.join("result_rgb_layer.png");
        rgb_disassembled_image
            .save(rgb_disassembled_image_path)
            .unwrap();
    }
}

fn set_log_level(log_level: &str) -> LevelFilter {
    let logger_level: LevelFilter;
    match &log_level[..] {
        "Error" => logger_level = LevelFilter::Error,
        "Warn" => logger_level = LevelFilter::Warn,
        "Info" => logger_level = LevelFilter::Info,
        "Debug" => logger_level = LevelFilter::Debug,
        "Trace" => logger_level = LevelFilter::Trace,
        _ => logger_level = LevelFilter::Error,
    }
    logger_level
}

fn main() {
    let log_level_input;
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        log_level_input = "Error";
    } else {
        log_level_input = &args[1];
    }

    let logger_level = set_log_level(log_level_input);
    let logger = CarrotLogger::init("./data/result", logger_level, logger_level);

    let input_image_path = "./data/ball_2.jpg";
    let input_image_ = image::open(input_image_path).unwrap().to_rgb8();
    let input_image = imageops::resize(&input_image_, 640, 360, imageops::Gaussian);
    debug::print_image_info(&input_image);

    threshold_detection(&input_image, logger.root_result_directory);
}
