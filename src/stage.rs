use ash::vk::{DescriptorSet, PipelineLayout};

use crate::Pipeline;

pub struct StageBuilder {}

pub struct StageDescriptor {
    pub(crate) desc_sets: Vec<DescriptorSet>,
    pub(crate) pipeline_layout: PipelineLayout,
}

pub struct Stage {
    pipeline: Vec<Pipeline>,
}
