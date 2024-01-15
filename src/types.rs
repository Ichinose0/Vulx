use crate::{
    geometry::{Line, PathGeometry},
    Image,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Vec2<T>(T, T);

impl<T> Vec2<T> {
    pub fn new(a: T, b: T) -> Self {
        Self(a, b)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Vec3<T>(T, T, T);

impl<T> Vec3<T> {
    pub fn new(a: T, b: T, c: T) -> Self {
        Self(a, b, c)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Vec4<T>(T, T, T, T);

impl<T> Vec4<T> {
    pub fn new(a: T, b: T, c: T, d: T) -> Self {
        Self(a, b, c, d)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    RGB(f64, f64, f64),
    RGBA(f64, f64, f64, f64),
    HSV(f64, f64, f64),
}

#[derive(Clone, Copy)]
pub struct PhysicalDevice(pub(crate) ash::vk::PhysicalDevice);

pub trait RenderTarget {
    fn begin(&self);
    fn path(&self, path: PathGeometry, color: Color, thickness: f64);
    fn stroke(&self, line: Line, color: Color, thickness: f64);
    fn end(&self);

    fn set_image(&mut self, image: Image);
}
