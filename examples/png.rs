use Vulx::{
    geometry::{Mvp, PathGeometry},
    target::{CommandBuffer, RenderTargetBuilder},
    Color, ImageBuilder, InstanceBuilder, Pipeline, PipelineBuilder, RenderPass, RenderTarget,
    ShaderKind, Spirv, SubPass, Vec3, Vec4,
};

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn main() {
    let instance = InstanceBuilder::new().build().unwrap();
    let mut queue_family_index = 0;
    let physical_device = instance
        .default_physical_device(&mut queue_family_index)
        .unwrap();

    let device = instance.create_logical_device(physical_device, queue_family_index);
    let queue = device.get_queue(queue_family_index);

    let image =
        ImageBuilder::new()
            .width(WIDTH)
            .height(HEIGHT)
            .build(&instance, physical_device, &device);
    let image_view = image.create_image_view(&device).unwrap();

    let subpasses = vec![SubPass::new()];

    let render_pass = RenderPass::new(&device, &subpasses);

    let frame_buffer = image_view
        .create_frame_buffer(&device, &render_pass, WIDTH,HEIGHT)
        .unwrap();

    let fragment_shader = device
        .create_shader_module(
            Spirv::new(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/examples/shader/shader.frag.spv"
            )),
            ShaderKind::Fragment,
        )
        .unwrap();
    let vertex_shader = device
        .create_shader_module(
            Spirv::new(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/examples/shader/shader.vert.spv"
            )),
            ShaderKind::Vertex,
        )
        .unwrap();

    let projection = nalgebra_glm::perspective(
        WIDTH as f32 / HEIGHT as f32,
        45.0 * (180.0 / std::f32::consts::PI),
        0.1,
        100.0,
    );

    let view = nalgebra_glm::look_at(
        &Vec3::new(2.0, 0.0, 1.0),
        &Vec3::new(0.0, 0.0, 0.0),
        &Vec3::new(0.0, 1.0, 0.0),
    );

    let model = nalgebra_glm::identity();

    let mvp = Mvp::new(model, view, projection);

    let (pipeline, descriptor) = Pipeline::builder()
        .image(&image)
        .render_pass(&render_pass)
        .logical_device(&device)
        .shaders(&[fragment_shader, vertex_shader])
        .width(WIDTH)
        .height(HEIGHT)
        .mvp(mvp)
        .build(&instance, physical_device)
        .unwrap();

    let command_buffer = CommandBuffer::new(&device, queue_family_index).unwrap();

    let mut triangle = PathGeometry::new();

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
        .physical_device(physical_device)
        .image(Some(image))
        .pipeline(pipeline[0])
        .queue(queue)
        .renderpass(render_pass)
        .frame_buffer(frame_buffer)
        .descriptor(descriptor)
        .build_png("Example.png",WIDTH,HEIGHT)
        .unwrap();

    render_target.begin();
    render_target.fill(&mut triangle, Color::RGBA(1.0, 0.0, 0.0, 1.0), 1.0);
    render_target.end();
    let device = render_target.logical_device();

    for i in &pipeline {
        device.destroy_pipeline(i);
    }
    device.destroy(&fragment_shader);
    device.destroy(&vertex_shader);
    device.destroy(&image_view);
}
