use crate::{
    Destroy, HardwareError, LogicalDevice, PhysicalDevice, QueueProperties, VlError, VlResult,
};
use ash::{
    vk::{DeviceCreateInfo, DeviceQueueCreateInfo, InstanceCreateInfo},
    Entry,
};

pub struct InstanceBuilder<'a> {
    entry: ash::Entry,
    name: &'a str,
    targets: Vec<InstanceTarget>,
}

impl<'a> InstanceBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: &'a str) -> Self {
        self.name = name;
        self
    }

    pub fn targets(mut self, targets: Vec<InstanceTarget>) -> Self {
        self.targets = targets;
        self
    }

    pub fn build(self) -> VlResult<Instance> {
        let mut exts = vec![];
        for i in self.targets {
            match i {
                InstanceTarget::Image => {}
                InstanceTarget::Window => {
                    #[cfg(target_os = "windows")]
                    {
                        exts.push(ash::extensions::khr::Surface::name().as_ptr());
                        exts.push(ash::extensions::khr::Win32Surface::name().as_ptr());
                    }
                }
            }
        }

        let create_info = InstanceCreateInfo::builder()
            .enabled_extension_names(&exts)
            .build();

        let inner = match unsafe { self.entry.create_instance(&create_info, None) } {
            Ok(x) => x,
            Err(e) => return Err(VlError::from(e)),
        };

        Ok(Instance {
            inner,
            entry: self.entry,
        })
    }
}

impl<'a> Default for InstanceBuilder<'a> {
    fn default() -> Self {
        let entry = Entry::linked();
        Self {
            entry,
            name: "",
            targets: vec![],
        }
    }
}

pub enum InstanceTarget {
    Image,
    Window,
}

/// # Instance
/// Instances that are central to the creation and processing of various objects such as LogicalDevice.
/// 
/// ## Example
/// ```
/// use vulx::InstanceBuilder;
/// 
/// fn main() {
///     let instance = InstanceBuilder::new().build().unwrap();
///     let mut queue_family_index = 0;
///     // Get physical device!
///     let physical_device = instance
///         .default_physical_device(&mut queue_family_index)
///         .unwrap();
/// }
/// ```
pub struct Instance {
    pub(crate) inner: ash::Instance,
    pub(crate) entry: Entry,
}

impl Instance {
    /// Enumerates available physical devices and returns them as Vec type.
    pub fn enumerate_physical_device(&self) -> Vec<PhysicalDevice> {
        let mut devices = vec![];
        let vk_devices = unsafe { self.inner.enumerate_physical_devices().unwrap() };
        for i in vk_devices {
            devices.push(PhysicalDevice(i));
        }
        devices
    }

    /// Returns the physical device considered optimal.
    /// The physical device returned by this method may not be the one you want!
    pub fn default_physical_device(
        &self,
        queue_family_index: &mut usize,
    ) -> VlResult<PhysicalDevice> {
        let devices = self.enumerate_physical_device();
        let mut index = 0;
        let mut found_suitable_device = false;

        for (_, i) in devices.iter().enumerate() {
            let props = self.get_queue_properties(*i);
            for (n, i) in props.iter().enumerate() {
                let graphic = i.is_graphic_support();
                let compute = i.is_compute_support();
                let transfer = i.is_transfer_support();
                if graphic && compute && transfer {
                    index = n;
                    found_suitable_device = true;
                    *queue_family_index = n;
                    break;
                }
            }
        }

        if !found_suitable_device {
            return Err(VlError::HardwareError(HardwareError::NoSuitableDevice));
        }

        Ok(devices[index])
    }

    /// Get the version of Vulkan.
    pub fn version(&self) -> Option<String> {
        match self.entry.try_enumerate_instance_version() {
            Ok(v) => {
                match v {
                    Some(v) => {
                        let major = ash::vk::api_version_major(v);
                        let minor = ash::vk::api_version_minor(v);
                        let patch = ash::vk::api_version_patch(v);
                        Some(format!("{}.{}.{}",major,minor,patch))
                    }
                    None => return None
                }
            },
            Err(_) => return None
        }
    }

    /// Gets the queue properties of the physical device.  
    /// The length of this Vec type is equivalent to the length of the Vec of the physical device obtained by enumerate_physica_device.
    pub fn get_queue_properties(&self, device: PhysicalDevice) -> Vec<QueueProperties> {
        let mut prop = vec![];
        let props = unsafe {
            self.inner
                .get_physical_device_queue_family_properties(device.0)
        };
        for i in props {
            prop.push(QueueProperties(i));
        }
        prop
    }

    /// Create a logical device.
    // This requires a valid physical device and QueueFamilyIndex.
    pub fn create_logical_device(
        &self,
        device: PhysicalDevice,
        queue_family_index: usize,
    ) -> LogicalDevice {
        let queue_infos = vec![DeviceQueueCreateInfo::builder()
            .queue_family_index(queue_family_index as u32)
            .queue_priorities(&[1.0])
            .build()];
        let create_info = DeviceCreateInfo::builder()
            .queue_create_infos(&queue_infos)
            .enabled_extension_names(&[ash::extensions::khr::Swapchain::name().as_ptr()])
            .build();
        let inner = unsafe { self.inner.create_device(device.0, &create_info, None) }.unwrap();
        LogicalDevice { inner }
    }

    pub fn destroy<D>(&self, object: &D)
    where
        D: Destroy,
    {
        object.destroy_with_instance(&self);
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe { self.inner.destroy_instance(None) }
    }
}
