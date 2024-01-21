use std::ffi::c_void;

use crate::{
    identity, Destroy, Instance, IntoPath, LogicalDevice, Mat4, PhysicalDevice, Vec2, Vec3, Vec4,
    VlError, VlResult,
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

    /// Returns the starting point.
    pub fn start(&self) -> Vec2<f32> {
        self.start
    }

    /// Returns the ending point.
    pub fn end(&self) -> Vec2<f32> {
        self.end
    }
}

#[doc(hidden)]
pub(crate) enum BufferUsage {
    Vertex,
    Uniform,
    Index,
}

#[doc(hidden)]
pub(crate) struct Buffer {
    pub(crate) buffer: ash::vk::Buffer,
    pub(crate) mem_prop: PhysicalDeviceMemoryProperties,
    pub(crate) memory: Option<DeviceMemory>,
    pub(crate) write_mem: Option<*mut c_void>,
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
            BufferUsage::Index => ash::vk::BufferUsageFlags::INDEX_BUFFER,
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
            write_mem: None,
            size,
        }
    }

    pub fn allocate_data(&mut self, data: *const c_void, device: &LogicalDevice) {
        let mem_req = unsafe { device.inner.get_buffer_memory_requirements(self.buffer) };
        let mut create_info = MemoryAllocateInfo::builder().allocation_size(mem_req.size);

        let mut suitable_memory_found = false;

        for i in 0..self.mem_prop.memory_type_count {
            if (mem_req.memory_type_bits & (1 << i)) != 0
                && (self.mem_prop.memory_types[i as usize].property_flags
                    & MemoryPropertyFlags::HOST_VISIBLE)
                    .as_raw()
                    != 0
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

            self.write_mem = Some(write_mem);
            self.memory = Some(memory);
        }
    }

    pub fn write(&self, data: *const c_void, size: usize) -> VlResult<()> {
        if !size == self.size {
            return Err(VlError::OutOfMemory);
        }
        match self.write_mem {
            Some(memory) => unsafe {
                libc::memcpy(memory, data, size);
            },
            None => return Err(VlError::InvalidState("Memory is not allocated.")),
        };

        Ok(())
    }

    pub fn flush_memory(&self, device: &LogicalDevice) -> VlResult<()> {
        match self.memory {
            Some(memory) => {
                let mapped_memory_range = MappedMemoryRange::builder()
                    .memory(memory)
                    .offset(0)
                    .size(self.size as u64)
                    .build();

                unsafe {
                    device
                        .inner
                        .flush_mapped_memory_ranges(&[mapped_memory_range])
                        .unwrap()
                };
            }
            None => return Err(VlError::InvalidState("Memory is not allocated.")),
        };

        Ok(())
    }

    pub fn unmap_memory(&mut self, device: &LogicalDevice) -> VlResult<()> {
        match self.memory {
            Some(memory) => {
                unsafe {
                    device.inner.unmap_memory(memory);
                };
            }
            None => return Err(VlError::InvalidState("Memory is not allocated.")),
        };
        self.write_mem = None;

        Ok(())
    }
}

impl Destroy for Buffer {
    fn destroy_with_instance(&self, instance: &Instance) {}

    fn destroy_with_device(&self, device: &LogicalDevice) {
        unsafe {
            if let Some(x) = self.memory {
                device.inner.free_memory(x, None)
            }
            device.inner.destroy_buffer(self.buffer, None);
        }
    }
}

pub struct Path {
    pub(crate) buffers: Vec<Buffer>,
    pub(crate) index_buffers: Vec<(Buffer, usize)>,
}

impl Destroy for Path {
    fn destroy_with_instance(&self, instance: &Instance) {}

    fn destroy_with_device(&self, device: &LogicalDevice) {
        // unsafe {
        //     device.inner.destroy_buffer(self.buffer.buffer, None);
        //     if let Some(x) = self.buffer.memory {
        //         device.inner.free_memory(x, None);
        //     }
        // }
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub(crate) struct VertexData {
    pub(crate) pos: Vec4<f32>,
    pub(crate) color: Vec4<f32>,
}

#[doc(hidden)]
pub(crate) struct IndexBuffer {
    pub(crate) data: Vec<VertexData>,
    pub(crate) indices: Vec<u32>,
}

#[doc(hidden)]
pub(crate) struct Mvp {
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

/// # PathGeometry
/// Represents complex shapes that can be represented by rectangles, circles, and other figures.
#[derive(Default)]
pub struct PathGeometry {
    index_buffer: Vec<IndexBuffer>,
}

impl PathGeometry {
    pub fn new() -> Self {
        Self {
            index_buffer: vec![],
        }
    }

    /// Draws a triangle.
    pub fn triangle(&mut self, vert: Vec3<Vec4<f32>>, color: Vec3<Vec4<f32>>) {
        let index_buffer = IndexBuffer {
            data: vec![
                VertexData {
                    pos: vert[0],
                    color: color[0],
                },
                VertexData {
                    pos: vert[1],
                    color: color[1],
                },
                VertexData {
                    pos: vert[2],
                    color: color[2],
                },
            ],
            indices: vec![0, 1, 2],
        };
        self.index_buffer.push(index_buffer);
    }

    /// Draws a rectangle.
    pub fn rectangle(&mut self, vert: Vec4<Vec4<f32>>, color: Vec4<Vec4<f32>>) {
        let index_buffer = IndexBuffer {
            data: vec![
                VertexData {
                    pos: vert[0],
                    color: color[0],
                },
                VertexData {
                    pos: vert[1],
                    color: color[1],
                },
                VertexData {
                    pos: vert[2],
                    color: color[2],
                },
                VertexData {
                    pos: vert[3],
                    color: color[3],
                },
            ],
            indices: vec![0, 1, 2, 1, 0, 3],
        };
        self.index_buffer.push(index_buffer);
    }

    // pub fn geometries(&mut self, vertices: Vec<Vec4<f32>>, color: Vec<Vec4<f32>>) {
    //     for pos in vertices {
    //         for color in &color {
    //             self.vertices.push(VertexData { pos, color: *color })
    //         }
    //     }
    // }

    /// Get the number of vertices.
    pub fn size(&self) -> usize {
        let mut size = 0;
        for i in &self.index_buffer {
            size += i.data.len();
        }
        size
    }
}

impl IntoPath for PathGeometry {
    fn into_path(
        &mut self,
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &LogicalDevice,
    ) -> Path {
        let mut index_buffers = vec![];
        let mut buffers = vec![];
        for i in &self.index_buffer {
            let mut buffer = Buffer::new(
                instance,
                physical_device,
                device,
                std::mem::size_of::<VertexData>() * i.data.len(),
                BufferUsage::Vertex,
            );
            buffer.allocate_data(i.data.as_ptr() as *const c_void, device);
            buffer.flush_memory(device).unwrap();
            buffer.unmap_memory(device).unwrap();
            let mut index_buffer = Buffer::new(
                instance,
                physical_device,
                device,
                std::mem::size_of::<u32>() * i.indices.len(),
                BufferUsage::Index,
            );
            index_buffer.allocate_data(i.indices.as_ptr() as *const c_void, device);
            index_buffer.flush_memory(device).unwrap();
            index_buffer.unmap_memory(device).unwrap();
            buffers.push(buffer);
            index_buffers.push((index_buffer, i.indices.len()));
        }

        Path {
            buffers,
            index_buffers,
        }
    }
}
