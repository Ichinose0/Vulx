use ash::vk::{QueueFamilyProperties, QueueFlags};

pub struct Queue(pub(crate) ash::vk::Queue);

pub struct QueueProperties(pub(crate) QueueFamilyProperties);

impl QueueProperties {
    pub fn count(&self) -> usize {
        self.0.queue_count as usize
    }

    pub fn is_graphic_support(&self) -> bool {
        (self.0.queue_flags & QueueFlags::GRAPHICS).as_raw() != 0
    }

    pub fn is_compute_support(&self) -> bool {
        (self.0.queue_flags & QueueFlags::COMPUTE).as_raw() != 0
    }

    pub fn is_transfer_support(&self) -> bool {
        (self.0.queue_flags & QueueFlags::TRANSFER).as_raw() != 0
    }
}