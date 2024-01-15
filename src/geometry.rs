use crate::Vec2;

pub struct Line {
    start: Vec2<f64>,
    end: Vec2<f64>,
}

impl Line {
    pub fn new(start: Vec2<f64>, end: Vec2<f64>) -> Self {
        Self { start, end }
    }
}

pub struct PathGeometry {}
