use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    raw_window_handle::HasWindowHandle,
    window::WindowBuilder,
};
use Vulx::{
    geometry::PathGeometry,
    target::{CommandBuffer, PngRenderTarget, RenderTargetBuilder},
    Color, ImageBuilder, InstanceBuilder, InstanceTarget, IntoPath, RenderPass, RenderTarget,
    ShaderKind, Spirv, SubPass, Vec2, Vec3, Vec4,
};

#[cfg(target_os = "windows")]
fn main() {
    use Vulx::{geometry::Mvp, Pipeline};

    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_title("Vulx")
        .with_inner_size(winit::dpi::LogicalSize::new(640.0, 480.0))
        .build(&event_loop)
        .unwrap();
    let win_size = window.inner_size();
    let instance = InstanceBuilder::new()
        .targets(vec![InstanceTarget::Window])
        .build()
        .unwrap();
    let mut queue_family_index = 0;
    let physical_device = instance
        .default_physical_device(&mut queue_family_index)
        .unwrap();

    let device = instance.create_logical_device(physical_device, queue_family_index);
    let queue = device.get_queue(queue_family_index);

    let image = ImageBuilder::new()
        .width(win_size.width)
        .height(win_size.height)
        .build(&instance, physical_device, &device);
    let image_view = image.create_image_view(&device).unwrap();

    let subpasses = vec![SubPass::new()];

    let render_pass = RenderPass::new(&device, &subpasses);

    let frame_buffer = image_view
        .create_frame_buffer(&device, &render_pass, win_size.width, win_size.height)
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
        win_size.width as f32 / win_size.height as f32,
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
        .width(win_size.width)
        .height(win_size.height)
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

    let window_handle = window.window_handle().unwrap();

    let mut render_target = match window_handle.as_raw() {
        winit::raw_window_handle::RawWindowHandle::Win32(handle) => RenderTargetBuilder::new()
            .instance(instance)
            .command_buffer(command_buffer)
            .logical_device(device)
            .physical_device(physical_device)
            .image(Some(image))
            .queue(queue)
            .build_hwnd(isize::from(handle.hwnd), 0, win_size.width, win_size.height)
            .unwrap(),
        _ => todo!(),
    };

    window.pre_present_notify();

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::RedrawRequested => {
                    // Notify the windowing system that we'll be presenting to the window.

                    render_target.begin();
                    render_target.fill(&mut triangle, Color::RGBA(1.0, 0.0, 0.0, 1.0), 1.0);
                    render_target.end();
                }
                WindowEvent::Resized(size) => {
                    window.pre_present_notify();
                }
                _ => (),
            },

            Event::AboutToWait => {
                window.request_redraw();
            }

            _ => (),
        }
    });
}

#[cfg(not(target_os = "windows"))]
fn main() {
    panic!("This example is supported only windows.")
}
