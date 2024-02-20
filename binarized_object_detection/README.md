# binarized_object_detection


This crate is library for binarized based object detection.

## Support

- [x] Calculation center of gravity for binarized image
- [ ] Calculation moment for binarized image

## Get started

- Add dependencies
  - See <https://github.com/scepter914/simple-image-interface-rs>

```
sudo apt install libv4l-dev
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
