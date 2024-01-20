use std::ffi::c_void;

use ash::vk::{
    ComponentMapping, ComponentSwizzle, DeviceMemory, Extent3D, Format, FramebufferCreateInfo,
    ImageAspectFlags, ImageCreateInfo, ImageLayout, ImageSubresourceRange, ImageTiling,
    ImageUsageFlags, ImageViewCreateInfo, MemoryAllocateInfo, MemoryMapFlags, MemoryPropertyFlags,
    Rect2D, SampleCountFlags, SharingMode, Viewport,
};

use crate::{Destroy, Instance, LogicalDevice, PhysicalDevice, RenderPass};

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
            if ((mem_req.memory_type_bits & (1 << i)) != 0
                && (mem_prop.memory_types[i as usize].property_flags
                    & MemoryPropertyFlags::HOST_VISIBLE)
                    .as_raw()
                    != 0)
            {
                create_info = create_info.memory_type_index(i);
                suitable_memory_found = true;
                break;
            }
        }

        if !suitable_memory_found {
            panic!("No memory available");
        }

        let memory;
        unsafe {
            memory = device.inner.allocate_memory(&create_info, None).unwrap();
            device.inner.bind_image_memory(inner, memory, 0).unwrap();
        }

        Image {
            inner,
            memory,
            mem_size: mem_req.size,
        }
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

#[derive(Clone, Copy)]
pub struct Image {
    pub inner: ash::vk::Image,
    pub(crate) memory: DeviceMemory,
    pub(crate) mem_size: u64,
}

impl Image {
    /// Create an image.
    ///
    /// # Arguments
    ///
    /// * `device` - Valid LogicalDevice
    pub fn create_image_view(&self, device: &LogicalDevice) -> Result<ImageView, ()> {
        let create_info = ImageViewCreateInfo::builder()
            .image(self.inner)
            .view_type(ash::vk::ImageViewType::TYPE_2D)
            .format(Format::R8G8B8A8_UNORM)
            .components(
                ComponentMapping::builder()
                    .a(ComponentSwizzle::IDENTITY)
                    .r(ComponentSwizzle::IDENTITY)
                    .g(ComponentSwizzle::IDENTITY)
                    .b(ComponentSwizzle::IDENTITY)
                    .build(),
            )
            .subresource_range(
                ImageSubresourceRange::builder()
                    .aspect_mask(ImageAspectFlags::COLOR)
                    .base_mip_level(0)
                    .level_count(1)
                    .base_array_layer(0)
                    .layer_count(1)
                    .build(),
            )
            .build();
        let inner = match unsafe { device.inner.create_image_view(&create_info, None) } {
            Ok(i) => i,
            Err(_) => return Err(()),
        };
        Ok(ImageView { inner })
    }

    pub fn map_memory(&self, device: &LogicalDevice) -> *mut c_void {
        unsafe {
            device
                .inner
                .map_memory(self.memory, 0, self.mem_size, MemoryMapFlags::empty())
                .unwrap()
        }
    }
}

impl From<ash::vk::Image> for Image {
    fn from(value: ash::vk::Image) -> Self {
        Self {
            inner: value,
            memory: DeviceMemory::null(),
            mem_size: 0,
        }
    }
}

impl Destroy for Image {
    fn destroy_with_instance(&self, instance: &Instance) {}

    fn destroy_with_device(&self, device: &LogicalDevice) {
        unsafe {
            device.inner.destroy_image(self.inner, None);
            device.inner.free_memory(self.memory, None);
        }
    }
}

pub struct ImageView {
    pub(crate) inner: ash::vk::ImageView,
}

impl ImageView {
    pub fn create_frame_buffer(
        &self,
        device: &LogicalDevice,
        render_pass: &RenderPass,
        width: u32,
        height: u32,
    ) -> Result<FrameBuffer, ()> {
        let create_info = FramebufferCreateInfo::builder()
            .width(width)
            .height(height)
            .layers(1)
            .render_pass(render_pass.inner)
            .attachments(&[self.inner])
            .build();
        let inner = match unsafe { device.inner.create_framebuffer(&create_info, None) } {
            Ok(f) => f,
            Err(_) => return Err(()),
        };
        Ok(FrameBuffer { inner })
    }
}

impl Destroy for ImageView {
    fn destroy_with_instance(&self, instance: &Instance) {}

    fn destroy_with_device(&self, device: &LogicalDevice) {
        unsafe {
            device.inner.destroy_image_view(self.inner, None);
        }
    }
}

pub struct FrameBuffer {
    pub(crate) inner: ash::vk::Framebuffer,
}
