use Vulx::{
    geometry::PathGeometry,
    target::{CommandBuffer, PngRenderTarget, RenderTargetBuilder},
    ImageBuilder, InstanceBuilder, IntoPath, RenderPass, RenderTarget, ShaderKind, Spirv, SubPass,
    Vec2, Vec3,
};

fn main() {
    let instance = InstanceBuilder::new().build();
    let physical_devices = instance.enumerate_physical_device();
    let mut suitable_device = None;
    let mut queue_family_index = 0;
    for (n, i) in physical_devices.iter().enumerate() {
        let props = instance.get_queue_properties(*i);
        for (n, i) in props.iter().enumerate() {
            let graphic = i.is_graphic_support();
            let compute = i.is_compute_support();
            let transfer = i.is_transfer_support();
            println!("---- Queue {} ----", n + 1);
            println!("Graphic support: {}", graphic);
            println!("Compute support: {}", compute);
            println!("Transfer support: {}", transfer);
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

    let device = instance.create_logical_device(
        physical_devices[suitable_device.unwrap()],
        queue_family_index,
    );
    let queue = device.get_queue(queue_family_index);

    let image = ImageBuilder::new().width(640).height(480).build(
        &instance,
        physical_devices[suitable_device.unwrap()],
        &device,
    );
    let image_view = image.create_image_view(&device).unwrap();

    let subpasses = vec![SubPass::new()];

    let render_pass = RenderPass::new(&device, &subpasses);

    let frame_buffer = image_view
        .create_frame_buffer(&device, &render_pass, &image)
        .unwrap();

    let fragment_shader = device
        .create_shader_module(
            Spirv::new("examples/shader/shader.frag.spv"),
            ShaderKind::Fragment,
        )
        .unwrap();
    let vertex_shader = device
        .create_shader_module(
            Spirv::new("examples/shader/shader.vert.spv"),
            ShaderKind::Vertex,
        )
        .unwrap();

    let pipeline = render_pass
        .create_pipeline(&image, &device, &[fragment_shader, vertex_shader])
        .unwrap();

    let command_buffer = CommandBuffer::new(&device, queue_family_index);

    let render_target = RenderTargetBuilder::new()
        .instance(instance)
        .command_buffer(command_buffer)
        .logical_device(device)
        .physical_device(physical_devices[suitable_device.unwrap()])
        .image(Some(image))
        .pipeline(pipeline[0])
        .queue(queue)
        .renderpass(render_pass)
        .frame_buffer(frame_buffer)
        .build_png("triangle.png")
        .unwrap();

    render_target.begin();
    render_target.end();
}
