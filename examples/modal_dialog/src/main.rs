use native_dialog::{FileDialog, MessageDialog};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::EventLoop;
use winit::raw_window_handle::HasWindowHandle;
use winit::window::{Window, WindowAttributes};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = Application(None);
    event_loop.run_app(&mut app).unwrap();
}

struct Application(Option<Window>);
impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.0.is_some() {
            return;
        }

        let mut attributes = WindowAttributes::default();
        attributes.title = "A fantastic window!".to_string();
        attributes.inner_size = Some(winit::dpi::Size::Logical(LogicalSize::new(600f64, 400f64)));

        self.0 = event_loop.create_window(attributes).ok();
    }
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = self.0.as_ref() else {
            return;
        };

        let handle = window.window_handle().ok();

        match event {
            WindowEvent::CloseRequested if window_id == window.id() => {
                event_loop.exit();
            }
            WindowEvent::MouseInput {
                state: ElementState::Released,
                button: MouseButton::Right,
                ..
            } if window_id == window.id() => {
                let path = FileDialog::new()
                    .set_owner(handle.as_ref())
                    .show_open_single_file();

                let confirm = MessageDialog::new()
                    .set_title("Message")
                    .set_text(&format!("{:?}", path))
                    .set_owner(handle.as_ref())
                    .show_confirm();

                println!("{:?}", confirm);
            }
            _ => (),
        }
    }
}
