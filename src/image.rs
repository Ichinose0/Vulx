use ash::vk::{
    Extent3D, Format, ImageCreateInfo, ImageLayout, ImageTiling, ImageUsageFlags,
    MemoryAllocateInfo, SampleCountFlags, SharingMode, Viewport, Rect2D,
};

use crate::{Instance, LogicalDevice, PhysicalDevice};

#[allow(non_camel_case_types)]
pub enum ImageType {
    e3D,
    e2D,
}

impl Into<ash::vk::ImageType> for ImageType {
    fn into(self) -> ash::vk::ImageType {
        match self {
            ImageType::e2D => ash::vk::ImageType::TYPE_2D,
            ImageType::e3D => ash::vk::ImageType::TYPE_3D,
        }
    }
}

pub struct ImageBuilder {
    width: u32,
    height: u32,
    image_type: ImageType,
}

impl ImageBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn build(
        mut self,
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &LogicalDevice,
    ) -> Image {
        let create_info = ImageCreateInfo::builder()
            .image_type(self.image_type.into())
            .extent(
                Extent3D::builder()
                    .width(self.width)
                    .height(self.height)
                    .depth(1)
                    .build(),
            )
            .mip_levels(1)
            .array_layers(1)
            .format(Format::R8G8B8A8_UNORM)
            .tiling(ImageTiling::LINEAR)
            .initial_layout(ImageLayout::UNDEFINED)
            .usage(ImageUsageFlags::COLOR_ATTACHMENT)
            .sharing_mode(SharingMode::EXCLUSIVE)
            .samples(SampleCountFlags::TYPE_1)
            .build();
        let inner = unsafe { device.inner.create_image(&create_info, None).unwrap() };

        let mem_prop = unsafe {
            instance
                .inner
                .get_physical_device_memory_properties(physical_device.0)
        };

        let mem_req = unsafe { device.inner.get_image_memory_requirements(inner) };
        let mut create_info = MemoryAllocateInfo::builder().allocation_size(mem_req.size);

        let mut suitable_memory_found = false;

        for i in 0..mem_prop.memory_type_count {
            if (mem_req.memory_type_bits & (1 << i)) != 0 {
                create_info = create_info.memory_type_index(i);
                suitable_memory_found = true;
                break;
            }
        }

        if !suitable_memory_found {
            panic!("No memory available");
        }

        unsafe {
            let memory = device.inner.allocate_memory(&create_info, None).unwrap();
            device.inner.bind_image_memory(inner, memory, 0);
        }

        let viewport = Viewport::builder().width(self.width as f32).height(self.height as f32).min_depth(0.0).max_depth(1.0).x(0.0).y(0.0).build();

        Image { inner,viewport }
    }
}

impl Default for ImageBuilder {
    fn default() -> Self {
        Self {
            width: 100,
            height: 100,
            image_type: ImageType::e2D,
        }
    }
}

#[derive(Clone,Copy)]
pub struct Image {
    pub(crate) inner: ash::vk::Image,
    pub(crate) viewport: Viewport,
}

pub struct ImageView {
    pub(crate) inner: ash::vk::ImageView,
}

impl ImageView {
    pub fn create_frame_buffer(
        &self,
        device: &Device,
        render_pass: &RenderPass,
        width: u32,
        height: u32,
    ) -> Result<FrameBuffer, GMResult> {
        let create_info = FramebufferCreateInfo::builder()
            .width(width)
            .height(height)
            .layers(1)
            .render_pass(render_pass.inner)
            .attachments(&[self.inner])
            .build();
        let inner = match unsafe { device.inner.create_framebuffer(&create_info, None) } {
            Ok(f) => f,
            Err(_) => return Err(GMResult::UnknownError),
        };
        Ok(FrameBuffer { inner })
    }
}

pub struct FrameBuffer {
    pub(crate) inner: ash::vk::Framebuffer,
}
