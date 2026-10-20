use crate::distance::distance;
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
const UNIT: f64 = 25.0;

fn main() {
    let mut layer = Layer::new();
    let l1 = Line {
        from: (00.0, 500.0),
        to: (00.0, 500.0 - UNIT),
    };
    let t1 = right_triangle(&l1, 1.0);
    let s1 = square(&t1.lines.last().unwrap().reverse());
    dbg!(distance(&s1.lines[0]) / UNIT, 2.0_f64.sqrt());
    let l2 = s1.lines[0].clone().reverse();
    let t2 = right_triangle(&l2, 1.0 / 2.0_f64.sqrt());
    let s2 = square(&t2.lines.last().unwrap().reverse());
    dbg!(distance(&s2.lines[0]) / UNIT, 3.0_f64.sqrt());
    let l3 = s2.lines[3].clone();
    let t3 = right_triangle(&l3, -2.0_f64.sqrt() / 3.0_f64.sqrt());
    let s3 = square(&t3.lines.last().unwrap());
    dbg!(distance(&s3.lines[0]) / UNIT, 5.0_f64.sqrt());
    let l4 = s3.lines[3].clone();
    let t4 = right_triangle(&l4, -2.0_f64.sqrt() / 5.0_f64.sqrt());
    let s4 = square(&t4.lines.last().unwrap());
    dbg!(distance(&s4.lines[0]) / UNIT, 7.0_f64.sqrt());
    let l5 = s4.lines[3].clone();
    let t5 = right_triangle(&l5, -2.0_f64 / 7.0_f64.sqrt());
    let s5 = square(&t5.lines.last().unwrap());
    dbg!(distance(&s5.lines[0]) / UNIT, 11.0_f64.sqrt());
    let l6 = s5.lines[3].clone();
    let t6 = right_triangle(&l6, -2.0_f64.sqrt() / 11.0_f64.sqrt());
    let s6 = square(&t6.lines.last().unwrap());
    dbg!(distance(&s6.lines[0]) / UNIT, 13.0_f64.sqrt());
    let l7 = s6.lines[3].clone();
    let t7 = right_triangle(&l7, -2.0_f64 / 13.0_f64.sqrt());
    let s7 = square(&t7.lines.last().unwrap());
    dbg!(distance(&s7.lines[0]) / UNIT, 17.0_f64.sqrt());
    let l8 = s7.lines[1].clone();
    let t8 = right_triangle(&l8, -2.0_f64.sqrt() / 17.0_f64.sqrt());
    let s8 = square(&t8.lines.last().unwrap());
    dbg!(distance(&s8.lines[0]) / UNIT, 19.0_f64.sqrt());
    let l9 = s8.lines[2].clone();
    let t9 = right_triangle(&l9, -2.0_f64 / 19.0_f64.sqrt());
    let s9 = square(&t9.lines.last().unwrap());
    dbg!(distance(&s9.lines[0]) / UNIT, 23.0_f64.sqrt());
    layer.add(&s1);
    layer.add(&s2);
    layer.add(&s3);
    layer.add(&s4);
    layer.add(&s5);
    layer.add(&s6);
    layer.add(&s7);
    layer.add(&s8);
    layer.add(&s9);

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
