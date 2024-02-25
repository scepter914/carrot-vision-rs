use crate::geometry::point;
use imageproc::point::Point;
use imageproc::rect::Rect;

pub fn make_rect_from_chw(center_point: Point<f32>, width: u32, height: u32) -> Rect {
    let x;
    let y;
    (x, y) = point::to_i32_tuple(center_point);
    Rect::at(x - width as i32 / 2, y - height as i32 / 2).of_size(width, height)
}
