pub mod window;

use std::rc::Rc;
use winit::event_loop::EventLoop;
use crate::window::screen::Screen;
use crate::window::screen_state::WindowState;

pub async fn run(){
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let mut screen = Screen::new("WGPU");
    event_loop.run_app(&mut screen).unwrap()
}







