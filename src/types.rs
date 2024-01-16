//! Type definitions used in Vulx

use crate::{
    geometry::{Line, Path, PathGeometry},
    Image, Instance, LogicalDevice,
};

/// Vector type with fixed number of elements at 2
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Vec2<T>(T, T, usize)
where
    T: Clone + Copy;

impl<T> Vec2<T>
where
    T: Clone + Copy,
{
    pub fn new(a: T, b: T) -> Self {
        Self(a, b, 0)
    }
}

impl<T> Iterator for Vec2<T>
where
    T: Clone + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.2 {
            0 => Some(self.0),
            1 => Some(self.1),
            _ => None,
        };
        self.2 += 1;
        result
    }
}

/// Vector type with fixed number of elements at 3
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Vec3<T>(T, T, T, usize)
where
    T: Clone + Copy;

impl<T> Vec3<T>
where
    T: Clone + Copy,
{
    pub fn new(a: T, b: T, c: T) -> Self {
        Self(a, b, c, 0)
    }
}

impl<T> Iterator for Vec3<T>
where
    T: Clone + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.3 {
            0 => Some(self.0),
            1 => Some(self.1),
            2 => Some(self.2),
            _ => None,
        };
        self.3 += 1;
        result
    }
}

/// Vector type with fixed number of elements at 4
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Vec4<T>(T, T, T, T, usize)
where
    T: Clone + Copy;

impl<T> Vec4<T>
where
    T: Clone + Copy,
{
    pub fn new(a: T, b: T, c: T, d: T) -> Self {
        Self(a, b, c, d, 0)
    }
}

impl<T> Iterator for Vec4<T>
where
    T: Clone + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.4 {
            0 => Some(self.0),
            1 => Some(self.1),
            2 => Some(self.2),
            3 => Some(self.3),
            _ => None,
        };
        self.4 += 1;
        result
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
    fn fill<P>(&self, path: P, color: Color, thickness: f64)
    where
        P: IntoPath;
    fn stroke<P>(&self, path: P, color: Color, thickness: f64)
    where
        P: IntoPath;
    fn end(&self);

    fn set_image(&mut self, image: Image);
}

pub trait IntoPath {
    fn into_path(
        self,
        instance: &Instance,
        phsyical_device: PhysicalDevice,
        device: &LogicalDevice,
    ) -> Path;
}
