use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder, raw_window_handle::HasWindowHandle,
};
use Vulx::{
    geometry::PathGeometry,
    target::{CommandBuffer, PngRenderTarget, RenderTargetBuilder},
    ImageBuilder, InstanceBuilder, IntoPath, RenderPass, RenderTarget, ShaderKind, Spirv, SubPass,
    Vec2, Vec3, InstanceTarget,
};

fn main() {
    let instance = InstanceBuilder::new().targets(vec![InstanceTarget::Window]).build();
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

    

    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_title("Vulx")
        .with_inner_size(winit::dpi::LogicalSize::new(640.0, 480.0))
        .build(&event_loop)
        .unwrap();

        let window_handle = window.window_handle().unwrap();

        let mut render_target = match window_handle.as_raw() {
            winit::raw_window_handle::RawWindowHandle::Win32(handle) => {
                RenderTargetBuilder::new()
            .instance(instance)
            .command_buffer(command_buffer)
            .logical_device(device)
            .physical_device(physical_devices[suitable_device.unwrap()])
            .image(Some(image))
            .pipeline(pipeline[0])
            .queue(queue)
            .renderpass(render_pass)
            .frame_buffer(frame_buffer)
            .build_hwnd(isize::from(handle.hwnd),0)
            .unwrap()
            },
            _ => todo!(),
        };

        

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::RedrawRequested => {
                    // Notify the windowing system that we'll be presenting to the window.

                    render_target.begin();
                    render_target.end();
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
