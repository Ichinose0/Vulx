use std::{fs::File, io::BufWriter};

use ash::vk::{
    ClearValue, Extent2D, MemoryAllocateInfo, MemoryMapFlags, MemoryPropertyFlags, Offset2D,
    PipelineBindPoint, Rect2D, RenderPassBeginInfo, SubpassContents,
};

use super::CommandBuffer;

use crate::{
    geometry::PathGeometry, FrameBuffer, Image, Instance, IntoPath, LogicalDevice, PhysicalDevice,
    Pipeline, Queue, RenderPass, RenderTarget, Vec2, Vec3,
};

pub struct HwndRenderTarget {
    pub(crate) buffer: CommandBuffer,
    pub(crate) instance: Instance,
    pub(crate) logical_device: LogicalDevice,
    pub(crate) physical_device: PhysicalDevice,
    pub(crate) queue: Queue,

    pub(crate) frame_buffer: FrameBuffer,
    pub(crate) render_pass: RenderPass,
    pub(crate) pipeline: Pipeline,

    pub(crate) image: Option<Image>,

    pub(crate) surface: super::surface::Surface
}

impl HwndRenderTarget {}

impl RenderTarget for HwndRenderTarget {
    fn begin(&self) {
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

    fn fill<P>(&self, path: P, color: crate::Color, thickness: f64)
    where
        P: IntoPath,
    {
    }

    fn stroke<P>(&self, path: P, color: crate::Color, thickness: f64)
    where
        P: IntoPath,
    {
    }

    fn end(&self) {
        unsafe {
            self.logical_device.inner.cmd_bind_pipeline(
                self.buffer.cmd_buffers[0],
                PipelineBindPoint::GRAPHICS,
                self.pipeline.inner,
            );
            let mut path = PathGeometry::new();
            path.triangle(Vec3::new(
                Vec2::new(0.0, -0.5),
                Vec2::new(0.5, 0.5),
                Vec2::new(-0.5, 0.8),
            ));
            let size = path.vertex();
            let path = path.into_path(&self.instance, self.physical_device, &self.logical_device);

            self.logical_device.inner.cmd_bind_vertex_buffers(
                self.buffer.cmd_buffers[0],
                0,
                &[path.buffer.buffer],
                &[0],
            );

            self.logical_device
                .inner
                .cmd_draw(self.buffer.cmd_buffers[0], size as u32, 1, 0, 0);
            self.logical_device
                .inner
                .cmd_end_render_pass(self.buffer.cmd_buffers[0]);
        }
        self.buffer.end(&self.logical_device);
        self.buffer.submit(&self.logical_device, self.queue);
    }

    fn set_image(&mut self, image: crate::Image) {
        self.image = Some(image);
    }
}

impl Drop for HwndRenderTarget {
    fn drop(&mut self) {
        unsafe {
            self.surface.surface.destroy_surface(self.surface.surface_khr, None);    
        }
    }
}