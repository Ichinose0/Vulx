use ash::vk::{DescriptorPool, DescriptorSet, DescriptorSetLayout, PipelineLayout};

use crate::{geometry::Buffer, Destroy, Pipeline};

pub struct StageBuilder {}

pub struct StageDescriptor {
    pub(crate) mvp: Buffer,
    pub(crate) desc_sets: Vec<DescriptorSet>,
    pub(crate) desc_pool: DescriptorPool,
    pub(crate) desc_layout: DescriptorSetLayout,
    pub(crate) pipeline_layout: PipelineLayout,
}

impl Destroy for StageDescriptor {
    fn destroy_with_instance(&self, instance: &crate::Instance) {}

    fn destroy_with_device(&self, device: &crate::LogicalDevice) {
        unsafe {
            device
                .inner
                .destroy_pipeline_layout(self.pipeline_layout, None);
            device.inner.destroy_descriptor_pool(self.desc_pool, None);
            device
                .inner
                .destroy_descriptor_set_layout(self.desc_layout, None);
        }
        device.destroy(&self.mvp)
    }
}

pub struct Stage {
    pipeline: Vec<Pipeline>,
}
