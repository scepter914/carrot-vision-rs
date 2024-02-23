use image::{GrayImage, RgbImage};
use imageproc::point::Point;

pub struct Moment {
    moment_00: f32,
    moment_10: f32,
    moment_01: f32,
    moment_20: f32,
    moment_11: f32,
    moment_02: f32,
}

impl Moment {
    pub fn new(binary_image: &GrayImage) -> Moment {
        let mut moment_00: f32 = 0.0;
        let mut moment_10: f32 = 0.0;
        let mut moment_01: f32 = 0.0;
        let mut moment_20: f32 = 0.0;
        let mut moment_11: f32 = 0.0;
        let mut moment_02: f32 = 0.0;

        let width = binary_image.width();
        let height = binary_image.height();

        for i in 0..width {
            for j in 0..height {
                let pixel = binary_image.get_pixel(i, j)[0] as f32;
                let i_f32 = i as f32;
                let j_f32 = j as f32;
                moment_00 += pixel;
                moment_10 += pixel * i_f32;
                moment_01 += pixel * j_f32;
                moment_20 += pixel * i_f32 * i_f32;
                moment_11 += pixel * i_f32 * j_f32;
                moment_02 += pixel * j_f32 * j_f32;
            }
        }

        Moment {
            moment_00,
            moment_10,
            moment_01,
            moment_20,
            moment_11,
            moment_02,
        }
    }

    pub fn to_ellipse(&self) -> Ellipse {
        let center_of_gravity_x = self.moment_10 / self.moment_00;
        let center_of_gravity_y = self.moment_01 / self.moment_00;
        let a = self.moment_20 / self.moment_00 - center_of_gravity_x * center_of_gravity_x;
        let b = 2.0 * self.moment_11 / self.moment_00 - center_of_gravity_x * center_of_gravity_y;
        let c = self.moment_02 / self.moment_00 - center_of_gravity_y * center_of_gravity_y;

        let length_x: f32 = f32::sqrt(0.5 * ((a + c) + f32::sqrt(b * b + (a - c) * (a - c))));
        let length_y: f32 = f32::sqrt(0.5 * ((a + c) - f32::sqrt(b * b + (a - c) * (a - c))));
        let theta: f32 = 0.5 * b.atan2(a - c);

        Ellipse {
            length_x,
            length_y,
            theta,
        }
    }
}

pub struct Ellipse {
    length_x: f32,
    length_y: f32,
    theta: f32,
}
