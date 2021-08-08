# Hishspeed CV library

- This repository is Real-time Image Processing library by RuSt

## Support

- [x] Binarization with RGB threshold
- [x] Calculation center of gravity for binarized image (mask image)
- [ ] Calculation moment for binarized image (mask image)
- [ ] EfficientDet lapper

## with debug

```
cargo run --release --log={error(e)/warn(w)/info(i)/debug(d)/trace(t)}
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
