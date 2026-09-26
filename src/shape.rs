use crate::Line;
pub struct Shape {
    pub lines: Vec<Line>,
}

impl Shape {
    pub fn to_path_data(&self) -> svg::node::element::path::Data {
        let mut data  = svg::node::element::path::Data::new()
            .move_to(self.lines[0].from);
        for line in self.lines.iter() {
            data = data.line_to(line.to)
        };
        data = data.line_to(self.lines[0].from);
        data.close()
    }
}
