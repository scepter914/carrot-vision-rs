# object_detection

This crate is object detection library by rust

## Support

- [x] Binarization with RGB threshold
- [x] Calculation center of gravity for binarized image (mask image)
- [ ] Calculation moment for binarized image (mask image)
- [ ] EfficientDet lapper

## Get started (Under construction)

- add dependencies
  - See <https://github.com/scepter914/simple-image-interface-rs>

```
sudo apt install libv4l-dev
```

- Set config file

```
output_directory: /home/user_name/result/
camera_input:
  width:
  height:
  fps:
camera_internal_param:
camera_external_param:
target_tracking_param:
calibration_camera_param:
rgb_threshold:
  high_threshold :  [255, 255, 80]
  low_threshold : [100, 0, 0]
```

- Run example

```sh
# for image
cargo run --example threshold_tracking --input=path_to_image.png --config=config/config.yaml

# for camera device
cargo run --example threshold_tracking --input=dev/video0 --config=config/config.yaml

# for moview
cargo run --example threshold_tracking --input=path_to_movie.mp4 --config=config/config.yaml
```

- Log::Level
  - Error : Error message from processing
  - Warning : Warning message from processing
  - Info (Default level) : Information message from initialization, last process, result
    - Work real-time process, not logging for information from each loop
  - Debug : Information message from each loop information
    - Work real-time process as possible by only logging message from each loop
    - Use visualization window instead of save images
  - Trace : Debug image from each loop to analyse bug
    - Not Work real-time process, saving all debug image
    - Should use movie (mp4) instead of camera real-time input because of heavy processing

