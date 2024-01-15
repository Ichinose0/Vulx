use std::ffi::CString;

use ash::vk::{PipelineCache, SubpassDescription, PipelineBindPoint, ImageLayout, AttachmentReference, AttachmentDescription, Format, SampleCountFlags, AttachmentLoadOp, AttachmentStoreOp, RenderPassCreateInfo, ShaderStageFlags, PipelineShaderStageCreateInfo, PipelineViewportStateCreateInfo, Rect2D, Extent2D, Offset2D, PipelineVertexInputStateCreateInfo, PipelineInputAssemblyStateCreateInfo, PrimitiveTopology, PipelineRasterizationStateCreateInfo, PolygonMode, CullModeFlags, FrontFace, PipelineMultisampleStateCreateInfo, PipelineColorBlendAttachmentState, ColorComponentFlags, PipelineColorBlendStateCreateInfo, PipelineLayoutCreateInfo, GraphicsPipelineCreateInfo};

use crate::{LogicalDevice, Image, Pipeline, Shader};

pub struct SubPass(SubpassDescription);

impl SubPass {
    pub fn new() -> Self {
        let subpass_attachment = vec![AttachmentReference::builder()
            .attachment(0)
            .layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .build()];
        let subpass = SubpassDescription::builder()
            .pipeline_bind_point(PipelineBindPoint::GRAPHICS)
            .color_attachments(&subpass_attachment)
            .build();
        Self { 0: subpass }
    }
}

impl Default for SubPass {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub struct RenderPass {
    pub(crate) inner: ash::vk::RenderPass,
}

impl RenderPass {
    pub fn new(device: &LogicalDevice,subpasses: &[SubPass]) -> Self {
        let attachment_descs = vec![AttachmentDescription::builder()
            .format(Format::R8G8B8A8_UNORM)
            .samples(SampleCountFlags::TYPE_1)
            .load_op(AttachmentLoadOp::DONT_CARE)
            .store_op(AttachmentStoreOp::STORE)
            .stencil_load_op(AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(AttachmentStoreOp::DONT_CARE)
            .initial_layout(ImageLayout::UNDEFINED)
            .final_layout(ImageLayout::GENERAL)
            .build()];

        let mut subpass = vec![];
        for i in subpasses {
            subpass.push(i.0);
        }

        let create_info = RenderPassCreateInfo::builder()
            .attachments(&attachment_descs)
            .subpasses(&subpass)
            .dependencies(&[])
            .build();
        let inner = unsafe { device.inner.create_render_pass(&create_info, None) }.unwrap();
        Self { inner }
    }
    pub fn create_pipeline(
        &self,
        image: &Image,
        device: &LogicalDevice,
        shaders: &[Shader],
    ) -> Result<Vec<Pipeline>, ()> {
        if shaders.is_empty() {
            return Err(());
        }
        let mut shader_stages = vec![];
        let entry = CString::new("main").unwrap();
        for i in shaders {
            let flag = match i.kind {
                crate::ShaderKind::Vertex => ShaderStageFlags::VERTEX,
                crate::ShaderKind::Fragment => ShaderStageFlags::FRAGMENT,
            };
            shader_stages.push(
                PipelineShaderStageCreateInfo::builder()
                    .module(i.inner)
                    .name(entry.as_c_str())
                    .stage(flag)
                    .build(),
            );
        }
        let scissors = Rect2D::builder().extent(Extent2D::builder().width(image.viewport.width as u32).height(image.viewport.height as u32).build()).offset(Offset2D::builder().x(0).y(0).build()).build();
        let viewport_state_info = PipelineViewportStateCreateInfo::builder()
            .viewports(&[image.viewport])
            .scissors(&[scissors])
            .build();
        let vertex_input_info = PipelineVertexInputStateCreateInfo::builder()
            .vertex_attribute_descriptions(&[])
            .vertex_binding_descriptions(&[])
            .build();
        let input_assembly = PipelineInputAssemblyStateCreateInfo::builder()
            .topology(PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false)
            .build();
        let rasterizer = PipelineRasterizationStateCreateInfo::builder()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(CullModeFlags::BACK)
            .front_face(FrontFace::CLOCKWISE)
            .depth_bias_enable(false)
            .build();
        let multisample = PipelineMultisampleStateCreateInfo::builder()
            .sample_shading_enable(false)
            .rasterization_samples(SampleCountFlags::TYPE_1)
            .build();
        let blend_attachment = vec![PipelineColorBlendAttachmentState::builder()
            .color_write_mask(
                ColorComponentFlags::A
                    | ColorComponentFlags::R
                    | ColorComponentFlags::G
                    | ColorComponentFlags::B,
            )
            .blend_enable(false)
            .build()];
        let blend = PipelineColorBlendStateCreateInfo::builder()
            .logic_op_enable(false)
            .attachments(&blend_attachment)
            .build();
        let layout_create_info = PipelineLayoutCreateInfo::builder().set_layouts(&[]).build();

        let pipeline_layout = match unsafe {
            device
                .inner
                .create_pipeline_layout(&layout_create_info, None)
        } {
            Ok(p) => p,
            Err(e) => {
                return Err(());
            }
        };

        let pipeline_create_info = GraphicsPipelineCreateInfo::builder()
            .viewport_state(&viewport_state_info)
            .vertex_input_state(&vertex_input_info)
            .input_assembly_state(&input_assembly)
            .rasterization_state(&rasterizer)
            .multisample_state(&multisample)
            .color_blend_state(&blend)
            .layout(pipeline_layout)
            .stages(&[])
            .render_pass(self.inner)
            .subpass(0)
            .stages(&shader_stages)
            .build();

        let pipeline = unsafe {
            device.inner.create_graphics_pipelines(
                PipelineCache::null(),
                &[pipeline_create_info],
                None,
            )
        }.unwrap();

        let mut pipelines = vec![];

        for i in pipeline {
            pipelines.push(Pipeline { inner: i });
        }

        Ok(pipelines)
    }
}
