use native_dialog::{FileDialog, MessageDialogBuilder};
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
                    let path = FileDialog::new()
                        .set_owner(&window)
                        .open_single_file()
                        .show();

                    let confirm = MessageDialogBuilder::new()
                        .set_title("Message")
                        .set_text(&format!("{:?}", path))
                        .set_owner(&window)
                        .confirm()
                        .show();

                    println!("{:?}", confirm);
                }
                _ => (),
            },
            _ => (),
        }
    })
}
