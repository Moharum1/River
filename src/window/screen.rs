use std::sync::Arc;
use crate::window::screen_events::escape_pressed;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes, WindowId};
use crate::window::screen_state::WindowState;

pub struct Screen<'window> {
    window_attributes: WindowAttributes,
    window: Option<Arc<Window>>,
    state: Option<WindowState<'window>>
}

impl<'window> Screen<'window> {
    pub fn new(title : &str) -> Screen{
        Screen {
            window_attributes: Window::default_attributes()
                .with_title(title),
            window: None,
            state : None
        }
    }

}
impl<'window> ApplicationHandler for Screen<'window> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(event_loop.create_window(self.window_attributes.clone()).unwrap());
        self.window = Some(window.clone());
        self.state = Some(WindowState::new(window.clone()));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
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