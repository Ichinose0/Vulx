use ash::vk::{
    ClearValue, CommandBufferResetFlags, Extent2D, Fence, Offset2D, PipelineBindPoint,
    PipelineStageFlags, PresentInfoKHR, Rect2D, RenderPassBeginInfo, Semaphore, SubpassContents,
};

use super::{swapchain::recreate_swapchain, CommandBuffer};

use crate::{
    geometry::Path, FrameBuffer, Image, ImageView, Instance, IntoPath, LogicalDevice,
    PhysicalDevice, Pipeline, Queue, RenderPass, RenderTarget, Shader, Stage, SubPass,
};

pub struct HwndRenderTarget {
    pub(crate) buffer: CommandBuffer,
    pub(crate) instance: Instance,
    pub(crate) logical_device: LogicalDevice,
    pub(crate) physical_device: PhysicalDevice,
    pub(crate) queue: Queue,

    pub(crate) frame_buffers: Vec<FrameBuffer>,
    pub(crate) image_view: Vec<ImageView>,
    pub(crate) images: Vec<ash::vk::Image>,
    pub(crate) render_pass: RenderPass,
    pub(crate) pipeline: Vec<Pipeline>,

    pub(crate) image: Option<Image>,

    pub(crate) surface: super::surface::Surface,
    pub(crate) swapchain: super::swapchain::Swapchain,
    pub(crate) fence: Fence,
    pub(crate) img_index: u32,

    pub(crate) vertex: u32,
    pub(crate) paths: Vec<Path>,
    pub(crate) offsets: Vec<u64>,

    pub(crate) shaders: Vec<Shader>,

    pub(crate) stage: Stage,

    pub(crate) swapchain_semaphore: Semaphore,
    pub(crate) rendered_semaphore: Semaphore,
}

impl HwndRenderTarget {}

impl RenderTarget for HwndRenderTarget {
    fn begin(&mut self) {
        unsafe {
            self.logical_device
                .inner
                .reset_command_buffer(self.buffer.cmd_buffers[0], CommandBufferResetFlags::empty())
                .unwrap();

            self.logical_device
                .inner
                .wait_for_fences(&[self.fence], true, u64::MAX)
                .unwrap();

            self.img_index = match {
                self.swapchain.inner.acquire_next_image(
                    self.swapchain.khr,
                    1000000000,
                    self.swapchain_semaphore,
                    Fence::null(),
                )
            } {
                Ok(i) => {
                    if i.1 {
                        for i in &self.frame_buffers {
                            self.logical_device.inner.destroy_framebuffer(i.inner, None);
                        }
                        self.frame_buffers.clear();
                        for i in &self.image_view {
                            self.logical_device.inner.destroy_image_view(i.inner, None);
                        }
                        self.image_view.clear();

                        let (swapchain, capabilities) = recreate_swapchain(
                            &self.instance,
                            &self.logical_device,
                            self.physical_device,
                            &self.surface,
                        );
                        self.stage.width = capabilities.current_extent.width;
                        self.stage.height = capabilities.current_extent.height;
                        self.swapchain = swapchain;

                        self.images = self
                            .swapchain
                            .inner
                            .get_swapchain_images(self.swapchain.khr)
                            .unwrap();
                        let image_view = self
                            .swapchain
                            .get_image(&self.logical_device, &self.images)
                            .unwrap();
                        let subpasses = vec![SubPass::new()];

                        self.logical_device.destroy_render_pass(&self.render_pass);
                        for i in &self.pipeline {
                            self.logical_device.destroy_pipeline(i);
                        }

                        self.render_pass = RenderPass::new(&self.logical_device, &subpasses);

                        let pipeline = Pipeline::builder()
                            .image(&Image::from(self.images[0]))
                            .logical_device(&self.logical_device)
                            .shaders(&self.shaders)
                            .width(capabilities.current_extent.width)
                            .height(capabilities.current_extent.height)
                            .stage(&mut self.stage)
                            .render_pass(&self.render_pass)
                            .build(&self.instance, self.physical_device)
                            .unwrap();

                        self.pipeline = pipeline;

                        for i in image_view {
                            self.frame_buffers.push(
                                i.create_frame_buffer(
                                    &self.logical_device,
                                    &self.render_pass,
                                    capabilities.current_extent.width,
                                    capabilities.current_extent.height,
                                )
                                .unwrap(),
                            );
                        }

                        let result = self
                            .swapchain
                            .inner
                            .acquire_next_image(
                                self.swapchain.khr,
                                1000000000,
                                self.swapchain_semaphore,
                                Fence::null(),
                            )
                            .unwrap();

                        result.0
                    } else {
                        i.0
                    }
                }
                Err(result) => {
                    if result != ash::vk::Result::SUCCESS {
                        panic!("Can't get next frame.");
                    } else if result == ash::vk::Result::SUBOPTIMAL_KHR
                        || result == ash::vk::Result::ERROR_OUT_OF_DATE_KHR
                    {
                        for i in &self.frame_buffers {
                            self.logical_device.inner.destroy_framebuffer(i.inner, None);
                        }
                        self.frame_buffers.clear();
                        for i in &self.image_view {
                            self.logical_device.inner.destroy_image_view(i.inner, None);
                        }
                        self.image_view.clear();
                        for i in &self.images {
                            self.logical_device.inner.destroy_image(*i, None);
                        }
                        self.images.clear();
                        for i in &self.pipeline {
                            self.logical_device.destroy_pipeline(i);
                        }
                        self.logical_device.destroy_render_pass(&self.render_pass);
                        println!("Cleared images");
                        self.swapchain
                            .inner
                            .destroy_swapchain(self.swapchain.khr, None);
                        let (swapchain, capabilities) = recreate_swapchain(
                            &self.instance,
                            &self.logical_device,
                            self.physical_device,
                            &self.surface,
                        );
                        self.swapchain = swapchain;
                        self.images = self
                            .swapchain
                            .inner
                            .get_swapchain_images(self.swapchain.khr)
                            .unwrap();
                        let image_view = self
                            .swapchain
                            .get_image(&self.logical_device, &self.images)
                            .unwrap();

                        for i in image_view {
                            self.frame_buffers.push(
                                i.create_frame_buffer(
                                    &self.logical_device,
                                    &self.render_pass,
                                    capabilities.current_extent.width,
                                    capabilities.current_extent.height,
                                )
                                .unwrap(),
                            );
                        }

                        let result = self
                            .swapchain
                            .inner
                            .acquire_next_image(
                                self.swapchain.khr,
                                1000000000,
                                self.swapchain_semaphore,
                                Fence::null(),
                            )
                            .unwrap();

                        result.0
                    } else {
                        panic!("Unknown error.");
                    }
                }
            };

            self.logical_device
                .inner
                .reset_fences(&[self.fence])
                .unwrap();

            self.buffer.begin(&self.logical_device);

            let mut clear = ClearValue::default();

            clear.color.float32[0] = 1.0;
            clear.color.float32[1] = 1.0;
            clear.color.float32[2] = 1.0;
            clear.color.float32[3] = 1.0;
            let create_info = RenderPassBeginInfo::builder()
                .render_pass(self.render_pass.inner)
                .framebuffer(self.frame_buffers[self.img_index as usize].inner)
                .render_area(
                    Rect2D::builder()
                        .extent(
                            Extent2D::builder()
                                .width(self.stage.width)
                                .height(self.stage.height)
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

    fn fill(&mut self, path: &mut impl IntoPath) {
        let path = path.into_path(&self.instance, self.physical_device, &self.logical_device);

            // self.vertex += path.size as u32;
            self.paths.push(path);
            // self.offsets.push(0);
    }

    fn stroke(&mut self, path: &mut impl IntoPath, _: f64) {}

    fn end(&mut self) {
        unsafe {
            self.logical_device.inner.cmd_bind_pipeline(
                self.buffer.cmd_buffers[0],
                PipelineBindPoint::GRAPHICS,
                self.pipeline[0].inner,
            );
            self.logical_device.inner.cmd_bind_descriptor_sets(
                self.buffer.cmd_buffers[0],
                PipelineBindPoint::GRAPHICS,
                self.stage.descriptor.as_ref().unwrap().pipeline_layout,
                0,
                &[self.stage.descriptor.as_ref().unwrap().desc_sets[0]],
                &[],
            );
            for i in &self.paths {
                self.logical_device.inner.cmd_bind_vertex_buffers(
                    self.buffer.cmd_buffers[0],
                    0,
                    &[i.buffer.buffer],
                    &[0],
                );
                self.logical_device.inner.cmd_bind_vertex_buffers(
                    self.buffer.cmd_buffers[0],
                    0,
                    &[i.buffer.buffer],
                    &self.offsets,
                );
                self.logical_device.inner.cmd_draw(
                    self.buffer.cmd_buffers[0],
                    i.size as u32,
                    1,
                    0,
                    0,
                );
            }
            
            
            
            self.logical_device
                .inner
                .cmd_end_render_pass(self.buffer.cmd_buffers[0]);
        }
        self.buffer.end(&self.logical_device);
        let render_wait_stages = vec![PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        self.buffer.submit(
            &self.logical_device,
            self.queue,
            self.fence,
            &[self.swapchain_semaphore],
            &[self.rendered_semaphore],
            &render_wait_stages,
        );

        let present_info = PresentInfoKHR::builder()
            .swapchains(&[self.swapchain.khr])
            .image_indices(&[self.img_index])
            .wait_semaphores(&[self.rendered_semaphore])
            .build();
        unsafe {
            self.swapchain
                .inner
                .queue_present(self.queue.0, &present_info)
                .unwrap()
        };
    }

    fn set_image(&mut self, image: crate::Image) {
        self.image = Some(image);
    }

    fn logical_device(&self) -> &LogicalDevice {
        &self.logical_device
    }

    fn instance(&self) -> &Instance {
        &self.instance
    }

    fn clear(&mut self) {
        for i in &self.paths {
            self.logical_device.destroy(i);
        }
        self.paths.clear();
        self.vertex = 0;
        self.offsets.clear();
    }
}

impl Drop for HwndRenderTarget {
    fn drop(&mut self) {
        unsafe {
            self.logical_device
                .inner
                .queue_wait_idle(self.queue.0)
                .unwrap();
            self.logical_device.destroy_command_buffer(&self.buffer);
            self.logical_device.destroy_render_pass(&self.render_pass);

            for i in &self.pipeline {
                self.logical_device.destroy_pipeline(i);
            }
            self.surface
                .surface
                .destroy_surface(self.surface.surface_khr, None);
        }
    }
}
