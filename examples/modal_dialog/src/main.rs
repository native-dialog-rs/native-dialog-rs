use native_dialog::{FileDialog, MessageDialog};
use winit::event::{ElementState, Event, MouseButton, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(600.0, 400.0))
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::MouseInput {
                    state: ElementState::Pressed,
                    button: MouseButton::Left,
                    ..
                } => {
                    let path = FileDialog::new().set_owner(&window).show_open_single_file();

                    MessageDialog::new()
                        .set_title("Message")
                        .set_text(&format!("{:?}", path))
                        .set_owner(&window)
                        .show_alert()
                        .unwrap();
                }
                _ => (),
            },
            _ => (),
        }
    })
}
