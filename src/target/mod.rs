#[cfg(target_os = "windows")]
#[cfg(feature = "window")]
mod hwnd;
mod png;
pub(crate) mod surface;
#[cfg(feature = "window")]
pub(crate) mod swapchain;
use ash::vk::{
    CommandBufferBeginInfo, CommandPool, Fence, ImageLayout, PipelineStageFlags, Semaphore,
    SubmitInfo,
};
#[cfg(target_os = "windows")]
#[cfg(feature = "window")]
pub use hwnd::*;
pub use png::*;

use crate::{
    FrameBuffer, Image, Instance, LogicalDevice, PhysicalDevice, Pipeline, Queue, RenderPass,
};

pub struct RenderTargetBuilder {
    buffer: Option<CommandBuffer>,
    device: Option<LogicalDevice>,
    physical_device: Option<PhysicalDevice>,
    instance: Option<Instance>,
    queue: Option<Queue>,
    frame_buffer: Option<FrameBuffer>,
    renderpass: Option<RenderPass>,
    pipeline: Option<Pipeline>,
    image: Option<Image>,
}

impl RenderTargetBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn instance(mut self, instance: Instance) -> Self {
        self.instance = Some(instance);
        self
    }

    pub fn queue(mut self, queue: Queue) -> Self {
        self.queue = Some(queue);
        self
    }

    pub fn renderpass(mut self, renderpass: RenderPass) -> Self {
        self.renderpass = Some(renderpass);
        self
    }

    pub fn frame_buffer(mut self, frame_buffer: FrameBuffer) -> Self {
        self.frame_buffer = Some(frame_buffer);
        self
    }

    pub fn pipeline(mut self, pipeline: Pipeline) -> Self {
        self.pipeline = Some(pipeline);
        self
    }

    pub fn logical_device(mut self, device: LogicalDevice) -> Self {
        self.device = Some(device);
        self
    }

    pub fn physical_device(mut self, physical_device: PhysicalDevice) -> Self {
        self.physical_device = Some(physical_device);
        self
    }

    pub fn command_buffer(mut self, command_buffer: CommandBuffer) -> Self {
        self.buffer = Some(command_buffer);
        self
    }

    pub fn image(mut self, image: Option<Image>) -> Self {
        self.image = image;
        self
    }

    #[cfg(target_os = "windows")]
    #[cfg(feature = "window")]
    pub fn build_hwnd(self, hwnd: isize, hinstance: isize) -> Result<HwndRenderTarget, ()> {
        use ash::vk::{FenceCreateFlags, FenceCreateInfo, SemaphoreCreateInfo};
        use libc::c_void;

        use crate::{ShaderKind, Spirv, SubPass};

        let buffer = match self.buffer {
            Some(b) => b,
            None => return Err(()),
        };
        let physical_device = match self.physical_device {
            Some(b) => b,
            None => return Err(()),
        };
        let device = match self.device {
            Some(b) => b,
            None => return Err(()),
        };
        let instance = match self.instance {
            Some(b) => b,
            None => return Err(()),
        };
        let queue = match self.queue {
            Some(b) => b,
            None => return Err(()),
        };
        let surface = surface::Surface::create_for_win32(
            &instance,
            hwnd as *const c_void,
            hinstance as *const c_void,
        );
        let swapchain = device
            .create_swapchain(&instance, physical_device, &surface)
            .unwrap();
        let subpasses = vec![SubPass::new()];

        let render_pass = RenderPass::new(&device, &subpasses);

        let images = match unsafe { swapchain.inner.get_swapchain_images(swapchain.khr) } {
            Ok(i) => i,
            Err(_) => panic!("Err"),
        };

        let mut frame_buffers = vec![];
        let image_view = swapchain.get_image(&device, &images).unwrap();

        for i in &image_view {
            frame_buffers.push(
                i.create_frame_buffer(
                    &device,
                    &render_pass,
                    self.image.as_ref().unwrap().viewport.width as u32,
                    self.image.as_ref().unwrap().viewport.height as u32,
                )
                .unwrap(),
            );
        }

        let fragment_shader = device
            .create_shader_module(
                Spirv::new("examples/shader/shader.frag.spv"),
                ShaderKind::Fragment,
            )
            .unwrap();
        let vertex_shader = device
            .create_shader_module(
                Spirv::new("examples/shader/shader.vert.spv"),
                ShaderKind::Vertex,
            )
            .unwrap();

        let pipeline = render_pass
            .create_pipeline(
                &self.image.unwrap().inner,
                &device,
                &[fragment_shader, vertex_shader],
                800,
                600,
            )
            .unwrap();
        let create_info = FenceCreateInfo::builder()
            .flags(FenceCreateFlags::SIGNALED)
            .build();
        let fence = unsafe { device.inner.create_fence(&create_info, None) }.unwrap();
        let create_info = SemaphoreCreateInfo::builder().build();
        let swapchain_semaphore =
            unsafe { device.inner.create_semaphore(&create_info, None) }.unwrap();
        let rendered_semaphore =
            unsafe { device.inner.create_semaphore(&create_info, None) }.unwrap();
        Ok(HwndRenderTarget {
            instance,
            buffer,
            logical_device: device,
            physical_device,
            queue,
            frame_buffers,
            image_view,
            images,
            render_pass,
            pipeline,
            image: self.image,
            surface,
            swapchain,
            fence,
            img_index: 0,
            vertex: 0,
            buffers: vec![],
            offsets: vec![],
            swapchain_semaphore,
            rendered_semaphore,

            width: 800,
            height: 600,

            fragment_shader,
            vertex_shader,
        })
    }

    pub fn build_png(self, file_path: &str) -> Result<PngRenderTarget, ()> {
        let buffer = match self.buffer {
            Some(b) => b,
            None => return Err(()),
        };
        let physical_device = match self.physical_device {
            Some(b) => b,
            None => return Err(()),
        };
        let device = match self.device {
            Some(b) => b,
            None => return Err(()),
        };
        let instance = match self.instance {
            Some(b) => b,
            None => return Err(()),
        };
        let queue = match self.queue {
            Some(b) => b,
            None => return Err(()),
        };
        let frame_buffer = match self.frame_buffer {
            Some(b) => b,
            None => return Err(()),
        };
        let renderpass = match self.renderpass {
            Some(b) => b,
            None => return Err(()),
        };
        let pipeline = match self.pipeline {
            Some(b) => b,
            None => return Err(()),
        };
        Ok(PngRenderTarget {
            instance,
            buffer,
            logical_device: device,
            physical_device,
            queue,
            frame_buffer,
            render_pass: renderpass,
            pipeline,
            image: self.image,
            path: file_path.to_owned(),
            vertex: 0,
            buffers: vec![],
            offsets: vec![],
        })
    }
}

impl Default for RenderTargetBuilder {
    fn default() -> Self {
        Self {
            buffer: None,
            device: None,
            physical_device: None,
            instance: None,
            queue: None,
            frame_buffer: None,
            renderpass: None,
            pipeline: None,
            image: None,
        }
    }
}

pub struct CommandBuffer {
    pub(crate) command_pool: CommandPool,
    cmd_buffers: Vec<ash::vk::CommandBuffer>,
}

impl CommandBuffer {
    pub fn new(device: &LogicalDevice, queue_family_index: usize) -> Self {
        let command_pool = device.create_command_pool(queue_family_index);
        let cmd_buffers = device.allocate_command_buffer(command_pool);
        Self {
            command_pool,
            cmd_buffers,
        }
    }

    pub(crate) fn begin(&self, device: &LogicalDevice) {
        unsafe {
            let begin_info = CommandBufferBeginInfo::builder().build();
            device
                .inner
                .begin_command_buffer(self.cmd_buffers[0], &begin_info)
                .unwrap();
        }
    }

    pub(crate) fn end(&self, device: &LogicalDevice) {
        unsafe {
            device
                .inner
                .end_command_buffer(self.cmd_buffers[0])
                .unwrap();
        }
    }

    pub(crate) fn submit(
        &self,
        device: &LogicalDevice,
        queue: Queue,
        fence: Fence,
        semaphores: &[Semaphore],
        signal_semaphores: &[Semaphore],
        wait_dst_stage_mask: &[PipelineStageFlags],
    ) {
        let submit_cmd_buf = vec![self.cmd_buffers[0]];
        let info = vec![SubmitInfo::builder()
            .command_buffers(&submit_cmd_buf)
            .wait_semaphores(semaphores)
            .signal_semaphores(&signal_semaphores)
            .wait_dst_stage_mask(wait_dst_stage_mask)
            .build()];
        unsafe {
            device.inner.queue_submit(queue.0, &info, fence).unwrap();
        }
    }
}
