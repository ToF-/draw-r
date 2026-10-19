use crate::Line;

pub fn distance(line: &Line) -> f64 {
    let (x0, y0) = line.from;
    let (x1, y1) = line.to;
    let dx = x1 - x0;
    let dy = y1 - y0;
    (dx * dx + dy * dy).sqrt()
}
