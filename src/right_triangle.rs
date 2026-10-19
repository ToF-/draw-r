use crate::Line;
use crate::Shape;
use crate::distance::distance;
use crate::translate::translate_point;

pub fn right_triangle(side: &Line, opp_side_length: f64) -> Shape {
    let (x0, y0) = side.from;
    let (x1, y1) = side.to;
    let dx = x1 - x0;
    let dy = y1 - y0;
    let mut lines: Vec<Line> = Vec::new();
    let p0 = side.from;
    let p1 = translate_point(p0, dx, dy);
    let p2 = translate_point(p1, -dy * opp_side_length, dx * opp_side_length );
    lines.push(Line { from: p0, to: p1 });
    lines.push(Line { from: p1, to: p2 });
    lines.push(Line { from: p2, to: p0 });
    Shape { lines }
}
