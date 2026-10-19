use crate::layer::Layer;
use crate::side_line::side_line;
use line::Line;
use point::Point;
use rand::prelude::*;
use right_triangle::right_triangle;
use shape::Shape;
use square::square;
use std::mem;
use svg::node::element::tag::Group;

mod distance;
mod layer;
mod line;
mod point;
mod right_triangle;
mod shape;
mod side_line;
mod square;
mod translate;

const MAX_X: usize = 1000;
const MAX_Y: usize = 1000;

fn main() {
    let mut layer = Layer::new();
    let line1 = Line {
        from: (00.0, 500.0),
        to: (00.0, 450.0),
    };
    let triangle1 = right_triangle(&line1, 1.0);
    let side1 = triangle1.lines.last().unwrap().reverse();
    let square1 = square(&side1);
    layer.add(&triangle1);
    layer.add(&square1);
    let line2 = Line {
        from: (105.0, 500.0),
        to: (105.0, 450.0),
    };
    let triangle2 = right_triangle(&line2, 1.0);
    let side2 = triangle2.lines.last().unwrap().reverse();
    let triangle3 = right_triangle(&side2, 1.0 / 2.0_f64.sqrt());
    let side3 = triangle3.lines.last().unwrap();
    let square3 = square(&side3.reverse());
    layer.add(&triangle2);
    layer.add(&triangle3);
    layer.add(&square3);
    let line3 = Line {
        from: (200.0, 500.0),
        to: (200.0, 450.0),
    };
    let triangle4 = right_triangle(&line3, 2.0);
    layer.add(&triangle4);
    let side4 = triangle4.lines.last().unwrap();
    let square4 = square(&side4.reverse());
    layer.add(&square4);

    let line4 = Line {
        from: (450.0, 500.0),
        to: (450.0, 450.0),
    };
    let triangle5 = right_triangle(&line4, 1.0);
    let side5 = triangle5.lines.last().unwrap().reverse();
    let triangle6 = right_triangle(&side5, 1.0 / 2.0_f64.sqrt()); 
    let side6 = triangle6.lines.last().unwrap();
    let triangle7 = right_triangle(&side6.reverse(), 2.0 / 3.0_f64.sqrt());
    let side7 = triangle7.lines.last().unwrap();
    let square5 = square(&side7.reverse());
    layer.add(&triangle5);
    layer.add(&triangle6);
    layer.add(&triangle7);
    layer.add(&square5);
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
