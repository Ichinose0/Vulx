use crate::Vec2;

/// # Represents a line segment
/// ## Members
/// * `start` - starting coordinate.
/// * `end` - ending coordinate.
pub struct Line {
    start: Vec2<f64>,
    end: Vec2<f64>,
}

impl Line {
    /// # Example
    /// ```
    /// use vulx::{Line,Vec2};
    /// let line = Line::new(Vec2::new(30.0,30.0),Vec2::new(100.0,70.0));
    /// ```
    pub fn new(start: Vec2<f64>, end: Vec2<f64>) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> Vec2<f64> {
        self.start
    }

    pub fn end(&self) -> Vec2<f64> {
        self.end
    }
}

/// Represents complex shapes that can be represented by rectangles, circles, and other figures.
pub struct PathGeometry {}
