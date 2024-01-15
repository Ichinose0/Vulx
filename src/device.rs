use ash::vk::{
    CommandBuffer, CommandBufferAllocateInfo, CommandBufferLevel, CommandPool,
    CommandPoolCreateInfo, DeviceQueueCreateInfo,
};

use crate::Queue;

pub struct LogicalDevice {
    pub(crate) inner: ash::Device,
}

impl LogicalDevice {
    pub fn get_queue(&self, queue_family_index: usize) -> Queue {
        Queue(unsafe { self.inner.get_device_queue(queue_family_index as u32, 0) })
    }

    pub(crate) fn create_command_pool(&self, queue_family_index: usize) -> CommandPool {
        let create_info = CommandPoolCreateInfo::builder()
            .queue_family_index(queue_family_index as u32)
            .build();
        unsafe { self.inner.create_command_pool(&create_info, None) }.unwrap()
    }

    pub(crate) fn allocate_command_buffer(&self, command_pool: CommandPool) -> Vec<CommandBuffer> {
        let create_info = CommandBufferAllocateInfo::builder()
            .command_pool(command_pool)
            .command_buffer_count(1)
            .level(CommandBufferLevel::PRIMARY)
            .build();
        unsafe { self.inner.allocate_command_buffers(&create_info) }.unwrap()
    }
}
