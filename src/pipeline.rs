use crate::{
    Image, Instance, LogicalDevice, PhysicalDevice, RenderPass, Shader, ShaderKind, Spirv, Stage,
    VlResult,
};

pub enum VertexDataLayout {
    Vertex2Color3,
    Vertex3Color3,
    Vertex4Color3,
    Vertex2Color4,
    Vertex3Color4,
    Vertex4Color4,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum PolygonMode {
    #[default]
    Fill,
    Line,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum PrimitiveTopology {
    #[default]
    TriangleList,
    TriangleStrip,
    TriangleFan,
}

impl Into<ash::vk::PrimitiveTopology> for PrimitiveTopology {
    fn into(self) -> ash::vk::PrimitiveTopology {
        match self {
            PrimitiveTopology::TriangleFan => ash::vk::PrimitiveTopology::TRIANGLE_FAN,
            PrimitiveTopology::TriangleList => ash::vk::PrimitiveTopology::TRIANGLE_LIST,
            PrimitiveTopology::TriangleStrip => ash::vk::PrimitiveTopology::TRIANGLE_STRIP,
        }
    }
}

impl Into<ash::vk::PolygonMode> for PolygonMode {
    fn into(self) -> ash::vk::PolygonMode {
        match self {
            PolygonMode::Fill => ash::vk::PolygonMode::FILL,
            PolygonMode::Line => ash::vk::PolygonMode::LINE,
        }
    }
}

pub struct PipelineBuilder<'a> {
    renderpass: Option<&'a RenderPass>,
    device: Option<&'a LogicalDevice>,
    shaders: Vec<Shader>,
    image: Option<&'a Image>,
    stage: Option<&'a mut Stage>,
    mode: PolygonMode,
    topology: PrimitiveTopology,
    line_width: f32,
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
    pub fn mode(mut self, mode: PolygonMode) -> Self {
        self.mode = mode;
        self
    }
    pub fn topology(mut self, topology: PrimitiveTopology) -> Self {
        self.topology = topology;
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

    pub fn line_width(mut self, line_width: f32) -> Self {
        self.line_width = line_width;
        self
    }

    pub fn stage(mut self, stage: &'a mut Stage) -> Self {
        self.stage = Some(stage);
        self
    }

    pub fn build(
        mut self,
        instance: &Instance,
        physical_device: PhysicalDevice,
    ) -> VlResult<Vec<Pipeline>> {
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
        let stage = match self.stage {
            Some(x) => x,
            None => return Err(crate::VlError::MissingParameter("stage")),
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

        renderpass.create_pipeline(
            device,
            &self.shaders,
            stage,
            self.mode.into(),
            self.topology.into(),
            self.width,
            self.height,
            self.line_width,
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
            stage: None,
            mode: Default::default(),
            topology: Default::default(),
            width: 100,
            height: 100,
            line_width: 1.0,
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
