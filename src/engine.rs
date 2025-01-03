use crate::audio::audio_manager::AudioManager;
use crate::input::input_manager::{InputManager, Key, Action};
use crate::video::window::WindowManager;
use crate::video::color::Color;
use crate::video::sprite::{SpriteSheetId, Sprite, SpriteId, ImageType};
use crate::video::shader_manager::{ShaderId, FragmentShader, VertexShader};
use crate::video::glfw_window::GlfwWindow;

use std::collections::HashMap;

pub struct Engine {
    audio_manager: AudioManager,
    input_manager: InputManager,
    window: WindowManager,
}

impl Default for Engine {
    fn default() -> Self {
        let window = GlfwWindow::default();

        Engine {
            audio_manager: AudioManager::new(),
            input_manager: InputManager::new(window.glfw, window.events),
            window: WindowManager::new(window.window),
        }
    }
}

impl Engine {
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
        should_poll_scroll: bool,
    ) -> Self {
        let window = GlfwWindow::new(
            width,
            height,
            clear_color,
            window_name,
            show_cursor,
            is_bordered,
            is_resizable,
            should_poll_keys,
            should_poll_cursor_pos,
            should_poll_mouse_buttons,
            should_poll_scroll,
        );
        Engine {
            audio_manager: AudioManager::new(),
            input_manager: InputManager::new(window.glfw, window.events),
            window: WindowManager::new(window.window),
        }
    }

    pub fn add_sprite_sheet(
        &mut self, 
        image_type: ImageType, 
        source: String,
        sprite_width: u32,
        sprite_height: u32
    ) -> SpriteSheetId {
        self.window.add_sprite_sheet(image_type, source, sprite_width, sprite_height)
    }

    pub fn add_shader(&mut self, vertex_shader: VertexShader, fragment_shader: FragmentShader) -> ShaderId {
        self.window.add_shader(vertex_shader, fragment_shader)
    }

    pub fn get_sprite(&mut self, id: &SpriteId) -> Option<&mut Sprite> {
        self.window.get_sprite(id)
    }

    pub fn get_all_sprites(&self) -> &HashMap<SpriteId, Sprite> {
        self.window.get_all_sprites()
    }

    pub fn add_sprite(&mut self, sprite: Sprite) -> SpriteId {
        self.window.add_sprite(sprite)
    }

    pub fn remove_sprite(&mut self, sprite_id: &SpriteId) {
        self.window.remove_sprite(sprite_id);
    }

    pub fn toggle_fullscreen(&mut self) {
        self.window.toggle_fullscreen();
    }

    pub fn set_window_size(&mut self, width: i32, height: i32) {
        self.window.set_window_size(width, height);
    }

    pub fn toggle_show_fps(&mut self) {
        self.window.toggle_show_fps();
    }

    pub fn set_fps(&mut self, target_fps: f32) {
        self.window.set_fps(target_fps);
    }

    pub fn stop(&mut self) {
        self.window.shutdown();
    }

    pub fn get_key_events(&mut self) -> HashMap<Key, Action> {
        return self.input_manager.read_events();
    }

    pub fn is_running(&self) -> bool {
        return self.window.is_running();
    }

    pub fn toggle_border(&mut self) {
        self.window.toggle_border();
    }

    pub fn draw_frame(&mut self) {
        unsafe {
            self.window.draw_frame();
        }
    }
}
