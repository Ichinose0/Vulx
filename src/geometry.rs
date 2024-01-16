use std::ffi::c_void;

use crate::{Instance, IntoPath, LogicalDevice, PhysicalDevice, Vec2, Vec3};
use ash::vk::{
    BufferCreateInfo, MappedMemoryRange, MemoryAllocateInfo, MemoryMapFlags, MemoryPropertyFlags,
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
    /// ```
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

pub(crate) struct Buffer {
    pub(crate) buffer: ash::vk::Buffer,
}

impl Buffer {
    pub fn new(
        vertices: &mut [Vec2<f32>],
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &LogicalDevice,
    ) -> Self {
        let create_info = BufferCreateInfo::builder()
            .size((std::mem::size_of::<Vec2<f32>>() * vertices.len()) as u64)
            .usage(ash::vk::BufferUsageFlags::VERTEX_BUFFER)
            .sharing_mode(ash::vk::SharingMode::EXCLUSIVE)
            .build();
        let buffer = unsafe { device.inner.create_buffer(&create_info, None) }.unwrap();
        let mem_prop = unsafe {
            instance
                .inner
                .get_physical_device_memory_properties(physical_device.0)
        };

        let mem_req = unsafe { device.inner.get_buffer_memory_requirements(buffer) };
        let mut create_info = MemoryAllocateInfo::builder().allocation_size(mem_req.size);

        let mut suitable_memory_found = false;

        for i in 0..mem_prop.memory_type_count {
            if ((mem_req.memory_type_bits & (1 << i)) != 0
                && (mem_prop.memory_types[i as usize].property_flags
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
            device.inner.bind_buffer_memory(buffer, memory, 0).unwrap();
            let write_mem = device
                .inner
                .map_memory(
                    memory,
                    0,
                    (std::mem::size_of::<Vec2<f32>>() * vertices.len()) as u64,
                    MemoryMapFlags::empty(),
                )
                .unwrap();
            libc::memcpy(
                write_mem,
                vertices.as_ptr() as *const c_void,
                (std::mem::size_of::<Vec2<f32>>() * vertices.len()) as usize,
            );

            let mapped_memory_range = MappedMemoryRange::builder()
                .memory(memory)
                .offset(0)
                .size((std::mem::size_of::<Vec2<f32>>() * vertices.len()) as u64)
                .build();

            device
                .inner
                .flush_mapped_memory_ranges(&[mapped_memory_range]);
            device.inner.unmap_memory(memory);
        }
        Self { buffer }
    }
}

pub struct Path {
    pub(crate) buffer: Buffer,
}

/// Represents complex shapes that can be represented by rectangles, circles, and other figures.
pub struct PathGeometry {
    vertices: Vec<Vec<Vec2<f32>>>,
}

impl PathGeometry {
    pub fn new() -> Self {
        Self {
            vertices: vec![vec![]],
        }
    }

    pub fn triangle(&mut self, vert: Vec3<Vec2<f32>>) {
        for i in vert {
            self.vertices[0].push(i);
        }
    }

    pub fn vertex(&self) -> usize {
        let mut size = 0;
        for v in &self.vertices {
            for _ in v {
                size += 1;
            }
        }
        size
    }
}

impl IntoPath for PathGeometry {
    fn into_path(
        mut self,
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &LogicalDevice,
    ) -> Path {
        let buffer = Buffer::new(&mut self.vertices[0], instance, physical_device, device);
        Path { buffer }
    }
}
