
## Whole Architecture

- /examples
  - /config
  - threshold_object_detection.rs
  - calibration_camera.rs
  - rosrust_efficient_det.rs
- /src
  - /tool
    - calibration_camera.rs
  - /geometory
    - self_window.rs: input cg return x1, y1, x2, y2
    - object_2d_to_3d.rs: circle, rectangle 3d estimation using model assumption
  - /object_detection
    - threshold_detection.rs
    - threshold_detection_depth.rs
    - efficient_det.rs
  - /ros
    - ros_publish.rs
- /ros_package
  - package.xml
  - /launch
- /tests

### /riprs_cv_lib

- /src
  - /util
    - file.rs:
    - performance.rs (calculate time)
    - logger.rs
  - /image
    - visualization.rs: cg, bbox
  - /convertor
    - visualize_rgb_threshold.rs
    - rgb_convertor.rs
    - hsv_convertor.rs
    - gray_convertor.rs
    - rgbd_convertor.rs
  - /interface
    - web_camera_interface.rs
    - real_sense_interface.rs
    - image_interface.rs
    - video_interface.rs
    - eathernet_interface.rs

### /efficient_det

- /efficient_det
  - /config
    - config_param.yaml
      - label.yaml
      - parameter.yaml
  - /src
    - train.rs
    - evaluate.rs
    - inference.rs
    - /python
      - train.py

## Detail

### threshold_detection.rs:

- RGBtheres, HSVtheres
- RGBdetection::get_target_cg: cg
- RGBdetection::get_target_moment: cg + moment
- RGBdetection::get_target_optical_tau: optical_tau
  - input: scequence moment

### threshold_detection_depth.rs:

- RGBDtheres, RGBtheres, HSVDtheres, HSVtheres
- RGBDdetection::get_target_cg: cg + depth
- RGBDdetection::get_target_moment: cg + moment + depth

## Application
### target_tracking.rs

```
target_tracking --input={hoge/image.png, hoge/video.mp4, dev/video0} --config={config/config.yaml}
```

- debug

```
RUST_LOG={error/warn/info/debug/trace} cargo run --release
```

- config file

```
output_directory: /result
camera_input:
  width:
  height:
  fps:
camera_internal_param
camera_external_param
target_tracking_param
calibration_camera_param
rgb_threshold:
  high_threshold :  [255, 255, 80]
  low_threshold : [100, 0, 0]
```

### ros_package

- To process with high-speed, not subscribe image by ros topic
  - But publish compress image for debug option
