use glfw::{Context, Glfw, PWindow, GlfwReceiver, WindowEvent};

use std::time::{Instant, Duration};
use std::thread::yield_now;

pub struct WindowManager {
    window: PWindow,
    target_fps: f64,
    last_frame: Instant,
    show_fps: bool,
}

impl WindowManager {
    pub fn new(window: PWindow) -> Self {
        WindowManager{
            window,
            target_fps: 60.0,
            last_frame: Instant::now(),
            show_fps: false,
        }
    }

    pub fn show_fps(&mut self, a: bool) {
        self.show_fps = a;
    }

    pub fn set_fps(&mut self, fps: f64) {
        self.target_fps = fps;
    }

    pub fn shutdown(&mut self) {
        self.window.set_should_close(true);
    }

    pub fn swap_buffers(&mut self) {
        let target = Duration::from_secs_f64(1.0 / self.target_fps);
        while Instant::now() - self.last_frame < target {
            yield_now();
        }
        if self.show_fps {
            println!("{:?}", 1.0 / (Instant::now() - self.last_frame).as_secs_f64());
            // todo!("Showing FPS is not implemented yet.")
        }
        self.last_frame = Instant::now();
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
