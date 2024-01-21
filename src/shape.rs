use std::intrinsics::sqrtf32;

use libc::c_void;

use crate::{Shape, Vec4, geometry::{IndexBuffer, VertexData, Buffer, BufferUsage, Path}, LogicalDevice, Instance, PhysicalDevice};

pub struct Rectangle {
    x: Vec4<f32>,
    y: Vec4<f32>,
    z: Vec4<f32>,
    w: Vec4<f32>
}

impl Rectangle {
    pub fn new(x: Vec4<f32>,
        y: Vec4<f32>,
        z: Vec4<f32>,
        w: Vec4<f32>) -> Self {
            Self {
                x,y,z,w
            }
        }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        let width = self.y.data.0[0][0]-self.x.data.0[0][0];
        let height = self.w.data.0[0][1]-self.x.data.0[0][1];

        let width = power(width,2);
        let height = power(height,2);


        let diagonal = unsafe { sqrtf32(width+height) } as f64;
        
        diagonal*diagonal/2.0
    }

    fn size(&self) -> usize {
        4
    }

    fn to_path(&self,instance: &Instance,device: &LogicalDevice,physical_device: PhysicalDevice) -> crate::geometry::Path {
        let vert = IndexBuffer {
            data: vec![
                VertexData {
                    pos: self.x,
                    color: Vec4::new(0.0,0.0,1.0,1.0),
                },
                VertexData {
                    pos: self.z,
                    color: Vec4::new(0.0,0.0,1.0,1.0),
                },
                VertexData {
                    pos: self.w,
                    color: Vec4::new(0.0,0.0,1.0,1.0),
                },
                VertexData {
                    pos: self.y,
                    color: Vec4::new(0.0,0.0,1.0,1.0),
                },
            ],
            indices: vec![0, 1, 2, 1, 0, 3],
        };
        let mut buffer = Buffer::new(
            instance,
            physical_device,
            device,
            std::mem::size_of::<VertexData>() * vert.data.len(),
            BufferUsage::Vertex,
        );
        buffer.allocate_data(vert.data.as_ptr() as *const c_void, device);
        buffer.flush_memory(device).unwrap();
        buffer.unmap_memory(device).unwrap();
        let mut index_buffer = Buffer::new(
            instance,
            physical_device,
            device,
            std::mem::size_of::<u32>() * vert.indices.len(),
            BufferUsage::Index,
        );
        index_buffer.allocate_data(vert.indices.as_ptr() as *const c_void, device);
        index_buffer.flush_memory(device).unwrap();
        index_buffer.unmap_memory(device).unwrap();
        Path {
            buffers: (vec![buffer]),
            index_buffers: vec![(index_buffer,vert.indices.len())],
        }
    }

    
}

fn power(mut base: f32,exponent: u32) -> f32 {
    for i in 0..exponent-1 {
        base*=base;
    }

    base
}