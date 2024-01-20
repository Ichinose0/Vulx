use ash::vk::{DescriptorPool, DescriptorSet, DescriptorSetLayout, PipelineLayout};
use libc::c_void;

use crate::{
    geometry::{Buffer, Mvp, BufferUsage},
    Destroy, Instance, LogicalDevice, Mat4, PhysicalDevice, Pipeline, VlError, VlResult,
};

pub struct StageBuilder<'a> {
    instance: Option<&'a Instance>,
    device: Option<&'a LogicalDevice>,
    physical_device: Option<PhysicalDevice>,

    pipeline: Vec<Pipeline>,
    camera: Option<Camera>,
    mode: StageMode,
    width: u32,
    height: u32,
}

impl<'a> StageBuilder<'a> {
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }
    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }
    pub fn build(self) -> VlResult<Stage> {
        let instance = match self.instance {
            Some(x) => x,
            None => return Err(VlError::MissingParameter("instance"))
        };
        let device = match self.device {
            Some(x) => x,
            None => return Err(VlError::MissingParameter("instance"))
        };
        let physical_device = match self.physical_device {
            Some(x) => x,
            None => return Err(VlError::MissingParameter("instance"))
        };
        let projection = match self.mode {
            StageMode::Ortho => {
                nalgebra_glm::ortho(0.0, self.width as f32, 0.0, self.height as f32, -1.0, 1.0)
            }
            StageMode::Projection => nalgebra_glm::perspective(
                self.width as f32 / self.height as f32,
                45.0 * (180.0 / std::f32::consts::PI),
                0.1,
                100.0,
            ),
        };

        let mvp = Mvp::new(
            nalgebra_glm::identity(),
            nalgebra_glm::identity(),
            projection,
        );

        let mut buffer = Buffer::new(
            instance,
            physical_device,
            device,
            std::mem::size_of::<Mvp>(),
            BufferUsage::Uniform,
        );
        buffer.allocate_data(
            vec![mvp.model, mvp.view, mvp.projection].as_ptr() as *const c_void,
            &device,
        );

        Ok(Stage {
            pipeline: self.pipeline,
            camera: self.camera,
            width: self.width,
            height: self.height,
            buffer,
            mvp,
            descriptor: None
        })
    }
}

pub enum StageMode {
    Ortho,
    Projection,
}

pub struct Stage {
    pub(crate) pipeline: Vec<Pipeline>,
    pub(crate) camera: Option<Camera>,

    pub(crate) width: u32,
    pub(crate) height: u32,

    pub(crate) mvp: Mvp,
    pub(crate) buffer: Buffer,

    pub(crate) descriptor: Option<StageDescriptor>
}

impl Stage {
    pub fn builder<'a>() -> StageBuilder<'a> {
        StageBuilder {
            instance: None,
            device: None,
            physical_device: None,
            pipeline: vec![],
            camera: None,
            mode: StageMode::Ortho,
            width: 100,
            height: 100,
        }
    }
}

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

pub struct Angle(f32, f32, f32);

impl Angle {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z)
    }
}

pub struct Camera {
    angle: Angle,
}

impl Camera {
    pub fn new(angle: Angle) -> Self {
        Self { angle }
    }
}
