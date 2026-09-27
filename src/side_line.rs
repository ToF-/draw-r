use crate::Line;
use crate::translate::translate_point;

pub fn side_line(line: &Line, factor: f64) -> Line {
    let (x0, y0) = line.from;
    let (x1, y1) = line.to;
    let dx = x1 - x0;
    let dy = y1 - y0;
    let (x2, y2) = translate_point((x1, y1), -dy / factor, dx / factor);
    Line {
        from: (x0, y0),
        to: (x2, y2),
    }
}
