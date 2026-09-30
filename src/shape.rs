use crate::Line;
use svg::node::element::path::Data;
#[derive(Clone)]
pub struct Shape {
    pub lines: Vec<Line>,
}

impl Shape {
    pub fn to_path_data(&self) -> svg::node::element::path::Data {
        let mut data = svg::node::element::path::Data::new().move_to(self.lines[0].from);
        for line in self.lines.iter() {
            data = data.line_to(line.to)
        }
        data = data.line_to(self.lines[0].from);
        data.close()
    }
    pub fn append_to_path_data(&self, mut data: svg::node::element::path::Data) -> Data {
        for line in &self.lines {
            data = data.move_to(line.from);
            data = data.line_to(line.to);
        }
        data
    }

    pub fn intersect(&self, other: &Self) -> bool {
        for self_line in &self.lines {
            for other_line in &other.lines {
                if other_line.intersect(self_line) {
                    return true
                }
            }
        };
        false
    }
}
