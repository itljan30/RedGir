use glfw::{Context, Glfw, PWindow, GlfwReceiver, WindowEvent};

pub struct WindowManager {
    window: PWindow,
}

impl WindowManager {
    pub fn new(window: PWindow) -> Self {
        WindowManager{
            window,
        }
    }

    pub fn shutdown(&mut self) {
        self.window.set_should_close(true);
    }

    pub fn update(&mut self) {
        self.window.swap_buffers();
    }

    pub fn is_running(&self) -> bool {
        return !self.window.should_close();
    }

    pub fn glfw_init() -> (Glfw, PWindow, GlfwReceiver<(f64, WindowEvent)>) {
        use glfw::fail_on_errors;
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();

        // Create a windowed mode window and its OpenGL context
        let (mut window, events) = glfw.create_window(800, 600, "Hello this is window", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        // Make the window's context current
        window.make_current();
        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.set_scroll_polling(true);
        window.set_cursor_pos_polling(true);

        return (glfw, window, events);
    }
}
