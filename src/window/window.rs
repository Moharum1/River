use glfw::{fail_on_errors, Context};
use crate::window::screen_events::escape_pressed;

pub struct Window{
    title:String,
    width:u32,
    height:u32,
}

impl Window{

    pub fn new(title : &str, width : u32, height : u32) -> Window{
        Window {
            title : title.to_string(),
            width,
            height
        }
    }

    pub fn run(&self){

        let mut glfw = glfw::init(fail_on_errors!())
            .unwrap();

        // Use Modern OpenGL version
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 5));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));


        let (mut window, events) = glfw.create_window(
            self.width,
            self.height,
            self.title.as_str(),
            glfw::WindowMode::Windowed
        ).expect("Failed to create GLFW window.");

        // Load the OpenGL binaries
        gl::load_with(|s| window.get_proc_address(s) as *const _);

        window.make_current();
        window.set_key_polling(true);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.2, 1.0)
        }

        while !window.should_close() {
            window.swap_buffers();

            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events){
                println!("{:?}", event);

                if escape_pressed(&event){
                    window.set_should_close(true);
                }
            }

            // Clear screen
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
        }
    }
}