use native_dialog::{FileDialog, MessageDialog};
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, MouseButton, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(LogicalSize::new(600f64, 400f64))
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        #[allow(clippy::single_match)]
        match event {
            Event::WindowEvent { window_id, event } => match event {
                WindowEvent::CloseRequested if window_id == window.id() => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::MouseInput {
                    state: ElementState::Released,
                    button: MouseButton::Right,
                    ..
                } => {
                    let path = FileDialog::new().set_owner(&window).show_open_single_file();

                    let confirm = MessageDialog::new()
                        .set_title("Message")
                        .set_text(&format!("{:?}", path))
                        .set_owner(&window)
                        .show_confirm();

                    println!("{:?}", confirm);
                }
                _ => (),
            },
            _ => (),
        }
    })
}
