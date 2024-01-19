use libc::c_void;

use crate::{
    geometry::{Buffer, BufferUsage, Mvp},
    identity, Image, Instance, LogicalDevice, PhysicalDevice, RenderPass, Shader, ShaderKind,
    Spirv, StageDescriptor, VlResult, VlError,
};

pub enum VertexDataLayout {
    Vertex2Color3,
    Vertex3Color3,
    Vertex4Color3,
    Vertex2Color4,
    Vertex3Color4,
    Vertex4Color4,
}

pub struct PipelineBuilder<'a> {
    renderpass: Option<&'a RenderPass>,
    device: Option<&'a LogicalDevice>,
    shaders: Vec<Shader>,
    image: Option<&'a Image>,
    mvp: Option<Mvp>,
    width: u32,
    height: u32,
}

impl<'a> PipelineBuilder<'a> {
    pub fn logical_device(mut self, device: &'a LogicalDevice) -> Self {
        self.device = Some(device);
        self
    }
    pub fn render_pass(mut self, renderpass: &'a RenderPass) -> Self {
        self.renderpass = Some(renderpass);
        self
    }
    pub fn shaders(mut self, shaders: &'a [Shader]) -> Self {
        for x in shaders {
            self.shaders.push(*x);
        }
        self
    }
    pub fn image(mut self, image: &'a Image) -> Self {
        self.image = Some(image);
        self
    }
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }
    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn mvp(mut self, mvp: Mvp) -> Self {
        self.mvp = Some(mvp);
        self
    }

    pub fn build(
        mut self,
        instance: &Instance,
        physical_device: PhysicalDevice,
    ) -> VlResult<(Vec<Pipeline>, StageDescriptor)> {
        let renderpass = match self.renderpass {
            Some(x) => x,
            None => return Err(crate::VlError::MissingParameter("render_pass")),
        };
        let device = match self.device {
            Some(x) => x,
            None => return Err(crate::VlError::MissingParameter("logical_device")),
        };
        let image = match self.image {
            Some(x) => x,
            None => return Err(crate::VlError::MissingParameter("image")),
        };
        let mvp = match self.mvp {
            Some(x) => x,
            None => Mvp::new(identity(1.0), identity(1.0), identity(1.0)),
        };
        if self.shaders.is_empty() {
            let vertex = device
                .create_shader_module(Spirv::vertex_default(), ShaderKind::Vertex)
                .unwrap();
            let fragment = device
                .create_shader_module(Spirv::fragment_default(), ShaderKind::Fragment)
                .unwrap();
            self.shaders.push(vertex);
            self.shaders.push(fragment);
        }

        let buffer = Buffer::new(
            instance,
            physical_device,
            &device,
            std::mem::size_of::<Mvp>(),
            BufferUsage::Uniform,
        );
        buffer.allocate_data(
            vec![mvp.model, mvp.view, mvp.projection].as_ptr() as *const c_void,
            &device,
        );

        renderpass.create_pipeline(
            &image.inner,
            device,
            &self.shaders,
            buffer,
            self.width,
            self.height,
        )
    }
}

impl<'a> Default for PipelineBuilder<'a> {
    fn default() -> Self {
        Self {
            renderpass: None,
            device: None,
            shaders: vec![],
            image: None,
            width: 100,
            height: 100,
            mvp: None,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Pipeline {
    pub(crate) inner: ash::vk::Pipeline,
}

impl Pipeline {
    pub fn builder<'a>() -> PipelineBuilder<'a> {
        PipelineBuilder::default()
    }
}
