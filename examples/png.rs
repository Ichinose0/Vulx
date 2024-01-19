use Vulx::{
    geometry::{Mvp, PathGeometry},
    identity, look_at, perspective, radians,
    target::{CommandBuffer, PngRenderTarget, RenderTargetBuilder},
    translate, Color, ImageBuilder, InstanceBuilder, IntoPath, Mat4, Pipeline, PipelineBuilder,
    RenderPass, RenderTarget, ShaderKind, Spirv, SubPass, Vec2, Vec3, Vec4,
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
        .create_frame_buffer(&device, &render_pass, 640, 480)
        .unwrap();

    let fragment_shader = device
        .create_shader_module(Spirv::new("shader.frag.spv"), ShaderKind::Fragment)
        .unwrap();
    let vertex_shader = device
        .create_shader_module(Spirv::new("shader.vert.spv"), ShaderKind::Vertex)
        .unwrap();

    let projection = nalgebra_glm::perspective(640.0/480.0, 45.0*(180.0/std::f32::consts::PI), 0.1, 100.0);

    let view = nalgebra_glm::look_at(
        &Vec3::new(2.0,0.0,1.0),
        &Vec3::new(0.0,0.0,0.0),
        &Vec3::new(0.0,1.0,0.0)
    );
    
    let model = nalgebra_glm::identity();

    let mvp = Mvp::new(
        model,
        view,
        projection
    );

    let (pipeline, descriptor) = Pipeline::builder()
        .image(&image)
        .render_pass(&render_pass)
        .logical_device(&device)
        .shaders(&[fragment_shader, vertex_shader])
        .width(640)
        .height(480)
        .mvp(mvp)
        .build(&instance, physical_devices[suitable_device.unwrap()])
        .unwrap();

    let command_buffer = CommandBuffer::new(&device, queue_family_index);

    let mut triangle = PathGeometry::new();
    // triangle.triangle(
    //     Vec3::new(
    //         Vec4::new(0.0, -0.5, 0.0, 1.0),
    //         Vec4::new(0.5, 0.5, 0.0, 1.0),
    //         Vec4::new(-0.5, 0.5, 0.0, 1.0),
    //     ),
    //     Vec3::new(
    //         Vec4::new(1.0, 0.0, 0.0, 1.0),
    //         Vec4::new(0.0, 1.0, 0.0, 1.0),
    //         Vec4::new(0.0, 0.0, 1.0, 1.0),
    //     ),
    // );

    triangle.triangle(
        Vec3::new(
            Vec4::new(0.0, -1.0, 0.0, 1.0),
            Vec4::new(1.0, 1.0, 0.0, 1.0),
            Vec4::new(-1.0, 1.0, 0.0, 1.0),
        ),
        Vec3::new(
            Vec4::new(1.0, 0.0, 0.0, 1.0),
            Vec4::new(0.0, 1.0, 0.0, 1.0),
            Vec4::new(0.0, 0.0, 1.0, 1.0),
        ),
    );

    let mut render_target = RenderTargetBuilder::new()
        .instance(instance)
        .command_buffer(command_buffer)
        .logical_device(device)
        .physical_device(physical_devices[suitable_device.unwrap()])
        .image(Some(image))
        .pipeline(pipeline[0])
        .queue(queue)
        .renderpass(render_pass)
        .frame_buffer(frame_buffer)
        .descriptor(descriptor)
        .build_png("Example.png")
        .unwrap();

    render_target.begin();
    render_target.fill(&mut triangle, Color::RGBA(1.0, 0.0, 0.0, 1.0), 1.0);
    render_target.end();
    let device = render_target.logical_device();

    for i in &pipeline {
        device.destroy_pipeline(i);
    }
}
