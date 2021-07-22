
## Whole Architecture

- /examples
  - /config
    - camera_internal_param.yaml
    - camera_external_param.yaml
    - target_tracking_param.yaml
    - calibration_camera_param.yaml
  - target_tracking.rs --mode=test/debug/ --output=$HOME/result
  - calibration_camera.rs
  - calculate_fps.rs
- /tests
- /src
  - /tool
      - calibration_camera.rs
  - /geometory
    - self_window.rs: input cg return x1, y1, x2, y2
    - object_2d_to_3d.rs circle, rectangleの3次元推定(model仮定)
  - /object_detection
    - threshold_detection.rs
    - threshold_detection_depth.rs
    - efficient_det.rs
- /ros_package
  - package.xml
  - /launch
  - /src
    - ros_pub.rs

### /riprs_cv_lib

- /src
  - /util
    - file.rs:
    - performance.rs (calculate time)
    - logger.rs
  - /image
    - image_converter.rs: RGB to Gray, RGB to HSV, 2値化
    - visualization.rs
  - /interface
    - camera_interface.rs: input:camera
      - RGBD -> RGB
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
      - train.python

## Detail
### ros_package

- 画像で通信しないようにする
- inputは無しでoutputに画像をcompressしてdebugするかどうか

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

