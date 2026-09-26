use svg::node::element::tag::Group;
use point::Point;
use line::Line;
use shape::Shape;
use square::square;

mod point;
mod line;
mod shape;
mod square;
mod translate;

const MAX_X: usize = 1000;
const MAX_Y: usize = 1000;

fn main() {
   let line = Line {
       from: (200.0, 350.0),
       to: (150.0, 700.0),
   };
   let square = square(&line);
   let data = square.to_path_data();
    let path = svg::node::element::Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("d", data);
    let group = svg::node::element::Group::new()
        .set("transform", format!("translate(0, {}) scale(1, -1)", MAX_Y))
        .add(path);
   let document = svg::Document::new()
       .set("viewBox", (0, 0, MAX_X, MAX_Y))
       .add(group);
   svg::save("a.svg", &document)
       .expect("can't save a.svg file");
}
