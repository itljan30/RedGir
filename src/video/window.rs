use glfw::{Context, PWindow};
use gl::types::GLuint;

use crate::utility::timer::Timer;
use crate::video::color::Color;
use crate::video::sprite::{Sprite, SpriteId, SpriteSheet, SpriteSheetId, SpriteSheetError};
use crate::video::shader_manager::{
    ShaderError, ShaderProgram, DEFAULT_FRAGMENT_SHADER, DEFAULT_VERTEX_SHADER
};

use std::thread::yield_now;
use std::collections::HashMap;
use std::ffi::CString;

pub struct WindowManager {
    window: PWindow,
    sprite_sheets: HashMap<SpriteSheetId, SpriteSheet>,
    sprites: HashMap<SpriteId, Sprite>,
    shaders: HashMap<ShaderId, ShaderProgram>,
    default_shader: ShaderId,
    timer: Timer,
    target_frame_time: f32,
    show_fps: bool,
    last_sprite_id: u32,
    last_sheet_id: u32,
    vao: GLuint,
    vbo: GLuint,
}

impl Drop for WindowManager {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}

impl WindowManager {
    pub fn new(window: PWindow) -> Self {
        let mut shaders = HashMap::new();
        
        let default_shaders = [
            Shader::new(DEFAULT_VERTEX_SHADER, ShaderType::VertexShader),
            Shader::new(DEFAULT_FRAGMENT_SHADER, ShaderType::FragmentShader),
        ];

        let mut shader: Vec<Shader> = Vec::new();

        for shader_result in default_shaders.into_iter() {
            match shader_result {
                Ok(s) => shader.push(s),
                Err(e) => eprintln!("Error: Failed to create default shader: {}", e),
            }
        }

        let mut shader_id: ShaderId = ShaderId::new(0);

        if shader.len() == 2 {
            let default_shader_program = ShaderProgram::new(&shader);
            match default_shader_program {
                Ok(program) => {
                    shader_id = ShaderId::new(program.get_id());
                    shaders.insert(shader_id, program);
                }
                Err(e) => eprintln!("Error: Failed to link default shader program: {}", e),
            }
        }

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
            default_shader: shader_id,
            timer: Timer::new(),
            target_frame_time: 1.0 / 60.0,
            show_fps: false,
            last_sprite_id: 0,
            last_sheet_id: 0,
            vao,
            vbo,
        }
    }

    pub fn get_dimensions(&self) -> (i32, i32) {
        self.window.get_framebuffer_size()
    }

    pub fn is_running(&self) -> bool {
        return !self.window.should_close();
    }

    pub fn add_sprite_sheet(
        &mut self, 
        path: &str,
        sprite_width: u32,
        sprite_height: u32
    ) -> Result<SpriteSheetId, SpriteSheetError> {
        let mut sprite_sheet = SpriteSheet::from_image(path, sprite_width, sprite_height)?;
        sprite_sheet.set_id(self.last_sprite_id);
        self.last_sheet_id += 1;
        let sheet_id = sprite_sheet.get_id();
        self.sprite_sheets.insert(sheet_id.clone(), sprite_sheet);
        Ok(sheet_id)
    }

    pub fn add_quad(&mut self, color: Color, x: i32, y: i32, layer: i32, width: u32, height: u32) -> SpriteId {
        let mut sprite_sheet = SpriteSheet::from_color(color);
        sprite_sheet.set_id(self.last_sprite_id);
        self.last_sheet_id += 1;
        let sheet_id = sprite_sheet.get_id();
        self.sprite_sheets.insert(sheet_id.clone(), sprite_sheet);

        self.add_sprite(sheet_id, 0, x, y, layer, width, height, None)
    }

    pub fn get_sprite(&mut self, id: SpriteId) -> Option<&mut Sprite> {
        self.sprites.get_mut(&id)
    }

    pub fn add_shader_program(&mut self, shaders: &[Shader]) -> Result<ShaderId, ShaderError> {
        let program = ShaderProgram::new(shaders)?;
        let shader_id = ShaderId::new(program.get_id());
        self.shaders.insert(shader_id, program);
        Ok(shader_id)
    }

    pub fn get_all_sprites(&self) -> &HashMap<SpriteId, Sprite> {
        &self.sprites
    }

    pub fn add_sprite(
        &mut self, 
        sprite_sheet: SpriteSheetId,
        sprite_index: usize,
        x_position: i32, y_position: i32,
        layer: i32, width: u32, height: u32,
        shader: Option<ShaderId>,
    ) -> SpriteId {
        let mut sprite = Sprite::new(
            sprite_sheet, sprite_index, 
            x_position, y_position, layer, 
            width, height, shader,
        );

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
        while self.timer.get_elapsed_seconds() < self.target_frame_time {
            yield_now();
        }

        if self.show_fps {
            println!("{:.2}", 1.0 / self.timer.get_elapsed_seconds());
        }

        self.timer.reset();
        self.window.swap_buffers();
    }

    fn get_normalized_vertices(&self, sprite: &Sprite) -> Option<[f32; 24]> {
        let (width, height) = self.window.get_framebuffer_size();
        let vertices = sprite.get_vertices();
        let sheet_id = sprite.get_sprite_sheet();
        let index = sprite.get_sprite_sheet_index();

        self.sprite_sheets.get(&sheet_id).map(|sheet| {
            let (u_min, v_max, u_max, v_min) = sheet.get_uv(index);


            let normalized_vertices: [f32; 24] = [
                // bottom left
                2.0 * vertices[0] as f32 / width as f32 - 1.0, 2.0 * vertices[1] as f32 / height as f32 - 1.0,
                u_min, v_min,
                
                // bottom right
                2.0 * vertices[2] as f32 / width as f32 - 1.0, 2.0 * vertices[3] as f32 / height as f32 - 1.0,
                u_max, v_min,

                // top left
                2.0 * vertices[4] as f32 / width as f32 - 1.0, 2.0 * vertices[5] as f32 / height as f32 - 1.0,
                u_min, v_max,

                // bottom right
                2.0 * vertices[6] as f32 / width as f32 - 1.0, 2.0 * vertices[7] as f32 / height as f32 - 1.0,
                u_max, v_min,

                // top left
                2.0 * vertices[8] as f32 / width as f32 - 1.0, 2.0 * vertices[9] as f32 / height as f32 - 1.0,
                u_min, v_max,

                // top right
                2.0 * vertices[10] as f32 / width as f32 - 1.0, 2.0 * vertices[11] as f32 / height as f32 - 1.0,
                u_max, v_max,
            ];

            normalized_vertices
        })
    }

    pub unsafe fn draw_frame(&mut self) {
        gl::Clear(gl::COLOR_BUFFER_BIT);

        let mut sprites: Vec<&Sprite> = self.sprites.values().collect();
        sprites.sort_by_key(|sprite| sprite.get_layer());

        for sprite in sprites {
            gl::BindVertexArray(self.vao);

            if let Some(vertices) = self.get_normalized_vertices(sprite) {
                gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
                gl::BufferData(
                    gl::ARRAY_BUFFER, 
                    (size_of::<f32>() * 24) as isize,
                    vertices.as_ptr() as *const _,
                    gl::DYNAMIC_DRAW
                );

                gl::VertexAttribPointer(
                    0,
                    2,
                    gl::FLOAT,
                    gl::FALSE,
                    (4 * size_of::<f32>()) as i32,
                    std::ptr::null()
                );
                gl::EnableVertexAttribArray(0);

                gl::VertexAttribPointer(
                    1,
                    2,
                    gl::FLOAT,
                    gl::FALSE,
                    (4 * size_of::<f32>()) as i32,
                    (2 * size_of::<f32>()) as *const _
                );
                gl::EnableVertexAttribArray(1);

                let shader = self.shaders.get(&self.default_shader).unwrap();
                shader.use_program();

                let texture = self.sprite_sheets.get(&sprite.get_sprite_sheet()).unwrap().get_texture();

                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, texture);

                let texture_name = CString::new("tex_sample").unwrap();
                let texture_location = gl::GetUniformLocation(shader.get_id(), texture_name.as_ptr());
                gl::Uniform1i(texture_location, 0);

                gl::DrawArrays(gl::TRIANGLES, 0, 6);
            }
        }
        self.swap_buffers();
    }
}
