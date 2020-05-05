use euclid::default::Size2D;
use surfman::Connection;
use surfman::{declare_surfman, SurfaceAccess, SurfaceType};
use surfman::{ContextAttributeFlags, ContextAttributes, GLVersion};
use winit::dpi::PhysicalSize;
use winit::{DeviceEvent, Event, EventsLoop, KeyboardInput, VirtualKeyCode};
use winit::{WindowBuilder, WindowEvent};

declare_surfman!();

fn main() {
    let mut event_loop = EventsLoop::new();
    let dpi = event_loop.get_primary_monitor().get_hidpi_factor();
    let window_size = Size2D::new(1024, 768);
    let logical_size =
        PhysicalSize::new(window_size.width as f64, window_size.height as f64).to_logical(dpi);
    let window = WindowBuilder::new()
        .with_title("SSCCE")
        .with_dimensions(logical_size)
        .build(&event_loop)
        .unwrap();
    window.show();

    let connection = Connection::from_winit_window(&window).unwrap();
    let native_widget = connection
        .create_native_widget_from_winit_window(&window)
        .unwrap();
    let adapter = connection.create_low_power_adapter().unwrap();
    let mut device = connection.create_device(&adapter).unwrap();

    let context_attributes = ContextAttributes {
        version: GLVersion::new(3, 0),
        flags: ContextAttributeFlags::ALPHA,
    };
    let context_descriptor = device
        .create_context_descriptor(&context_attributes)
        .unwrap();

    let surface_type = SurfaceType::Widget { native_widget };
    let mut context = device.create_context(&context_descriptor).unwrap();
    let surface = device
        .create_surface(&context, SurfaceAccess::GPUOnly, surface_type)
        .unwrap();
    device
        .bind_surface_to_context(&mut context, surface)
        .unwrap();

    gl::load_with(|symbol_name| device.get_proc_address(&context, symbol_name));

    let mut exit = false;

    while !exit {
        unsafe {
            gl::ClearColor(0.0, 0.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        let mut surface = device
            .unbind_surface_from_context(&mut context)
            .unwrap()
            .unwrap();

        device.present_surface(&mut context, &mut surface).unwrap();
        device
            .bind_surface_to_context(&mut context, surface)
            .unwrap();

        event_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            }
            | Event::DeviceEvent {
                event:
                    DeviceEvent::Key(KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    }),
                ..
            } => exit = true,
            _ => {}
        });
    }
}
