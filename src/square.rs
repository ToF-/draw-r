use crate::translate::translate_point;
use crate::Shape;
use crate::Line;


pub fn square(side: &Line) -> Shape {
    let (x0,y0) = side.from;
    let (x1,y1) = side.to;
    let dx = x1 - x0;
    let dy = y1 - y0;
    let mut lines: Vec<Line> = Vec::new();
    let mut p0 = side.from;
    let mut p1 = translate_point(p0, dx, dy);
    lines.push(Line { from: p0, to: p1 });
    p0 = p1;
    p1 = translate_point(p1, -dy, dx);
    lines.push(Line { from: p0, to: p1 });
    p0 = p1;
    p1 = translate_point(p1, -dx, -dy);
    lines.push(Line { from: p0, to: p1 });
    p0 = p1;
    p1 = translate_point(p1, dy, -dx);
    lines.push(Line { from: p0, to: p1 });
    Shape { lines, }
}
