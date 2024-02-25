use image::Rgb;

/// RGB threshold to convert from RGB image to binary image by high and low threshold
///  high_threshold : [r, g, b]
///  low_threshold : [r, g, b]
pub struct RGBThreshold {
    pub high_threshold: Rgb<u8>,
    pub low_threshold: Rgb<u8>,
}

impl RGBThreshold {
    pub fn r_high_threshold(&self) -> u8 {
        self.high_threshold[0]
    }
    pub fn g_high_threshold(&self) -> u8 {
        self.high_threshold[1]
    }
    pub fn b_high_threshold(&self) -> u8 {
        self.high_threshold[2]
    }
    pub fn r_low_threshold(&self) -> u8 {
        self.low_threshold[0]
    }
    pub fn g_low_threshold(&self) -> u8 {
        self.low_threshold[1]
    }
    pub fn b_low_threshold(&self) -> u8 {
        self.low_threshold[2]
    }

    /// If below condition satisfy, then it return 255 (white) and the others return 0 (block).
    /// - R threshold low < pixel.r < R threshold high
    /// - G threshold low < pixel.g < G threshold high
    /// - B threshold low < pixel.b < B threshold high
    pub fn convert_to_binary_pixel_by_threshold(&self, pixel: &Rgb<u8>) -> [u8; 1] {
        let binary_pixel: [u8; 1] = if self.low_threshold[0] <= pixel[0]
            && pixel[0] <= self.high_threshold[0]
            && self.low_threshold[1] <= pixel[1]
            && pixel[1] <= self.high_threshold[1]
            && self.low_threshold[2] <= pixel[2]
            && pixel[2] <= self.high_threshold[2]
        {
            [255; 1]
        } else {
            [0; 1]
        };
        binary_pixel
    }
}
