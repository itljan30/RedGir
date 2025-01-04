use glfw::{Context, PWindow};
use gl::types::{GLuint, GLint};

use crate::utility::timer::Timer;
use crate::video::sprite::{Sprite, SpriteId, SpriteSheet, SpriteSheetId};
use crate::video::shader_manager::{ShaderId, ShaderProgram, VertexShader, FragmentShader};

use std::thread::yield_now;
use std::collections::HashMap;

pub struct WindowManager {
    window: PWindow,
    sprite_sheets: HashMap<SpriteSheetId, SpriteSheet>,
    sprites: HashMap<SpriteId, Sprite>,
    shaders: HashMap<ShaderId, ShaderProgram>,
    timer: Timer,
    target_frame_time: f32,
    show_fps: bool,
    last_sprite_id: u32,
    last_sheet_id: u32,
    vao: GLuint,
    vbo: GLuint,
}

impl WindowManager {
    pub fn new(window: PWindow) -> Self {
        let mut shaders = HashMap::new();

        let mut default_shader = ShaderProgram::default();
        default_shader.compile_and_link();
        let shader = ShaderId::new(default_shader.id);

        shaders.insert(shader, default_shader);

        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
        }

        WindowManager{
            window,
            sprite_sheets: HashMap::new(),
            sprites: HashMap::new(),
            shaders,
            timer: Timer::new(),
            target_frame_time: 1.0 / 60.0,
            show_fps: false,
            last_sprite_id: 0,
            last_sheet_id: 0,
            vao,
            vbo,
        }
    }

    pub fn is_running(&self) -> bool {
        return !self.window.should_close();
    }

    pub fn add_sprite_sheet(&mut self, mut sprite_sheet: SpriteSheet) -> SpriteSheetId {
        sprite_sheet.set_id(self.last_sprite_id);
        self.last_sheet_id += 1;
        let sheet_id = sprite_sheet.get_id();
        self.sprite_sheets.insert(sheet_id.clone(), sprite_sheet);
        sheet_id
    }

    pub fn get_sprite(&mut self, id: &SpriteId) -> Option<&mut Sprite> {
        self.sprites.get_mut(id)
    }

    pub fn add_shader(&mut self, vertex_shader: VertexShader, fragment_shader: FragmentShader) -> ShaderId {
        let mut shader = ShaderProgram::new(vertex_shader, fragment_shader);
        shader.compile_and_link();
        let id = ShaderId::new(shader.id);
        self.shaders.insert(id.clone(), shader);
        id
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

    pub fn toggle_border(&mut self) {
        if self.window.is_decorated() {
            self.window.set_decorated(false);
        }
        else {
            self.window.set_decorated(true);
        }
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

    pub fn set_fps(&mut self, fps: f32) {
        self.target_frame_time = 1.0 / fps;
    }

    pub fn shutdown(&mut self) {
        self.window.set_should_close(true);
    }

    pub fn swap_buffers(&mut self) {
        while self.timer.get_elapsed() < self.target_frame_time {
            yield_now();
        }

        if self.show_fps {
            println!("{:.2}", 1.0 / self.timer.get_elapsed());
        }

        self.timer.reset();
        self.window.swap_buffers();
    }

    // to use custom shaders I should make it so that it sorts the sprites by shader and then
    // creates a separate vao for each shader used to batch efficiently
    pub unsafe fn draw_frame(&mut self) {
        gl::Clear(gl::COLOR_BUFFER_BIT);

        for sprite in self.sprites.values() {
            gl::BindVertexArray(self.vao);
            todo!()
        }

        self.swap_buffers();
    }
}
