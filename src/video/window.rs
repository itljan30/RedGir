use glfw::{Context, Glfw, PWindow, GlfwReceiver, WindowEvent};

use crate::video::sprite::{Sprite, SpriteId};

use std::time::{Instant, Duration};
use std::thread::yield_now;
use std::collections::HashMap;

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

    pub fn get_sprite(&mut self, id: SpriteId) -> Option<&mut Sprite> {
        self.sprites.get_mut(&id)
    }

    pub fn get_sprites_ids(&self) -> &HashMap<SpriteId, Sprite> {
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

    pub fn draw_frame(&mut self) {
        // run through self.sprites and draw them
        self.swap_buffers();
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
