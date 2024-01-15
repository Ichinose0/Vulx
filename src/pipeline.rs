#[derive(Clone, Copy)]
pub struct Pipeline {
    pub(crate) inner: ash::vk::Pipeline,
}
