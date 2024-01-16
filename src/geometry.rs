use crate::{Vec3,Vec2,LogicalDevice};
use ash::vk::BufferCreateInfo;

/// # Represents a line segment
/// ## Members
/// * `start` - starting coordinate.
/// * `end` - ending coordinate.
pub struct Line {
    start: Vec2<f64>,
    end: Vec2<f64>,
}

impl Line {
    /// # Example
    /// ```
    /// use vulx::{Line,Vec2};
    /// let line = Line::new(Vec2::new(30.0,30.0),Vec2::new(100.0,70.0));
    /// ```
    pub fn new(start: Vec2<f64>, end: Vec2<f64>) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> Vec2<f64> {
        self.start
    }

    pub fn end(&self) -> Vec2<f64> {
        self.end
    }
}

pub(crate) struct Buffer {
    buffer: ash::vk::Buffer
}

impl Buffer {
    pub fn new(vertices: &[&[Vec2<f64>]],instance: &Instance,phsyical_device: PhysicalDevice,device: &LogicalDevice) -> Self {
        let vertices = &vertices[0];
        let create_info = BufferCreateInfo::builder().size((std::mem::size_of::<Vec2<f64>>()*vertices.len()) as u64).usage(ash::vk::BufferUsageFlag::VERTEX_BUFFER).sharing_mode(ash::vk::SharingMode::EXCLUSIVE).build();
        let buffer = unsafe { device.inner.create_buffer(&create_info) }.unwrap();
        let mem_prop = unsafe {
            instance
                .inner
                .get_physical_device_memory_properties(physical_device.0)
        };

        let mem_req = unsafe { device.inner.get_buffer_memory_requirements(inner) };
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
        let mut write_mem;
        unsafe {
            memory = device.inner.allocate_memory(&create_info, None).unwrap();
            device.inner.bind_buffer_memory(buffer, memory, 0).unwrap();
            write_mem = device.inner.map_memory(memory,0,(std::mem::size_of::<Vec2<f64>>()*vertices.len()) as u64);
            write_mem = vertices.as_mut_ptr() as *mut c_void;
            let mapped_memory_range = MappedMemoryRange::builder().memory(write_mem).offset(0).size((std::mem::size_of::<Vec2<f64>>()*vertices.len()) as u64).build();
            device.flush_mapped_memory_ranges(&[mapped_memory_range]);
            device.inner.unmap_memory(write_mem);
        }
        Self {
            buffer
        }
    }
}

pub struct Path {
    buffer: Buffer
}

/// Represents complex shapes that can be represented by rectangles, circles, and other figures.
pub struct PathGeometry {
    vertices: Vec<Vec<Vec2<f64>>>
}

impl PathGeometry {
    pub fn new() -> Self {
        Self {
            vertices: vec![]
        }
    }

    pub fn triangle(mut self,vert: Vec3<Vec2<f64>>>) -> Self {
        self
    }
}

impl IntoPath for PathGeometry {
    fn into(self,instance: &Instance,phsyical_device: PhysicalDevice,device: &LogicalDevice) -> Path {
        let buffer = Buffer::new(&self.vertices,instance,physical_device,device);
        Path {
            buffer
        }
    }
}