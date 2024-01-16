use crate::{Vec3,Vec2,LogicalDevice};
use ash::vk::BufferCreateInfo;

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

pub(crate) struct Buffer {
    buffer: ash::vk::Buffer
}

impl Buffer {
    pub fn new(vertices: &[Vec2<f64>],device: &LogicalDevice) -> Self {
        let create_info = BufferCreateInfo::builder().size((std::mem::size_of::<Vec2<f64>>()*vertices.len()) as u64).usage(ash::vk::BufferUsageFlag::VERTEX_BUFFER).sharing_mode(ash::vk::SharingMode::EXCLUSIVE).build();
        let buffer = unsafe { device.inner.create_buffer(&create_info) }.unwrap();
        Self {
            buffer
        }
    }
}

pub struct Path {
    buffer: Buffer
}

/// Represents complex shapes that can be represented by rectangles, circles, and other figures.
pub struct PathGeometry {
    vertices: Vec<Vec2<f64>>
}

impl PathGeometry {
    pub fn new() -> Self {
        Self {
            vertices: vec![]
        }
    }

    pub fn triangle(mut self,vert: Vec3<Vec2<f64>>>) -> Self {
        self
    }

    fn into(self,device: &LogicalDevice) -> Path {
        let buffer = Buffer::new(&self.vertices,device);
        Path {
            buffer
        }
    }
}