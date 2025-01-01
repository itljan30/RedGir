use glfw::{Context, Glfw, PWindow, GlfwReceiver, WindowEvent};
use gl;

use crate::video::sprite::{Sprite, SpriteId};
use crate::video::color::Color;

use std::time::{Instant, Duration};
use std::thread::yield_now;
use std::collections::HashMap;

pub struct GlfwWindow {
    pub glfw: Glfw,
    pub window: PWindow,
    pub events: GlfwReceiver<(f64, WindowEvent)>,
}

impl Default for GlfwWindow {
    fn default() -> Self {
        Self::new(
            800,
            600,
            Color::DARK_GRAY,
            "Window",
            true,
            true,
            true,
            true,
            true,
            true,
            true,
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

        window.set_framebuffer_size_polling(true);
        window.set_framebuffer_size_callback(|_, width, height| {
            unsafe {
                gl::Viewport(0, 0, width as i32, height as i32);
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
            events
        }
    }
}

pub struct WindowManager {
    window: PWindow,
    target_fps: f64,
    last_frame: Instant,
    show_fps: bool,
    sprites: HashMap<SpriteId, Sprite>,
    last_sprite_id: u64,
}

impl WindowManager {
    pub fn new(window: PWindow) -> Self {
        WindowManager{
            window,
            target_fps: 60.0,
            last_frame: Instant::now(),
            show_fps: false,
            sprites: HashMap::new(),
            last_sprite_id: 0
        }
    }


    pub fn draw_point(&mut self, x: f64, y: f64) {
    }

    pub fn get_sprite(&mut self, id: SpriteId) -> Option<&mut Sprite> {
        self.sprites.get_mut(&id)
    }

    pub fn get_all_sprites(&self) -> &HashMap<SpriteId, Sprite> {
        &self.sprites
    }

    pub fn add_sprite(&mut self, mut sprite: Sprite) -> SpriteId {
        sprite.set_id(self.last_sprite_id);
        self.last_sprite_id += 1;
        let sprite_id = sprite.get_id();
        self.sprites.insert(sprite_id.clone(), sprite);
        sprite_id
    }

    pub fn remove_sprite(&mut self, sprite_id: &SpriteId) {
        self.sprites.remove(sprite_id);
    }

    pub fn toggle_fullscreen(&mut self) {
        if !self.window.is_maximized() {
            self.window.maximize();
        }
        else {
            self.window.restore();
        }
    }

    pub fn set_window_size(&mut self, width: i32, height: i32) {
        self.window.set_size(width, height);
    }

    pub fn toggle_show_fps(&mut self) {
        match self.show_fps {
            true => self.show_fps = false,
            false => self.show_fps = true,
        }
    }

    pub fn set_fps(&mut self, fps: f64) {
        self.target_fps = fps;
    }

    pub fn shutdown(&mut self) {
        self.window.set_should_close(true);
    }

    pub fn swap_buffers(&mut self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        let target = Duration::from_secs_f64(1.0 / self.target_fps);
        while Instant::now() - self.last_frame < target {
            yield_now();
        }
        if self.show_fps {
            println!("{:?}", 1.0 / (Instant::now() - self.last_frame).as_secs_f64());
        }
        self.last_frame = Instant::now();
        self.window.swap_buffers();
    }

    // BUG? I probably need a &mut self in order to call drawing functions on self.window
    fn draw_sprite(&self, sprite: &Sprite) {
        
    }

    pub fn draw_frame(&mut self) {
        for sprite in self.sprites.values() {
            self.draw_sprite(sprite);
        }

        self.swap_buffers();
    }

    pub fn is_running(&self) -> bool {
        return !self.window.should_close();
    }
}
