use crate::audio::audio_manager::AudioManager;
use crate::input::input_manager::{InputManager, Key, Action};
use crate::video::window::WindowManager;
use crate::video::color::Color;
use crate::video::sprite::{SpriteSheetId, SpriteSheetError, Sprite, SpriteId};
use crate::video::shader_manager::{ShaderId, FragmentShader, VertexShader, ShaderError};
use crate::video::glfw_window::GlfwWindow;

use std::collections::HashMap;

pub trait GetId {
    type Id;
    fn id(&self) -> Self::Id;
}

pub struct EngineBuilder {
    window_name: Option<String>,
    width: u32,
    height: u32,
    clear_color: Color,
    show_cursor: bool,
    is_bordered: bool,
    is_resizable: bool,
    should_poll_keys: bool,
    should_poll_cursor_pos: bool,
    should_poll_mouse_buttons: bool,
    should_poll_scroll: bool,
}

impl EngineBuilder {
    pub fn init(&mut self) -> Engine {
        Engine::build(
            self.width, self.height,
            self.clear_color, 
            self.window_name.clone().unwrap_or("Window".to_string()).as_str(),
            self.show_cursor,
            self.is_bordered, self.is_resizable,
            self.should_poll_keys, self.should_poll_cursor_pos,
            self.should_poll_mouse_buttons, self.should_poll_scroll
        )
    }

    pub fn hide_cursor(&mut self) -> &mut Self {
        self.show_cursor = false;
        self
    }

    pub fn borderless(&mut self) -> &mut Self {
        self.is_bordered = false;
        self
    }

    pub fn not_resizable(&mut self) -> &mut Self {
        self.is_resizable = false;
        self
    }

    pub fn poll_mouse_buttons(&mut self) -> &mut Self {
        self.should_poll_mouse_buttons = true;
        self.should_poll_scroll = true;
        self
    }

    pub fn poll_cursor(&mut self) -> &mut Self {
        self.should_poll_cursor_pos = true;
        self
    }

    pub fn poll_keyboard(&mut self) -> &mut Self {
        self.should_poll_keys = true;
        self
    }

    pub fn set_window_size(&mut self, width: u32, height: u32) -> &mut Self {
        self.width = width;
        self.height = height;
        self
    } 

    pub fn set_clear_color(&mut self, color: Color) -> &mut Self {
        self.clear_color = color;
        self
    }

    pub fn set_window_name(&mut self, name: &str) -> &mut Self {
        self.window_name = Some(name.to_string());
        self
    }
}

pub struct Engine {
    // NOTE window is first so that all openGL things get dropped before the glfw context
    window: WindowManager,
    input_manager: InputManager,
    audio_manager: AudioManager,
}

impl Default for Engine {
    fn default() -> Self {
        let window = GlfwWindow::default();

        Engine {
            window: WindowManager::new(window.window),
            audio_manager: AudioManager::new(),
            input_manager: InputManager::new(window.glfw, window.events),
        }
    }
}

impl Engine {
    pub fn new() -> EngineBuilder {
        EngineBuilder {
            width: 800,
            height: 600,
            clear_color: Color::BLACK,
            window_name: None,
            show_cursor: true,
            is_bordered: true,
            is_resizable: true,
            should_poll_keys: false,
            should_poll_scroll: false,
            should_poll_mouse_buttons: false,
            should_poll_cursor_pos: false,
        }
    }

    fn build(
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

    pub fn add_quad(
        &mut self, color: Color, 
        x_position: i32, y_position: i32, 
        layer: i32, width: u32, height: u32
    ) -> SpriteId {
        self.window.add_quad(color, x_position, y_position, layer, width, height)
    }

    pub fn get_window_dimensions(&self) -> (i32, i32) {
        self.window.get_dimensions()
    }

    pub fn add_sprite_sheet(
        &mut self,
        path: &str,
        sprite_width: u32,
        sprite_height: u32
    ) -> Result<SpriteSheetId, SpriteSheetError> {
        self.window.add_sprite_sheet(path, sprite_width, sprite_height)
    }

    pub fn add_shader_group(
        &mut self,
        vertex_shader: &VertexShader,
        fragment_shader: &FragmentShader,
    ) -> Result<ShaderId, ShaderError> {
        self.window.add_shader_program(vertex_shader, fragment_shader)
    }

    pub fn get_sprite(&mut self, id: SpriteId) -> Option<&mut Sprite> {
        self.window.get_sprite(id)
    }

    pub fn get_all_sprites(&self) -> &HashMap<SpriteId, Sprite> {
        self.window.get_all_sprites()
    }

    pub fn add_sprite(
        &mut self, sprite_sheet: SpriteSheetId, 
        sprite_index: usize,
        x_position: i32, y_position: i32,
        layer: i32, width: u32, height: u32,
        shader: Option<ShaderId>,
    ) -> SpriteId {
        self.window.add_sprite(
            sprite_sheet, sprite_index, 
            x_position, y_position, layer, width, 
            height, shader
        )
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
