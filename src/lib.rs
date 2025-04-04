pub mod window;

use crate::window::screen::Screen;
use winit::event_loop::EventLoop;

pub async fn run(){
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let mut screen = Screen::new("WGPU");
    event_loop
        .run_app(&mut screen)
        .unwrap()
}







