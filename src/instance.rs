use std::ffi::{c_char, CStr};

use crate::{LogicalDevice, PhysicalDevice, QueueProperties};
use ash::{
    vk::{DeviceCreateInfo, DeviceQueueCreateInfo, InstanceCreateInfo, InstanceCreateInfoBuilder},
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

    pub fn build(self) -> Instance {
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
        let inner = unsafe { self.entry.create_instance(&create_info, None).unwrap() };
        Instance {
            inner,
            entry: self.entry,
        }
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

pub struct Instance {
    pub(crate) inner: ash::Instance,
    pub(crate) entry: Entry,
}

impl Instance {
    pub fn enumerate_physical_device(&self) -> Vec<PhysicalDevice> {
        let mut devices = vec![];
        let vk_devices = unsafe { self.inner.enumerate_physical_devices().unwrap() };
        for i in vk_devices {
            devices.push(PhysicalDevice(i));
        }
        devices
    }

    pub fn default_physical_device(&self,queue_family_index: &mut usize) -> PhysicalDevice {
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
            panic!("No suitable physical device found");
        }
        devices[index]
    }

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
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe { self.inner.destroy_instance(None) }
    }
}
