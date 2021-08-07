
## Whole Architecture

- /examples
  - /config
    - camera_internal_param.yaml
    - camera_external_param.yaml
    - target_tracking_param.yaml
    - calibration_camera_param.yaml
  - target_tracking.rs
  - calibration_camera.rs
  - calculate_fps.rs
- /src
  - /tool
    - calibration_camera.rs
  - /geometory
    - self_window.rs: input cg return x1, y1, x2, y2
    - object_2d_to_3d.rs: circle, rectangle 3d estimation using model assumption
  - /object_detection
    - threshold_detection.rs
    - threshold_detection_depth.rs
    - dnn_detection.rs
  - /ros
    - ros_pub.rs
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
    - camera_interface.rs: input:camera
    - image_interface.rs: input, output: mp4
    - output_interface.rs: output:eathernet

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
### ros_package

- To process with high-speed, not subscribe image by ros topic
  - But publish compress image for debug option

### target_tracking.rs

```
target_tracking --debug={False/cg/threshold} --input={hoge/image.png, hoge/video.mp4, dev/video0} --config={config/config.yaml}
```

- config file

```
output_directory: /result
camera:
  width:
  height:
  fps:
```

### threshold_detection.rs: 

- RGBtheres, HSVtheres
- RGBdetection::get_target_cg: cg
- RGBdetection::get_target_moment: cg + moment 
- RGBdetection::get_target_circle: cg + radius
- RGBdetection::get_target_circle_optitau: cg + radius + optical_tau
- RGBdetection::get_rectangle: cg + rectangle; x1, y1, x2, y2 

### threshold_detection_depth.rs: 

- RGBDtheres, RGBtheres, HSVDtheres, HSVtheres
- RGBDdetection::get_target_cg: cg + depth
- RGBDdetection::get_target_moment: cg + moment + depth
- RGBDdetection::get_target_circle: cg + radius + depth
- RGBDdetection::get_rectangle: rectangle; cg + x1, y1, x2, y2 
