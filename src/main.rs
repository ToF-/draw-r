use crate::layer::Layer;
use crate::side_line::side_line;
use line::Line;
use point::Point;
use rand::prelude::*;
use shape::Shape;
use square::square;
use right_triangle::right_triangle;
use std::mem;
use svg::node::element::tag::Group;

mod layer;
mod distance;
mod line;
mod point;
mod shape;
mod side_line;
mod square;
mod right_triangle;
mod translate;

const MAX_X: usize = 1000;
const MAX_Y: usize = 1000;

fn main() {
    let mut line = Line {
        from: (500.0, 500.0),
        to: (500.0, 400.0),
    };
    let mut layer = Layer::new();
    let triangle = right_triangle(&line, 50.0);
    layer.add(&triangle);
    let path = svg::node::element::Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("d", layer.to_path_data());
    let group = svg::node::element::Group::new()
        .set("transform", format!("translate(0, {}) scale(1, -1)", MAX_Y))
        .add(path);
    let document = svg::Document::new()
        .set("viewBox", (0, 0, MAX_X, MAX_Y))
        .add(group);
    svg::save("a.svg", &document).expect("can't save a.svg file");
}
