use crate::Line;
use crate::Point;
use crate::Shape;

pub fn translate_point(point: Point, dx: f64, dy: f64) -> Point {
    let (x, y) = point;
    (x + dx, y + dy)
}

pub fn translate_line(line: &Line, dx: f64, dy: f64) -> Line {
    Line {
        from: translate_point(line.from, dx, dy),
        to: translate_point(line.to, dx, dy),
    }
}
pub fn translate_shape(shape: &Shape, dx: f64, dy: f64) -> Shape {
    let mut lines = Vec::new();
    for line in shape.lines.iter() {
        lines.push(translate_line(line, dx, dy))
    }
    Shape { lines }
}
