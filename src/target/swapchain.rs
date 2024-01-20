use ash::vk::{
    ComponentMapping, ComponentSwizzle, Image, ImageAspectFlags, ImageSubresourceRange,
    ImageUsageFlags, ImageViewCreateInfo, ImageViewType, SharingMode, SurfaceCapabilitiesKHR,
    SurfaceFormatKHR, SwapchainKHR,
};

use crate::{ImageView, Instance, LogicalDevice, PhysicalDevice};

use super::surface::Surface;

pub struct AcquireImageResult {
    pub state: SwapchainState,
    pub buffer_index: usize,
}

pub enum SwapchainState {
    Validity,
    Invalid,
}

pub struct Swapchain {
    pub(crate) inner: ash::extensions::khr::Swapchain,
    pub(crate) khr: SwapchainKHR,
    pub(crate) format: SurfaceFormatKHR,
}

impl Swapchain {
    pub(crate) fn create_swapchain(
        instance: &Instance,
        device: &LogicalDevice,
        physical_device: PhysicalDevice,
        surface: &Surface,
    ) -> Result<(Self, SurfaceCapabilitiesKHR), ()> {
        use ash::vk::SwapchainCreateInfoKHR;

        let surface_capabilities = match unsafe {
            surface
                .surface
                .get_physical_device_surface_capabilities(physical_device.0, surface.surface_khr)
        } {
            Ok(c) => c,
            Err(_) => panic!("Err"),
        };
        let surface_formats = match unsafe {
            surface
                .surface
                .get_physical_device_surface_formats(physical_device.0, surface.surface_khr)
        } {
            Ok(f) => f,
            Err(_) => panic!("Err"),
        };
        let surface_present_modes = match unsafe {
            surface
                .surface
                .get_physical_device_surface_present_modes(physical_device.0, surface.surface_khr)
        } {
            Ok(m) => m,
            Err(_) => panic!("Err"),
        };
        let format = surface_formats[0];
        let mode = surface_present_modes[0];
        let create_info = SwapchainCreateInfoKHR::builder()
            .surface(surface.surface_khr)
            .min_image_count(surface_capabilities.min_image_count + 1)
            .image_format(format.format)
            .image_color_space(format.color_space)
            .image_extent(surface_capabilities.current_extent)
            .image_array_layers(1)
            .image_usage(ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(SharingMode::EXCLUSIVE)
            .pre_transform(surface_capabilities.current_transform)
            .present_mode(mode)
            .clipped(true)
            .build();
        let inner = ash::extensions::khr::Swapchain::new(&instance.inner, &device.inner);

        let khr = match unsafe { inner.create_swapchain(&create_info, None) } {
            Ok(k) => k,
            Err(e) => panic!("{:?}", e),
        };
        Ok((Self { inner, khr, format }, surface_capabilities))
    }

    pub fn get_image(
        &self,
        device: &LogicalDevice,
        images: &[Image],
    ) -> Result<Vec<ImageView>, ()> {
        let mut image_views = vec![];
        for image in images {
            let create_info = ImageViewCreateInfo::builder()
                .image(*image)
                .view_type(ImageViewType::TYPE_2D)
                .format(self.format.format)
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
            match unsafe { device.inner.create_image_view(&create_info, None) } {
                Ok(inner) => {
                    image_views.push(ImageView { inner });
                }
                Err(_) => panic!("Err"),
            }
        }

        Ok(image_views)
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe { self.inner.destroy_swapchain(self.khr, None) };
    }
}

pub fn recreate_swapchain(
    instance: &Instance,
    device: &LogicalDevice,
    physical_device: PhysicalDevice,
    surface: &Surface,
) -> (Swapchain, SurfaceCapabilitiesKHR) {
    Swapchain::create_swapchain(instance, device, physical_device, surface).unwrap()
}
