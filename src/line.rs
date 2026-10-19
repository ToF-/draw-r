use crate::Point;

#[derive(Clone)]
pub struct Line {
    pub from: Point,
    pub to: Point,
}

impl Line {
    pub fn intersect(&self, other: &Self) -> bool {
        let (a, b) = (self.from, self.to);
        let (c, d) = (other.from, other.to);
        let o1 = cross(a, b, c);
        let o2 = cross(a, b, d);
        let o3 = cross(c, d, a);
        let o4 = cross(c, d, b);

        (o1 > 0.0 && o2 < 0.0 || o1 < 0.0 && o2 > 0.0)
            && (o3 > 0.0 && o4 < 0.0 || o3 < 0.0 && o4 > 0.0)
    }
}

fn cross(a: Point, b: Point, c: Point) -> f64 {
    (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
}
