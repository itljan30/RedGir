use glfw::{Context, PWindow};
// use gl::types::{GLint, GLuint, GLsizeiptr};

use crate::core::timer::Timer;
use crate::video::sprite::{Sprite, SpriteId};
use crate::video::shader_manager::{ShaderId, ShaderProgram, VertexShader, FragmentShader};
// use crate::video::glfw_window::GlfwWindow;

use std::thread::yield_now;
use std::collections::HashMap;
// use std::ffi::c_void;

pub enum ImageType {
    JPEG,
    PNG,
}

pub struct WindowManager {
    window: PWindow,
    target_frame_time: f64,
    timer: Timer,
    show_fps: bool,
    sprites: HashMap<SpriteId, Sprite>,
    last_sprite_id: u64,
    shaders: Vec<ShaderProgram>,
}

impl WindowManager {
    pub fn new(window: PWindow) -> Self {
        let mut default_shader = ShaderProgram::default();
        default_shader.compile_and_link();
        default_shader.use_shader();
        WindowManager{
            window,
            target_frame_time: 1.0 / 60.0,
            timer: Timer::new(),
            show_fps: false,
            sprites: HashMap::new(),
            last_sprite_id: 0,
            shaders: vec![default_shader],
        }
    }

    pub fn get_sprite(&mut self, id: SpriteId) -> Option<&mut Sprite> {
        self.sprites.get_mut(&id)
    }

    pub fn add_shader(&mut self, vertex_shader: VertexShader, fragment_shader: FragmentShader) -> ShaderId {
        let mut shader = ShaderProgram::new(vertex_shader, fragment_shader);
        shader.compile_and_link();
        let id = ShaderId::new(shader.id);
        self.shaders.push(shader);
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

    pub fn set_fps(&mut self, fps: f64) {
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

    // TODO just add gl error reporting so that I can figure what is going on
    // to use custom shaders I should make it so that it sorts the sprites by shader and then
    // creates a separate vao for each shader used, then send them in separately
    pub unsafe fn draw_frame(&mut self) {
        gl::Clear(gl::COLOR_BUFFER_BIT);

        // let mut vao: GLuint = 0;
        // gl::GenVertexArrays(1, &mut vao);
        // gl::BindVertexArray(vao);
        //
        // for sprite in self.sprites.values() {
        //     add_vbo_from_sprite(sprite);
        // }
        //
        // gl::UseProgram(self.default_shader.id);
        // gl::DrawArrays(gl::TRIANGLES, 0, 3);
        // gl::BindVertexArray(0);

        self.swap_buffers();
    }

    pub fn is_running(&self) -> bool {
        return !self.window.should_close();
    }
}

// unsafe fn add_vbo_from_sprite(sprite: &Sprite) {
//     let mut vbo: GLuint = 0;
//     gl::GenBuffers(1, &mut vbo);
//     gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
//     gl::BufferData(
//         gl::ARRAY_BUFFER,
//         sprite.vertices.len() as GLsizeiptr,
//         sprite.vertices.as_ptr() as *const c_void,
//         gl::DYNAMIC_DRAW
//     );
//     gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * size_of::<f32>() as GLint, 0 as *const c_void);
//     gl::EnableVertexAttribArray(0);
// }
