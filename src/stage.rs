use ash::vk::{DescriptorPool, DescriptorSet, DescriptorSetLayout, PipelineLayout};
use libc::c_void;
use nalgebra_glm::Mat3;

use crate::{
    geometry::{Buffer, BufferUsage, Mvp},
    Destroy, Instance, LogicalDevice, Mat4, PhysicalDevice, Vec3, VlError, VlResult,
};

pub struct StageBuilder<'a> {
    instance: Option<&'a Instance>,
    device: Option<&'a LogicalDevice>,
    physical_device: Option<PhysicalDevice>,

    camera: Camera,
    mode: StageMode,
    width: u32,
    height: u32,
}

impl<'a> StageBuilder<'a> {
    pub fn instance(mut self, instance: &'a Instance) -> Self {
        self.instance = Some(instance);
        self
    }
    pub fn logical_device(mut self, logical_device: &'a LogicalDevice) -> Self {
        self.device = Some(logical_device);
        self
    }
    pub fn physical_device(mut self, physical_device: PhysicalDevice) -> Self {
        self.physical_device = Some(physical_device);
        self
    }
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self.camera.width(width as f32);
        self
    }
    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self.camera.height(height as f32);
        self
    }
    pub fn mode(mut self, mode: StageMode) -> Self {
        self.mode = mode;
        self
    }
    pub fn build(self) -> VlResult<Stage> {
        let instance = match self.instance {
            Some(x) => x,
            None => return Err(VlError::MissingParameter("instance")),
        };
        let device = match self.device {
            Some(x) => x,
            None => return Err(VlError::MissingParameter("instance")),
        };
        let physical_device = match self.physical_device {
            Some(x) => x,
            None => return Err(VlError::MissingParameter("instance")),
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

        let mvp = self.camera.mvp(projection);

        let mut buffer = Buffer::new(
            instance,
            physical_device,
            device,
            std::mem::size_of::<Mvp>(),
            BufferUsage::Uniform,
        );
        buffer.allocate_data(
            vec![mvp.model, mvp.view, mvp.projection].as_ptr() as *const c_void,
            device,
        );

        Ok(Stage {
            camera: self.camera,
            width: self.width,
            height: self.height,
            buffer,
            mvp,
            descriptor: None,
        })
    }
}

pub enum StageMode {
    Ortho,
    Projection,
}

pub struct Stage {
    pub(crate) camera: Camera,

    pub(crate) width: u32,
    pub(crate) height: u32,

    pub(crate) mvp: Mvp,
    pub(crate) buffer: Buffer,

    pub(crate) descriptor: Option<StageDescriptor>,
}

impl Stage {
    pub fn builder<'a>() -> StageBuilder<'a> {
        StageBuilder {
            instance: None,
            device: None,
            physical_device: None,
            camera: Camera::default(),
            mode: StageMode::Ortho,
            width: 100,
            height: 100,
        }
    }
}

pub struct StageDescriptor {
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
    }
}

pub struct Angle(f32, f32, f32);

impl Default for Angle {
    fn default() -> Self {
        Self(0.0, 0.0, 1.0)
    }
}

impl Angle {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z)
    }
}

pub struct Camera {
    fov: f32,
    width: f32,
    height: f32,
    angle: Angle,
}

impl Camera {
    pub fn new(fov: f32, width: f32, height: f32, angle: Angle) -> Self {
        let fov = fov * (180.0 / std::f32::consts::PI);
        Self {
            fov,
            width,
            height,
            angle,
        }
    }

    pub(crate) fn mvp(&self, projection: Mat4<f32>) -> Mvp {
        let view = nalgebra_glm::look_at(
            &Vec3::new(self.angle.0, self.angle.1, self.angle.2),
            &Vec3::new(0.0, 0.0, 0.0),
            &Vec3::new(0.0, 1.0, 0.0),
        );
        Mvp::new(nalgebra_glm::identity(), view, projection)
    }

    pub(crate) fn width(&mut self, width: f32) {
        self.width = width;
    }

    pub(crate) fn height(&mut self, height: f32) {
        self.height = height;
    }

    pub fn angle(&mut self, angle: Angle) {
        self.angle = angle;
    }
}

impl Default for Camera {
    fn default() -> Self {
        let fov = 45.0 * (180.0 / std::f32::consts::PI);
        Self {
            angle: Default::default(),
            fov: fov,
            width: 100.0,
            height: 100.0,
        }
    }
}
