use ash::vk::{
    CommandBuffer, CommandBufferAllocateInfo, CommandBufferLevel, CommandPool,
    CommandPoolCreateFlags, CommandPoolCreateInfo, DeviceQueueCreateInfo, ImageUsageFlags,
    ShaderModuleCreateInfo, SharingMode,
};

use crate::{
    target::surface::Surface, target::swapchain::Swapchain, Instance, PhysicalDevice, Pipeline,
    Queue, RenderPass, Shader, ShaderKind, Spirv, VlResult, VlError,
};

/// Represents a logical device.
pub struct LogicalDevice {
    pub(crate) inner: ash::Device,
}

impl LogicalDevice {
    /// Get the queue from the index number
    pub fn get_queue(&self, queue_family_index: usize) -> Queue {
        Queue(unsafe { self.inner.get_device_queue(queue_family_index as u32, 0) })
    }

    pub fn create_shader_module(&self, spirv: Spirv, kind: ShaderKind) -> VlResult<Shader> {
        let shader_create_info = ShaderModuleCreateInfo::builder().code(&spirv.data).build();
        let shader = match unsafe { self.inner.create_shader_module(&shader_create_info, None) } {
            Ok(s) => s,
            Err(e) => return Err(VlError::from(e)),
        };
        Ok(Shader {
            inner: shader,
            kind,
        })
    }

    #[doc(hidden)]
    pub(crate) fn create_command_pool(&self, queue_family_index: usize) -> VlResult<CommandPool> {
        let create_info = CommandPoolCreateInfo::builder()
            .queue_family_index(queue_family_index as u32)
            .flags(CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .build();
        match unsafe { self.inner.create_command_pool(&create_info, None) } {
            Ok(x) => Ok(x),
            Err(e) => {
                return Err(VlError::from(e));
            }
        }
    }

    #[doc(hidden)]
    pub(crate) fn allocate_command_buffer(&self, command_pool: CommandPool) -> VlResult<Vec<CommandBuffer>> {
        let create_info = CommandBufferAllocateInfo::builder()
            .command_pool(command_pool)
            .command_buffer_count(1)
            .level(CommandBufferLevel::PRIMARY)
            .build();
        match unsafe { self.inner.allocate_command_buffers(&create_info) } {
            Ok(x) => Ok(x),
            Err(e) => {
                return Err(VlError::from(e));
            }
        }
    }

    pub(crate) fn create_swapchain(
        &self,
        instance: &Instance,
        physical_device: PhysicalDevice,
        surface: &Surface,
    ) -> VlResult<Swapchain> {
        use ash::vk::{SurfaceCapabilitiesKHR, SwapchainCreateInfoKHR};

        let surface_capabilities = match unsafe {
            surface
                .surface
                .get_physical_device_surface_capabilities(physical_device.0, surface.surface_khr)
        } {
            Ok(c) => c,
            Err(e) => return Err(VlError::from(e)),
        };
        let surface_formats = match unsafe {
            surface
                .surface
                .get_physical_device_surface_formats(physical_device.0, surface.surface_khr)
        } {
            Ok(f) => f,
            Err(e) => return Err(VlError::from(e)),
        };
        let surface_present_modes = match unsafe {
            surface
                .surface
                .get_physical_device_surface_present_modes(physical_device.0, surface.surface_khr)
        } {
            Ok(m) => m,
            Err(e) => return Err(VlError::from(e)),
        };
        let format = surface_formats[0];
        let mode = surface_present_modes[0];
        let create_info = SwapchainCreateInfoKHR::builder()
            .surface(surface.surface_khr)
            .min_image_count(surface_capabilities.min_image_count + 1)
            .image_format(format.format)
            .image_color_space(format.color_space)
            .image_extent(surface_capabilities.current_extent)
            .image_array_layers(1)
            .image_usage(ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(SharingMode::EXCLUSIVE)
            .pre_transform(surface_capabilities.current_transform)
            .present_mode(mode)
            .clipped(true)
            .build();
        let inner = ash::extensions::khr::Swapchain::new(&instance.inner, &self.inner);
        let khr = match unsafe { inner.create_swapchain(&create_info, None) } {
            Ok(k) => k,
            Err(e) => return Err(VlError::from(e)),
        };
        Ok(Swapchain { inner, khr, format })
    }

    pub fn destroy_command_buffer(&self, buffer: &crate::target::CommandBuffer) {
        unsafe {
            self.inner.destroy_command_pool(buffer.command_pool, None);
        }
    }

    pub fn destroy_render_pass(&self, render_pass: &RenderPass) {
        unsafe {
            self.inner.destroy_render_pass(render_pass.inner, None);
        }
    }

    pub fn destroy_pipeline(&self, pipeline: &Pipeline) {
        unsafe {
            self.inner.destroy_pipeline(pipeline.inner, None);
        }
    }
}

impl Drop for LogicalDevice {
    fn drop(&mut self) {
        unsafe {
            self.inner.destroy_device(None);
        }
    }
}
