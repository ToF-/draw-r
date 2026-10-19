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
    let mut layer = Layer::new();
    let mut line1 = Line {
        from: (00.0, 500.0),
        to: (00.0, 450.0),
    };
    let triangle1 = right_triangle(&line1, 50.0);
    let side1 = triangle1.lines.last().unwrap().reverse();
    let square1 = square(&side1);
    layer.add(&triangle1);
    layer.add(&square1);
    let mut line2 = Line {
        from: (100.0, 500.0),
        to: (100.0, 450.0),
    };
    let triangle2 = right_triangle(&line2, 50.0);
    let side2 = triangle2.lines.last().unwrap().reverse();
    let triangle3 = right_triangle(&side2, -100.0);

    let side3 = triangle3.lines.last().unwrap();
    let square3 = square(&side3);
    layer.add(&triangle2);
    layer.add(&triangle3);
    layer.add(&square3);
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
