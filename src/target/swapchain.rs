use ash::vk::{
    ComponentMapping, ComponentSwizzle, ImageAspectFlags, ImageSubresourceRange,
    ImageViewCreateInfo, ImageViewType, SurfaceFormatKHR, SwapchainKHR,
};

use crate::{Device, GMResult, ImageView};

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
    pub fn get_image(&self, device: &Device) -> Result<Vec<ImageView>, GMResult> {
        let images = match unsafe { self.inner.get_swapchain_images(self.khr) } {
            Ok(i) => i,
            Err(_) => panic!("Err"),
        };

        let mut image_views = vec![];
        for image in images {
            let create_info = ImageViewCreateInfo::builder()
                .image(image)
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
