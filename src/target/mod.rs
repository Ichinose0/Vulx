#[doc(hidden)]
#[cfg(target_os = "windows")]
#[cfg(feature = "window")]
mod hwnd;
#[doc(hidden)]
mod png;
#[doc(hidden)]
#[cfg(feature = "window")]
pub(crate) mod surface;
#[doc(hidden)]
#[cfg(feature = "window")]
pub(crate) mod swapchain;
use ash::vk::{
    CommandBufferBeginInfo, CommandPool, Fence, PipelineStageFlags, Semaphore, SubmitInfo,
};
#[cfg(target_os = "windows")]
#[cfg(feature = "window")]
pub use hwnd::*;
pub use png::*;

use crate::{
    FrameBuffer, Image, Instance, LogicalDevice, PhysicalDevice, Pipeline, Queue, RenderPass,
    Stage, VlError, VlResult,
};

/// # RenderTargetBuilder
/// Various render targets can be created through this builder.
/// Some render targets cannot be used without a feature flag.
#[derive(Default)]
pub struct RenderTargetBuilder {
    buffer: Option<CommandBuffer>,
    device: Option<LogicalDevice>,
    physical_device: Option<PhysicalDevice>,
    instance: Option<Instance>,
    queue: Option<Queue>,
    frame_buffer: Option<FrameBuffer>,
    renderpass: Option<RenderPass>,
    pipeline: Option<Pipeline>,
    stage: Option<Stage>,
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

    pub fn stage(mut self, stage: Stage) -> Self {
        self.stage = Some(stage);
        self
    }

    /// Windows only.  
    /// "window" feature is required
    #[cfg(target_os = "windows")]
    #[cfg(feature = "window")]
    pub fn build_hwnd(
        self,
        hwnd: isize,
        hinstance: isize,
        width: u32,
        height: u32,
        shaders: Vec<crate::Shader>,
    ) -> VlResult<HwndRenderTarget> {
        use ash::vk::{FenceCreateFlags, FenceCreateInfo, SemaphoreCreateInfo};
        use libc::c_void;

        let buffer = match self.buffer {
            Some(b) => b,
            None => return Err(VlError::MissingParameter("command_buffer")),
        };
        let physical_device = match self.physical_device {
            Some(b) => b,
            None => return Err(VlError::MissingParameter("physical_device")),
        };
        let device = match self.device {
            Some(b) => b,
            None => return Err(VlError::MissingParameter("logical_device")),
        };
        let instance = match self.instance {
            Some(b) => b,
            None => return Err(VlError::MissingParameter("instance")),
        };
        let queue = match self.queue {
            Some(b) => b,
            None => return Err(VlError::MissingParameter("queue")),
        };
        let render_pass = match self.renderpass {
            Some(b) => b,
            None => return Err(VlError::MissingParameter("render_pass")),
        };
        let pipeline = match self.pipeline {
            Some(b) => b,
            None => return Err(VlError::MissingParameter("pipeline")),
        };
        let stage = match self.stage {
            Some(b) => b,
            None => return Err(VlError::MissingParameter("stage")),
        };

        let surface = surface::Surface::create_for_win32(
            &instance,
            hwnd as *const c_void,
            hinstance as *const c_void,
        );
        let swapchain = device
            .create_swapchain(&instance, physical_device, &surface)
            .unwrap();

        let images = match unsafe { swapchain.inner.get_swapchain_images(swapchain.khr) } {
            Ok(i) => i,
            Err(_) => panic!("Err"),
        };

        let mut frame_buffers = vec![];
        let image_view = swapchain.get_image(&device, &images).unwrap();

        for i in &image_view {
            frame_buffers.push(
                i.create_frame_buffer(&device, &render_pass, width, height)
                    .unwrap(),
            );
        }

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
            pipeline: vec![pipeline],
            image: self.image,
            surface,
            swapchain,
            fence,
            img_index: 0,
            vertex: 0,
            paths: vec![],
            offsets: vec![],
            swapchain_semaphore,
            rendered_semaphore,

            stage,

            shaders,
        })
    }

    pub fn build_png(self, file_path: &str, width: u32, height: u32) -> VlResult<PngRenderTarget> {
        let buffer = match self.buffer {
            Some(b) => b,
            None => return Err(VlError::MissingParameter("command_buffer")),
        };
        let physical_device = match self.physical_device {
            Some(b) => b,
            None => return Err(VlError::MissingParameter("physical_device")),
        };
        let device = match self.device {
            Some(b) => b,
            None => return Err(VlError::MissingParameter("logical_device")),
        };
        let instance = match self.instance {
            Some(b) => b,
            None => return Err(VlError::MissingParameter("instance")),
        };
        let queue = match self.queue {
            Some(b) => b,
            None => return Err(VlError::MissingParameter("queue")),
        };
        let frame_buffer = match self.frame_buffer {
            Some(b) => b,
            None => return Err(VlError::MissingParameter("frame_buffer")),
        };
        let renderpass = match self.renderpass {
            Some(b) => b,
            None => return Err(VlError::MissingParameter("render_pass")),
        };
        let pipeline = match self.pipeline {
            Some(b) => b,
            None => return Err(VlError::MissingParameter("pipeline")),
        };
        let stage = match self.stage {
            Some(b) => b,
            None => return Err(VlError::MissingParameter("stage")),
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
            stage,
            width,
            height,
            paths: vec![],
            offsets: vec![],
        })
    }
}

#[doc(hidden)]
pub struct CommandBuffer {
    pub(crate) command_pool: CommandPool,
    cmd_buffers: Vec<ash::vk::CommandBuffer>,
}

impl CommandBuffer {
    pub fn new(device: &LogicalDevice, queue_family_index: usize) -> VlResult<Self> {
        let command_pool = device.create_command_pool(queue_family_index)?;
        let cmd_buffers = device.allocate_command_buffer(command_pool)?;
        Ok(Self {
            command_pool,
            cmd_buffers,
        })
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
            .signal_semaphores(signal_semaphores)
            .wait_dst_stage_mask(wait_dst_stage_mask)
            .build()];
        unsafe {
            device.inner.queue_submit(queue.0, &info, fence).unwrap();
        }
    }
}
