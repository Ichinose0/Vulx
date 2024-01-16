//! Type definitions used in Vulx

use crate::{
    geometry::{Line, PathGeometry},
    Image,
};


/// Vector type with fixed number of elements at 2
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Vec2<T>(T, T);

impl<T> Vec2<T> {
    pub fn new(a: T, b: T) -> Self {
        Self(a, b)
    }
}

/// Vector type with fixed number of elements at 3
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Vec3<T>(T, T, T);

impl<T> Vec3<T> {
    pub fn new(a: T, b: T, c: T) -> Self {
        Self(a, b, c)
    }
}

/// Vector type with fixed number of elements at 4
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Vec4<T>(T, T, T, T);

impl<T> Vec4<T> {
    pub fn new(a: T, b: T, c: T, d: T) -> Self {
        Self(a, b, c, d)
    }
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
    fn begin(&self);
    fn fill<P>(&self, path: P, color: Color, thickness: f64) where P: IntoPath;
    fn stroke<P>(&self, path: P, color: Color, thickness: f64) where P: IntoPath;
    fn end(&self);

    fn set_image(&mut self, image: Image);
}

pub trait IntoPath {
    fn into(self,instance: &Instance,phsyical_device: PhysicalDevice,device: &LogicalDevice) -> Path;
}