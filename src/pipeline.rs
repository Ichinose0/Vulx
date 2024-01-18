use crate::{RenderPass,LogicalDevice,Shader,Image};

pub enum VertexDataLayout {
    Vertex2Color3
    Vertex3Color3
    Vertex4Color3,
    Vertex2Color4
    Vertex3Color4
    Vertex4Color4
}

pub struct PipelineBuilder<'a> {
    renderpass: Option<&'a RenderPass>,
    device: Option<&'a LogicalDevice>,
    shaders: Option<&'a[Shader]>,
    image: Option<Image>,
    width: u32,
    height: u32
}

impl<'a> PipelineBuilder<'a> {
    pub fn logical_device(mut self,device: &'a LogicalDevice) -> Self {
        self.device = Some(device);
        self
    }
    pub fn render_pass(mut self,renderpass: &'a RenderPass) -> Self {
        self.renderpass = Some(renderpass);
        self
    }
    pub fn shaders(mut self,shaders: &'a [Shader]) -> Self {
        self.shaders = Some(shaders);
        self
    }
    pub fn width(mut self,width: u32) -> Self {
        self.width = width;
        self
    }
    pub fn height(mut self,height: u32) -> Self {
        self.height = height;
        self
    }
    pub fn build(mut self) -> Result<Vec<Pipeline>, ()> {
        let renderpass = match self.renderpass {
            Some(x) => x,
            None => return Err(())
        }
        let device = match self.device {
            Some(x) => x,
            None => return Err(())
        }
        let image = match self.image {
            Some(x) => x,
            None => return Err(())
        }
        if self.shaders.is_none() {
            self.shaders = Some(&[Shader::vertex_default(),Shader::fragment_default()]);
        }

        renderpass.create_pipeline(&image,&device,&self.shaders.unwrap(),self.width,self.height)
    }
}

impl<'a> Default for PipelineBuilder<'a> {
    fn default() -> Self {
        Self {
            renderpass: None,
            device: None,
            shaders: None,
            image: None,
            width: 100,
            height: 100
        }
    }
}

#[derive(Clone, Copy)]
pub struct Pipeline {
    pub(crate) inner: ash::vk::Pipeline,
}

