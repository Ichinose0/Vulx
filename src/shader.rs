use ash::{util::read_spv, vk::ShaderModule};
use std::io::{Cursor, Read};

const DEFAULT_VERTEX_SHADER : &'static [u8] = include_bytes!("spv/shader.vert.spv");
const DEFAULT_FRAGMENT_SHADER : &'static [u8] = include_bytes!("spv/shader.frag.spv");

use crate::Vec2;

///Indicates shader type
///
/// # Value Meaning
/// * `Vertex` - Vertex shader.
/// * `Fragment` - Fragment shader.
#[derive(Clone, Copy, Debug)]
pub enum ShaderKind {
    Vertex,
    Fragment,
}

/// Represents a Spir-V intermediate representation
///
/// This structure contains binary data that has been processed so that Vulkan can read it
///
/// # Example
/// ```
/// let fragment_shader = device
/// .create_shader_module(
///     Spirv::new("examples/shader/shader.frag.spv"),
///     ShaderKind::Fragment,
/// )
/// .unwrap();
/// ```
pub struct Spirv {
    pub(crate) data: Vec<u32>,
}

impl Spirv {
    /// Process the spv file so that Vulkan can read it
    /// # Arguments
    ///
    /// * `file` - Spv file path.
    pub fn new(file: &str) -> Self {
        let mut file = std::fs::File::open(file).expect("file open failed");
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).expect("file read failed");
        let mut spirv_file = Cursor::new(&buf);
        let spirv = read_spv(&mut spirv_file).unwrap();

        Self { data: spirv }
    }

    pub fn fragment_default() -> Self {
        let mut spirv_file = Cursor::new(&DEFAULT_FRAGMENT_SHADER);
        let spirv = read_spv(&mut spirv_file).unwrap();

        Self { data: spirv }
    }

    pub fn vertex_default() -> Self {
        let mut spirv_file = Cursor::new(&DEFAULT_VERTEX_SHADER);
        let spirv = read_spv(&mut spirv_file).unwrap();

        Self { data: spirv }
    }
}

/// Represents a shader
///
/// It can be created with create_shader_module from Device
#[derive(Clone, Copy, Debug)]
pub struct Shader {
    pub(crate) inner: ShaderModule,
    pub(crate) kind: ShaderKind,
}
