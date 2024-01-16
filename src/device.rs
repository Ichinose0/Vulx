use ash::vk::{
    CommandBuffer, CommandBufferAllocateInfo, CommandBufferLevel, CommandPool,
    CommandPoolCreateInfo, DeviceQueueCreateInfo, ShaderModuleCreateInfo,
};

use crate::{Queue, Shader, ShaderKind, Spirv};

/// Represents a logical device.
pub struct LogicalDevice {
    pub(crate) inner: ash::Device,
}

impl LogicalDevice {
    /// Get the queue from the index number
    pub fn get_queue(&self, queue_family_index: usize) -> Queue {
        Queue(unsafe { self.inner.get_device_queue(queue_family_index as u32, 0) })
    }

    pub fn create_shader_module(&self, spirv: Spirv, kind: ShaderKind) -> Result<Shader, ()> {
        let shader_create_info = ShaderModuleCreateInfo::builder().code(&spirv.data).build();
        let shader = match unsafe { self.inner.create_shader_module(&shader_create_info, None) } {
            Ok(s) => s,
            Err(_) => panic!("Err"),
        };
        Ok(Shader {
            inner: shader,
            kind,
        })
    }

    #[doc(hidden)]
    pub(crate) fn create_command_pool(&self, queue_family_index: usize) -> CommandPool {
        let create_info = CommandPoolCreateInfo::builder()
            .queue_family_index(queue_family_index as u32)
            .build();
        unsafe { self.inner.create_command_pool(&create_info, None) }.unwrap()
    }

    #[doc(hidden)]
    pub(crate) fn allocate_command_buffer(&self, command_pool: CommandPool) -> Vec<CommandBuffer> {
        let create_info = CommandBufferAllocateInfo::builder()
            .command_pool(command_pool)
            .command_buffer_count(1)
            .level(CommandBufferLevel::PRIMARY)
            .build();
        unsafe { self.inner.allocate_command_buffers(&create_info) }.unwrap()
    }
}
