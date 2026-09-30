use crate::Shape;
pub struct Layer {
    pub shapes: Vec<Shape>,
}

impl Layer {
    pub fn new() -> Self {
        Self { shapes: Vec::new() }
    }
    pub fn add(&mut self, shape: &Shape) {
        self.shapes.push(shape.clone())
    }

    pub fn to_path_data(&self) -> svg::node::element::path::Data {
        let mut data = svg::node::element::path::Data::new();
        for shape in self.shapes.iter() {
            data = shape.append_to_path_data(data)
        }
        data
    }

    pub fn intersect(&self, shape: &Shape) -> bool {
        for self_shape in &self.shapes {
            if shape.intersect(&self_shape) {
                return true
            }
        }
        false
    }
}
