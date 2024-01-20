use std::{ffi::CString, mem::offset_of};

use ash::vk::{
    AttachmentDescription, AttachmentLoadOp, AttachmentReference, AttachmentStoreOp,
    ColorComponentFlags, CullModeFlags, DescriptorBufferInfo, DescriptorPoolCreateInfo,
    DescriptorPoolSize, DescriptorSetAllocateInfo, DescriptorSetLayoutBinding,
    DescriptorSetLayoutCreateInfo, DescriptorType, Extent2D, Format, FrontFace,
    GraphicsPipelineCreateInfo, ImageLayout, Offset2D, PipelineBindPoint, PipelineCache,
    PipelineColorBlendAttachmentState, PipelineColorBlendStateCreateInfo,
    PipelineInputAssemblyStateCreateInfo, PipelineLayoutCreateInfo,
    PipelineMultisampleStateCreateInfo, PipelineRasterizationStateCreateInfo,
    PipelineShaderStageCreateInfo, PipelineVertexInputStateCreateInfo,
    PipelineViewportStateCreateInfo, PolygonMode, PrimitiveTopology, Rect2D, RenderPassCreateInfo,
    SampleCountFlags, ShaderStageFlags, SubpassDescription, VertexInputAttributeDescription,
    VertexInputBindingDescription, VertexInputRate, Viewport, WriteDescriptorSet,
};

use crate::{
    geometry::{Buffer, Mvp, VertexData},
    Image, LogicalDevice, Pipeline, Shader, StageDescriptor, Vec2, VlError, VlResult,
};

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

pub struct RenderPassBuilder<'a> {
    device: Option<&'a LogicalDevice>,
    subpasses: &'a [SubPass],
}

impl<'a> RenderPassBuilder<'a> {
    pub fn logical_device(mut self, device: &'a LogicalDevice) -> Self {
        self.device = Some(device);
        self
    }

    pub fn subpasses(mut self, subpasses: &'a [SubPass]) -> Self {
        self.subpasses = subpasses;
        self
    }

    pub fn build(self) -> Result<RenderPass, ()> {
        let device = match self.device {
            Some(x) => x,
            None => return Err(()),
        };
        Ok(RenderPass::new(device, self.subpasses))
    }
}

impl<'a> Default for RenderPassBuilder<'a> {
    fn default() -> Self {
        Self {
            device: None,
            subpasses: &[],
        }
    }
}

pub struct RenderPass {
    pub(crate) inner: ash::vk::RenderPass,
}

impl RenderPass {
    pub fn new(device: &LogicalDevice, subpasses: &[SubPass]) -> Self {
        let attachment_descs = vec![AttachmentDescription::builder()
            .format(Format::R8G8B8A8_UNORM)
            .samples(SampleCountFlags::TYPE_1)
            .load_op(AttachmentLoadOp::CLEAR)
            .store_op(AttachmentStoreOp::STORE)
            .stencil_load_op(AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(AttachmentStoreOp::DONT_CARE)
            .initial_layout(ImageLayout::UNDEFINED)
            .final_layout(ImageLayout::PRESENT_SRC_KHR)
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
        image: &ash::vk::Image,
        device: &LogicalDevice,
        shaders: &[Shader],
        mvp: Buffer,
        width: u32,
        height: u32,
    ) -> VlResult<(Vec<Pipeline>, StageDescriptor)> {
        if shaders.is_empty() {
            return Err(VlError::MissingParameter("shaders"));
        }

        let desc_set_layout_bindings = vec![DescriptorSetLayoutBinding::builder()
            .binding(0)
            .descriptor_type(DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1)
            .stage_flags(ShaderStageFlags::VERTEX)
            .build()];
        let create_info = DescriptorSetLayoutCreateInfo::builder()
            .bindings(&desc_set_layout_bindings)
            .build();
        let desc_set_layout = match unsafe {
            device
                .inner
                .create_descriptor_set_layout(&create_info, None)
        } {
            Ok(x) => x,
            Err(e) => {
                return Err(VlError::from(e));
            }
        };

        let desc_pool_sizes = vec![DescriptorPoolSize::builder()
            .ty(DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1)
            .build()];
        let create_info = DescriptorPoolCreateInfo::builder()
            .pool_sizes(&desc_pool_sizes)
            .max_sets(1)
            .build();

        let desc_pool = match unsafe { device.inner.create_descriptor_pool(&create_info, None) } {
            Ok(x) => x,
            Err(e) => {
                return Err(VlError::from(e));
            }
        };

        let alloc_info = DescriptorSetAllocateInfo::builder()
            .descriptor_pool(desc_pool)
            .set_layouts(&[desc_set_layout])
            .build();

        let desc_sets = unsafe { device.inner.allocate_descriptor_sets(&alloc_info).unwrap() };

        let desc_buf_infos = vec![DescriptorBufferInfo::builder()
            .buffer(mvp.buffer)
            .offset(0)
            .range(std::mem::size_of::<Mvp>() as u64)
            .build()];
        let write_desc_set = WriteDescriptorSet::builder()
            .dst_set(desc_sets[0])
            .dst_binding(0)
            .dst_array_element(0)
            .descriptor_type(DescriptorType::UNIFORM_BUFFER)
            .buffer_info(&desc_buf_infos)
            .build();

        unsafe {
            device.inner.update_descriptor_sets(&[write_desc_set], &[]);
        }

        let vertex_binding_description = vec![VertexInputBindingDescription::builder()
            .binding(0)
            .stride(std::mem::size_of::<VertexData>() as u32)
            .input_rate(VertexInputRate::VERTEX)
            .build()];
        let vertex_input_description = vec![
            VertexInputAttributeDescription::builder()
                .binding(0)
                .location(0)
                .format(Format::R32G32_SFLOAT)
                .offset(offset_of!(VertexData, pos) as u32)
                .build(),
            VertexInputAttributeDescription::builder()
                .binding(0)
                .location(1)
                .format(Format::R32G32B32A32_SFLOAT)
                .offset(offset_of!(VertexData, color) as u32)
                .build(),
        ];

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
        let scissors = Rect2D::builder()
            .extent(Extent2D::builder().width(width).height(height).build())
            .offset(Offset2D::builder().x(0).y(0).build())
            .build();
        let viewport_state_info = PipelineViewportStateCreateInfo::builder()
            .viewports(&[Viewport::builder()
                .width(width as f32)
                .height(height as f32)
                .min_depth(0.0)
                .max_depth(1.0)
                .build()])
            .scissors(&[scissors])
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
        let layout_create_info = PipelineLayoutCreateInfo::builder()
            .set_layouts(&[desc_set_layout])
            .build();

        let pipeline_layout = match unsafe {
            device
                .inner
                .create_pipeline_layout(&layout_create_info, None)
        } {
            Ok(p) => p,
            Err(e) => {
                return Err(VlError::from(e));
            }
        };

        let vertex_input_info = PipelineVertexInputStateCreateInfo::builder()
            .vertex_binding_descriptions(&vertex_binding_description)
            .vertex_attribute_descriptions(&vertex_input_description)
            .build();

        let pipeline_create_info = GraphicsPipelineCreateInfo::builder()
            .viewport_state(&viewport_state_info)
            .vertex_input_state(&vertex_input_info)
            .input_assembly_state(&input_assembly)
            .rasterization_state(&rasterizer)
            .multisample_state(&multisample)
            .color_blend_state(&blend)
            .layout(pipeline_layout)
            .vertex_input_state(&vertex_input_info)
            .stages(&[])
            .render_pass(self.inner)
            .subpass(0)
            .stages(&shader_stages)
            .build();

        let pipeline = match unsafe {
            device.inner.create_graphics_pipelines(
                PipelineCache::null(),
                &[pipeline_create_info],
                None,
            )
        } {
            Ok(x) => x,
            Err(e) => {
                return Err(VlError::from(e.1));
            }
        };

        let mut pipelines = vec![];

        let stage_desc = StageDescriptor {
            mvp,
            desc_sets,
            desc_pool,
            desc_layout: desc_set_layout,
            pipeline_layout,
        };

        for i in pipeline {
            pipelines.push(Pipeline { inner: i });
        }

        Ok((pipelines, stage_desc))
    }
}
