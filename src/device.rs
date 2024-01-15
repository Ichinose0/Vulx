use crate::Queue;

pub struct LogicalDevice {
    pub(crate) inner: ash::Device
}

impl LogicalDevice {
    pub fn get_queue(&self,queue_family_index: usize) -> Queue {
        Queue(unsafe { self.inner.get_device_queue(queue_family_index as u32, 0) })
    }
}