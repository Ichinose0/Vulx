//! Type definitions used in Vulx

use std::ops::Index;

use nalgebra::{Vector2, Vector3, Vector4,Matrix4};

use crate::{
    geometry::{Line, Path, PathGeometry},
    Image, Instance, LogicalDevice,
};

pub type Vec2<T> = Vector2<T>;
pub type Vec3<T> = Vector3<T>;
pub type Vec4<T> = Vector4<T>;

pub type Mat4<T> = Matrix4<T>;

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
    fn fill<P>(&mut self, path: &mut P, color: Color, thickness: f64)
    where
        P: IntoPath;
    fn stroke<P>(&mut self, path: P, color: Color, thickness: f64)
    where
        P: IntoPath;
    fn end(&mut self);

    fn set_image(&mut self, image: Image);

    fn logical_device(&self) -> &LogicalDevice;
}

pub trait IntoPath {
    fn into_path(
        &mut self,
        instance: &Instance,
        phsyical_device: PhysicalDevice,
        device: &LogicalDevice,
    ) -> Path;
}
