#![feature(offset_of)]

mod device;
pub mod geometry;
mod image;
mod instance;
mod pipeline;
mod queue;
mod renderpass;
mod shader;
pub mod target;
mod types;
#[cfg(feature = "util")]
pub mod util;
pub use device::*;
pub use image::*;
pub use instance::*;
pub use pipeline::*;
pub use queue::*;
pub use renderpass::*;
pub use shader::*;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geometry_size() {
        use vulx::geometry::PathGeometry;
        const VERTEX_SIZE: usize = 6;
        let mut triangle = PathGeometry::new();
    triangle.triangle(
        Vec3::new(
            Vec4::new(0.0, -0.5, 0.0, 1.0),
            Vec4::new(0.5, 0.5, 0.0, 1.0),
            Vec4::new(-0.5, 0.5, 0.0, 1.0),
        ),
        Vec3::new(
            Vec4::new(1.0, 0.0, 0.0, 1.0),
            Vec4::new(0.0, 1.0, 0.0, 1.0),
            Vec4::new(0.0, 0.0, 1.0, 1.0),
        ),
    );

    triangle.rectangle(
        Vec4::new(
            Vec4::new(-0.5, -0.5, 0.0, 1.0),
            Vec4::new(0.5, -0.5, 0.0, 1.0),
            Vec4::new(0.5, 0.5, 0.0, 1.0),
            Vec4::new(-0.5, 0.5, 0.0, 1.0),
        ),
        Vec4::new(
            Vec4::new(1.0, 0.0, 0.0, 1.0),
            Vec4::new(0.0, 1.0, 0.0, 1.0),
            Vec4::new(0.0, 0.0, 1.0, 1.0),
            Vec4::new(1.0, 1.0, 0.0, 1.0),
        ),
    );
    assert_eq!(triangle.size(),VERTEX_SIZE);
    }
}
