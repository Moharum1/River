use glfw::{Action, Key, WindowEvent};

// use winit::event::{ElementState, KeyEvent};
// use winit::keyboard::{KeyCode, PhysicalKey};
//
pub fn escape_pressed(event: &WindowEvent) -> bool {
    matches!(
        event,
        WindowEvent::Key(Key::Escape, _, Action::Press, _)
    )
}