use ash::{Entry, vk::{InstanceCreateInfoBuilder, InstanceCreateInfo}};

pub struct InstanceBuilder<'a> {
    entry: ash::Entry,
    name: &'a str,
    targets: Vec<InstanceTarget>
}

impl<'a> InstanceBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self,name: &'a str) -> Self {
        self.name = name;
        self
    }

    pub fn targets(mut self,targets: Vec<InstanceTarget>) -> Self {
        self.targets = targets;
        self
    }

    pub fn build(self) -> Instance {
        let info = InstanceCreateInfo::builder().build();
        let inner = unsafe { self.entry.create_instance(&create_info, None).unwrap() };
        Instance {
            inner,
            entry: self.entry
        }
    }
}

impl<'a> Default for InstanceBuilder<'a> {
    fn default() -> Self {
        let entry = Entry::linked();
        Self {
            entry,
            name: "",
            targets: vec![]
        }
    }
}

pub enum InstanceTarget {
    Image,
    Window
}

pub struct Instance{
    inner: ash::Instance,
    entry: Entry
}

impl Instance {

}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe { self.inner.destroy_instance(None) }
    }
}