use glfw::{Context, Glfw, PWindow, GlfwReceiver, WindowEvent};

use crate::video::color::Color;

pub struct GlfwWindow {
    pub glfw: Glfw,
    pub window: PWindow,
    pub events: GlfwReceiver<(f64, WindowEvent)>,
}

impl Default for GlfwWindow {
    fn default() -> Self {
        Self::new(
            800, 600,
            Color::DARK_GRAY,
            "Window",
            true, true, true, true, true, true, true,
        )

    }
}

impl GlfwWindow {
    pub fn new(
        width: u32,
        height: u32,
        clear_color: Color,
        window_name: &str,
        show_cursor: bool,
        is_bordered: bool,
        is_resizable: bool,
        should_poll_keys: bool,
        should_poll_cursor_pos: bool,
        should_poll_mouse_buttons: bool,
        should_poll_scroll: bool
    ) -> Self {
        use glfw::fail_on_errors;
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();

        if is_resizable {
            glfw.window_hint(glfw::WindowHint::Resizable(true)); 
        }
        else {
            glfw.window_hint(glfw::WindowHint::Resizable(false));
        }
        if is_bordered {
            glfw.window_hint(glfw::WindowHint::Decorated(true));
        }
        else {
            glfw.window_hint(glfw::WindowHint::Decorated(false));
        }

        glfw.window_hint(glfw::WindowHint::ScaleToMonitor(true));
        glfw.window_hint(glfw::WindowHint::AlphaBits(Some(8)));

        // Create a windowed mode window and its OpenGL context
        let (mut window, events) = glfw.create_window(width, height, window_name, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        window.set_framebuffer_size_polling(true);
        window.set_framebuffer_size_callback(|_, width, height| {
            unsafe {
                gl::Viewport(0, 0, width, height);
            }
        });

        let (red, green, blue, alpha) = clear_color.to_tuple();
        let r = red as f32 / 255.0;
        let g = green as f32 / 255.0;
        let b = blue as f32 / 255.0;
        let a = alpha as f32 / 255.0;
        unsafe {
            gl::ClearColor(r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0), a.clamp(0.0, 1.0));
        }

        // Make the window's context current
        window.make_current();
        if should_poll_keys {window.set_key_polling(true)};
        if should_poll_scroll {window.set_scroll_polling(true)};
        if should_poll_cursor_pos {window.set_cursor_pos_polling(true)};
        if should_poll_mouse_buttons {window.set_mouse_button_polling(true)};

        if show_cursor {
            window.set_cursor_mode(glfw::CursorMode::Normal);
        }
        else {
            window.set_cursor_mode(glfw::CursorMode::Hidden);
        }

        GlfwWindow {
            glfw, 
            window, 
            events,
        }
    }
}
