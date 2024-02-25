use imageproc::point::Point;
use num::NumCast;

pub fn to_tuple<T>(point: Point<T>) -> (T, T) {
    (point.x, point.y)
}

pub fn to_i32_tuple<T: NumCast>(point: Point<T>) -> (i32, i32) {
    (point.x.to_i32().unwrap(), point.y.to_i32().unwrap())
}
