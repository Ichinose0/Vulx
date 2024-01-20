//! Type definitions used in Vulx

use nalgebra::{Matrix4, Vector2, Vector3, Vector4};

use crate::{geometry::Path, Image, Instance, LogicalDevice};

pub type Vec2<T> = Vector2<T>;
pub type Vec3<T> = Vector3<T>;
pub type Vec4<T> = Vector4<T>;

pub type Mat4<T> = Matrix4<T>;

pub fn identity(ident: f32) -> Mat4<f32> {
    Mat4::new(
        ident, 0.0, 0.0, 0.0, 0.0, ident, 0.0, 0.0, 0.0, 0.0, ident, 0.0, 0.0, 0.0, 0.0, ident,
    )
}

pub fn translate(x: f32, y: f32, z: f32) -> Mat4<f32> {
    Mat4::new(
        1.0, 0.0, 0.0, x, 0.0, 1.0, 0.0, y, 0.0, 0.0, 1.0, z, 0.0, 0.0, 0.0, 1.0,
    )
}

pub fn perspective(fovy: f32, aspect: f32, near: f32, far: f32) -> Mat4<f32> {
    let f = 1.0 / (fovy / 2.0).tan();

    Mat4::new(
        f / aspect,
        0.0,
        0.0,
        0.0,
        0.0,
        f,
        0.0,
        0.0,
        0.0,
        0.0,
        (far + near) / (near - far),
        -1.0,
        0.0,
        0.0,
        (2.0 * far * near) / (near - far),
        0.0,
    )
}

pub fn look_at(eye: Vec3<f32>, center: Vec3<f32>, up: Vec3<f32>) -> Mat4<f32> {
    let f = (center - eye).normalize();
    let r = up.cross(&f).normalize();
    let u = f.cross(&r).normalize();

    let m = Matrix4::new(
        r.x, u.x, -f.x, 0.0, r.y, u.y, -f.y, 0.0, r.z, u.z, -f.z, 0.0, 0.0, 0.0, 0.0, 1.0,
    );

    let translation = Matrix4::new_translation(&-eye);

    m * translation
}

pub fn radians(r: f32) -> f32 {
    use std::f32::consts::PI;
    r * (PI / 180.0)
}

/// Represents a color
///
/// Initialization with ARGB allows you to create your own colors
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    /// Specified in RGB color space.
    RGB(f64, f64, f64),
    /// Specified in RGBA color space.
    RGBA(f64, f64, f64, f64),
    /// Specified in HSV color space.
    HSV(f64, f64, f64),
}

/// A handle representing the physical device.
#[derive(Clone, Copy)]
pub struct PhysicalDevice(pub(crate) ash::vk::PhysicalDevice);

pub trait RenderTarget {
    fn begin(&mut self);
    fn fill(&mut self, path: &mut impl IntoPath);
    fn stroke(&mut self, path: &mut impl IntoPath, thickness: f64);
    fn end(&mut self);

    fn set_image(&mut self, image: Image);

    fn logical_device(&self) -> &LogicalDevice;
    fn instance(&self) -> &Instance;
}

pub trait IntoPath {
    fn into_path(
        &mut self,
        instance: &Instance,
        phsyical_device: PhysicalDevice,
        device: &LogicalDevice,
    ) -> Path;
}

pub trait Destroy {
    fn destroy_with_instance(&self, instance: &Instance);
    fn destroy_with_device(&self, device: &LogicalDevice);
}
