//! Type definitions used in Vulx

use std::ops::Index;

use crate::{
    geometry::{Line, Path, PathGeometry},
    Image, Instance, LogicalDevice,
};

/// Vector type with fixed number of elements at 2
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Vec2<T>(pub(crate)T, pub(crate) T)
where
    T: Clone + Copy;

impl<T> Vec2<T>
where
    T: Clone + Copy,
{
    pub fn new(a: T, b: T) -> Self {
        Self(a, b)
    }

    pub fn iter(&self) -> Vec2Iterator<T> {
        Vec2Iterator(0, *self)
    }

    pub fn as_ptr(&self) -> *const T {
        [self.0, self.1].as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        [self.0, self.1].as_mut_ptr()
    }
}

impl<T> Index<usize> for Vec2<T>
where
    T: Clone + Copy,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("Out of range.")
        }
    }
}

impl<T> IntoIterator for Vec2<T>
where
    T: Clone + Copy,
{
    type Item = T;

    type IntoIter = Vec2Iterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Vec2Iterator<T>(usize, Vec2<T>)
where
    T: Clone + Copy;

impl<T> Iterator for Vec2Iterator<T>
where
    T: Clone + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 < 3 {
            let result = match self.0 {
                0 => Some(self.1 .0),
                1 => Some(self.1 .1),
                _ => None,
            };
            self.0 += 1;
            result
        } else {
            None
        }
    }
}

/// Vector type with fixed number of elements at 3
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Vec3<T>(T, T, T)
where
    T: Clone + Copy;

impl<T> Vec3<T>
where
    T: Clone + Copy,
{
    pub fn new(a: T, b: T, c: T) -> Self {
        Self(a, b, c)
    }

    pub fn iter(&self) -> Vec3Iterator<T> {
        Vec3Iterator(0, *self)
    }

    pub fn as_ptr(&self) -> *const T {
        [self.0, self.1, self.2].as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        [self.0, self.1, self.2].as_mut_ptr()
    }
}

impl<T> Index<usize> for Vec3<T>
where
    T: Clone + Copy,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Out of range.")
        }
    }
}

impl<T> IntoIterator for Vec3<T>
where
    T: Clone + Copy,
{
    type Item = T;

    type IntoIter = Vec3Iterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Vec3Iterator<T>(usize, Vec3<T>)
where
    T: Clone + Copy;

impl<T> Iterator for Vec3Iterator<T>
where
    T: Clone + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 < 3 {
            let result = match self.0 {
                0 => Some(self.1 .0),
                1 => Some(self.1 .1),
                2 => Some(self.1 .2),
                _ => None,
            };
            self.0 += 1;
            result
        } else {
            None
        }
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

impl<T> Index<usize> for Vec4<T>
where
    T: Clone + Copy,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3
            _ => panic!("Out of range.")
        }
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
    fn begin(&mut self);
    fn fill<P>(&mut self, path: &mut P, color: Color, thickness: f64)
    where
        P: IntoPath;
    fn stroke<P>(&mut self, path: P, color: Color, thickness: f64)
    where
        P: IntoPath;
    fn end(&mut self);

    fn set_image(&mut self, image: Image);
}

pub trait IntoPath {
    fn into_path(
        &mut self,
        instance: &Instance,
        phsyical_device: PhysicalDevice,
        device: &LogicalDevice,
    ) -> Path;
}
