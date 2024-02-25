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

pub fn to_rect<T: NumCast>(center_point: Point<f32>, width: T, height: T) -> Rect {
    let (x, y): (i32, i32) = center_point.to_i32_tuple();
    Rect::at(
        x - width.to_i32().unwrap() / 2,
        y - height.to_i32().unwrap() / 2,
    )
    .of_size(width.to_u32().unwrap(), height.to_u32().unwrap())
}
