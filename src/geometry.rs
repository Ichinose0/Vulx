use std::ffi::c_void;

use crate::{
    identity, Destroy, Instance, IntoPath, LogicalDevice, Mat4, PhysicalDevice, Vec2, Vec3, Vec4,
};
use ash::vk::{
    BufferCreateInfo, DeviceMemory, MappedMemoryRange, MemoryAllocateInfo, MemoryMapFlags,
    MemoryPropertyFlags, PhysicalDeviceMemoryProperties,
};

/// # Represents a line segment
/// ## Members
/// * `start` - starting coordinate.
/// * `end` - ending coordinate.
pub struct Line {
    start: Vec2<f32>,
    end: Vec2<f32>,
}

impl Line {
    /// # Example
    /// ```no_run
    /// use vulx::{Line,Vec2};
    /// let line = Line::new(Vec2::new(30.0,30.0),Vec2::new(100.0,70.0));
    /// ```
    pub fn new(start: Vec2<f32>, end: Vec2<f32>) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> Vec2<f32> {
        self.start
    }

    pub fn end(&self) -> Vec2<f32> {
        self.end
    }
}

pub enum BufferUsage {
    Vertex,
    Uniform,
}

pub(crate) struct Buffer {
    pub(crate) buffer: ash::vk::Buffer,
    pub(crate) mem_prop: PhysicalDeviceMemoryProperties,
    pub(crate) memory: Option<DeviceMemory>,
    pub(crate) size: usize,
}

impl Buffer {
    pub fn new(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &LogicalDevice,
        size: usize,
        usage: BufferUsage,
    ) -> Self {
        let usage = match usage {
            BufferUsage::Vertex => ash::vk::BufferUsageFlags::VERTEX_BUFFER,
            BufferUsage::Uniform => ash::vk::BufferUsageFlags::UNIFORM_BUFFER,
        };
        let create_info = BufferCreateInfo::builder()
            .size(size as u64)
            .usage(usage)
            .sharing_mode(ash::vk::SharingMode::EXCLUSIVE)
            .build();
        let buffer = unsafe { device.inner.create_buffer(&create_info, None) }.unwrap();

        let mem_prop = unsafe {
            instance
                .inner
                .get_physical_device_memory_properties(physical_device.0)
        };

        Self {
            buffer,
            mem_prop,
            memory: None,
            size,
        }
    }

    pub fn allocate_data(&mut self, data: *const c_void, device: &LogicalDevice) {
        let mem_req = unsafe { device.inner.get_buffer_memory_requirements(self.buffer) };
        let mut create_info = MemoryAllocateInfo::builder().allocation_size(mem_req.size);

        let mut suitable_memory_found = false;

        for i in 0..self.mem_prop.memory_type_count {
            if ((mem_req.memory_type_bits & (1 << i)) != 0
                && (self.mem_prop.memory_types[i as usize].property_flags
                    & MemoryPropertyFlags::HOST_VISIBLE)
                    .as_raw()
                    != 0)
            {
                create_info = create_info.memory_type_index(i);
                suitable_memory_found = true;
                break;
            }
        }

        if !suitable_memory_found {
            panic!("No memory available");
        }

        let memory;
        unsafe {
            memory = device.inner.allocate_memory(&create_info, None).unwrap();
            device
                .inner
                .bind_buffer_memory(self.buffer, memory, 0)
                .unwrap();
            let write_mem = device
                .inner
                .map_memory(memory, 0, self.size as u64, MemoryMapFlags::empty())
                .unwrap();
            libc::memcpy(write_mem, data, self.size);

            let mapped_memory_range = MappedMemoryRange::builder()
                .memory(memory)
                .offset(0)
                .size(self.size as u64)
                .build();

            device
                .inner
                .flush_mapped_memory_ranges(&[mapped_memory_range])
                .unwrap();
            device.inner.unmap_memory(memory);
            self.memory = Some(memory);
        }
    }
}

impl Destroy for Buffer {
    fn destroy_with_instance(&self, instance: &Instance) {}

    fn destroy_with_device(&self, device: &LogicalDevice) {
        unsafe {
            match self.memory {
                Some(x) => device.inner.free_memory(x, None),
                None => {}
            }
            device.inner.destroy_buffer(self.buffer, None);
        }
    }
}

pub struct Path {
    pub(crate) buffer: Buffer,
    pub(crate) size: usize,
}

impl Destroy for Path {
    fn destroy_with_instance(&self, instance: &Instance) {}

    fn destroy_with_device(&self, device: &LogicalDevice) {
        unsafe {
            device.inner.destroy_buffer(self.buffer.buffer, None);
            match self.buffer.memory {
                Some(x) => {
                    device.inner.free_memory(x, None);
                }
                None => {}
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct VertexData {
    pub(crate) pos: Vec4<f32>,
    pub(crate) color: Vec4<f32>,
}

pub struct Mvp {
    pub(crate) model: Mat4<f32>,
    pub(crate) view: Mat4<f32>,
    pub(crate) projection: Mat4<f32>,
}

impl Mvp {
    pub fn new(model: Mat4<f32>, view: Mat4<f32>, projection: Mat4<f32>) -> Self {
        Self {
            model,
            view,
            projection,
        }
    }
}

impl Default for Mvp {
    fn default() -> Self {
        Self::new(identity(1.0), identity(1.0), identity(1.0))
    }
}

/// Represents complex shapes that can be represented by rectangles, circles, and other figures.
pub struct PathGeometry {
    vertices: Vec<VertexData>,
}

impl PathGeometry {
    pub fn new() -> Self {
        Self { vertices: vec![] }
    }

    pub fn triangle(&mut self, vert: Vec3<Vec4<f32>>, color: Vec3<Vec4<f32>>) {
        for i in 0..3 {
            self.vertices.push(VertexData {
                pos: vert[i],
                color: color[i],
            })
        }
    }

    pub fn rectangle(&mut self, vert: Vec4<Vec4<f32>>, color: Vec4<Vec4<f32>>) {
        self.vertices.push(VertexData {
            pos: vert[0],
            color: color[0],
        });
        self.vertices.push(VertexData {
            pos: vert[2],
            color: color[2],
        });
        self.vertices.push(VertexData {
            pos: vert[3],
            color: color[3],
        });
        self.vertices.push(VertexData {
            pos: vert[2],
            color: color[2],
        });
        self.vertices.push(VertexData {
            pos: vert[0],
            color: color[0],
        });
        self.vertices.push(VertexData {
            pos: vert[1],
            color: color[1],
        });
    }

    pub fn geometries(&mut self, vertices: Vec<Vec4<f32>>, color: Vec<Vec4<f32>>) {
        for pos in vertices {
            for color in &color {
                self.vertices.push(VertexData { pos, color: *color })
            }
        }
    }

    pub fn size(&self) -> usize {
        self.vertices.len()
    }
}

impl IntoPath for PathGeometry {
    fn into_path(
        &mut self,
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &LogicalDevice,
    ) -> Path {
        let mut buffer = Buffer::new(
            instance,
            physical_device,
            device,
            (std::mem::size_of::<VertexData>() * self.vertices.len()),
            BufferUsage::Vertex,
        );
        buffer.allocate_data(self.vertices.as_ptr() as *const c_void, device);
        Path {
            buffer,
            size: self.size(),
        }
    }
}
