use imageproc::point::Point;
use imageproc::rect::Rect;
use num::NumCast;

pub trait CarrotPoint<T: NumCast> {
    fn to_tuple(self) -> (T, T);
    fn to_i32_tuple(&self) -> (i32, i32);
}

impl<T: NumCast> CarrotPoint<T> for Point<T> {
    fn to_tuple(self) -> (T, T) {
        (self.x, self.y)
    }

    fn to_i32_tuple(&self) -> (i32, i32) {
        (self.x.to_i32().unwrap(), self.y.to_i32().unwrap())
    }
}

pub fn to_rect(center_point: Point<f32>, width: u32, height: u32) -> Rect {
    let x;
    let y;
    (x, y) = center_point.to_i32_tuple();
    Rect::at(x - width as i32 / 2, y - height as i32 / 2).of_size(width, height)
}
