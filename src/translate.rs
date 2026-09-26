use crate::Point;

pub fn translate_point(point: Point, dx: f64, dy: f64) -> Point {
   let (x,y) = point;
   (x+dx, y+dy)
}
