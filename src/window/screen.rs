use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes, WindowId};
use crate::window::screen_events::escape_pressed;

#[derive(Debug)]
pub struct Screen {
    window_attributes: WindowAttributes,
    window: Option<Window>,
}

impl Screen {
    pub fn new(title : &str) -> Screen{
        Screen {
            window_attributes: Window::default_attributes()
                .with_title(title),
            window: None,
        }
    }

    pub fn get_window(&mut self) -> &mut Window {
        self.window.as_mut().unwrap()
    }
}
impl ApplicationHandler for Screen {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(self.window_attributes.clone()).unwrap();
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        match &event {
            WindowEvent::CloseRequested => event_loop.exit(),


            WindowEvent::KeyboardInput {
                event, ..
            } => {
                if escape_pressed(event) {
                    event_loop.exit()
                }
            },

            _ => {}
        }
    }
}