use Vulx::{InstanceBuilder, target::PngRenderTarget, RenderTarget};

fn main() {
    let instance = InstanceBuilder::new().build();
    let physical_devices = instance.enumerate_physical_device();
    let mut suitable_device = None;
    let mut queue_family_index = 0;
    for (n,i) in physical_devices.iter().enumerate() {
        let props = instance.get_queue_properties(*i);
        for (n,i) in props.iter().enumerate() {
            let graphic = i.is_graphic_support();
            let compute = i.is_compute_support();
            let transfer = i.is_transfer_support();
            println!("---- Queue {} ----",n+1);
            println!("Graphic support: {}",graphic);
            println!("Compute support: {}",compute);
            println!("Transfer support: {}",transfer);
            if graphic && compute && transfer {
                suitable_device = Some(n);
                queue_family_index = n;
                break;
            }
        }
    }
    if suitable_device.is_none() {
        panic!("No physical device available");
    }

    let device = instance.create_logical_device(physical_devices[suitable_device.unwrap()],queue_family_index);
    let queue = device.get_queue(queue_family_index);

    let render_target = PngRenderTarget::new(instance,device,queue,queue_family_index);

    render_target.begin();
    render_target.end();
}