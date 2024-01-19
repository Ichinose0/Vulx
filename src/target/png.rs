use std::{fs::File, io::BufWriter};

use ash::vk::{
    ClearValue, Extent2D, Fence, MemoryAllocateInfo, MemoryMapFlags, MemoryPropertyFlags, Offset2D,
    PipelineBindPoint, Rect2D, RenderPassBeginInfo, SubpassContents,
};

use super::CommandBuffer;

use crate::{
    geometry::PathGeometry, FrameBuffer, Image, Instance, IntoPath, LogicalDevice, PhysicalDevice,
    Pipeline, Queue, RenderPass, RenderTarget, StageDescriptor, Vec2, Vec3, Vec4,
};

pub struct PngRenderTarget {
    pub(crate) buffer: CommandBuffer,
    pub(crate) instance: Instance,
    pub(crate) logical_device: LogicalDevice,
    pub(crate) physical_device: PhysicalDevice,
    pub(crate) queue: Queue,

    pub(crate) frame_buffer: FrameBuffer,
    pub(crate) render_pass: RenderPass,
    pub(crate) pipeline: Pipeline,

    pub(crate) vertex: u32,
    pub(crate) buffers: Vec<ash::vk::Buffer>,
    pub(crate) offsets: Vec<u64>,

    pub(crate) descriptor: StageDescriptor,

    pub(crate) image: Option<Image>,
    pub(crate) path: String,
}

impl PngRenderTarget {}

impl RenderTarget for PngRenderTarget {
    fn begin(&mut self) {
        self.buffer.begin(&self.logical_device);
        unsafe {
            let mut clear = ClearValue::default();
            clear.color.float32[0] = 1.0;
            clear.color.float32[1] = 1.0;
            clear.color.float32[2] = 1.0;
            clear.color.float32[3] = 1.0;
            let create_info = RenderPassBeginInfo::builder()
                .render_pass(self.render_pass.inner)
                .framebuffer(self.frame_buffer.inner)
                .render_area(
                    Rect2D::builder()
                        .extent(
                            Extent2D::builder()
                                .width(self.image.as_ref().unwrap().viewport.width as u32)
                                .height(self.image.as_ref().unwrap().viewport.height as u32)
                                .build(),
                        )
                        .offset(Offset2D::builder().x(0).y(0).build())
                        .build(),
                )
                .clear_values(&[clear])
                .build();
            self.logical_device.inner.cmd_begin_render_pass(
                self.buffer.cmd_buffers[0],
                &create_info,
                SubpassContents::INLINE,
            );
        }
    }

    fn fill<P>(&mut self, path: &mut P, color: crate::Color, thickness: f64)
    where
        P: IntoPath,
    {
        unsafe {
            if self.buffers.is_empty() {
                let path =
                    path.into_path(&self.instance, self.physical_device, &self.logical_device);

                self.vertex += path.size as u32;
                self.buffers.push(path.buffer.buffer);
                self.offsets.push(0);
            }
        }
    }

    fn stroke<P>(&mut self, path: P, color: crate::Color, thickness: f64)
    where
        P: IntoPath,
    {
    }

    fn end(&mut self) {
        unsafe {
            println!("End");
            self.logical_device.inner.cmd_bind_pipeline(
                self.buffer.cmd_buffers[0],
                PipelineBindPoint::GRAPHICS,
                self.pipeline.inner,
            );
            println!("Bind");
            self.logical_device.inner.cmd_bind_vertex_buffers(
                self.buffer.cmd_buffers[0],
                0,
                &self.buffers,
                &self.offsets,
            );
            println!("Vertex buffer");
            self.logical_device.inner.cmd_bind_descriptor_sets(
                self.buffer.cmd_buffers[0],
                PipelineBindPoint::GRAPHICS,
                self.descriptor.pipeline_layout,
                0,
                &[self.descriptor.desc_sets[0]],
                &[],
            );
            self.logical_device.inner.cmd_draw(
                self.buffer.cmd_buffers[0],
                self.vertex,
                self.buffers.len() as u32,
                0,
                0,
            );
            println!("Draw");
            self.logical_device
                .inner
                .cmd_end_render_pass(self.buffer.cmd_buffers[0]);
        }
        self.buffer.end(&self.logical_device);
        self.buffer.submit(
            &self.logical_device,
            self.queue,
            Fence::null(),
            &[],
            &[],
            &[],
        );
        let file = File::create(&self.path).unwrap();
        let w = &mut BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, 640, 480);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();

        let data = self.image.unwrap().map_memory(&self.logical_device);
        let slice: &[u8] = unsafe { std::slice::from_raw_parts(data as *const u8, 1228800) };
        writer.write_image_data(&slice).unwrap();
        unsafe {
            self.logical_device
                .inner
                .queue_wait_idle(self.queue.0)
                .unwrap();
        }
    }

    fn set_image(&mut self, image: crate::Image) {
        self.image = Some(image);
    }

    fn logical_device(&self) -> &LogicalDevice {
        &self.logical_device
    }
}

impl Drop for PngRenderTarget {
    fn drop(&mut self) {
        self.logical_device.destroy_render_pass(&self.render_pass);
        self.logical_device.destroy_command_buffer(&self.buffer);
        unsafe {
            self.logical_device
                .inner
                .destroy_image(self.image.unwrap().inner, None);
            for i in &self.buffers {
                self.logical_device.inner.destroy_buffer(*i, None);
            }
            self.logical_device
                .inner
                .destroy_framebuffer(self.frame_buffer.inner, None);
        }
    }
}
