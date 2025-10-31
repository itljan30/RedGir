use glfw::{Context, PWindow};
use gl::types::GLuint;

use crate::engine::{GetId, Engine};
use crate::utility::timer::Timer;
use crate::video::color::Color;
use crate::video::sprite::{Sprite, SpriteId, SpriteSheet, SpriteSheetId, SpriteSheetError};
use crate::video::shader_manager::{
    ShaderId, VertexShader, FragmentShader, ShaderError, ShaderProgram,
    DEFAULT_FRAGMENT_SHADER, DEFAULT_VERTEX_SHADER, Attribute, Uniform,
};

use std::thread::yield_now;
use std::collections::{HashMap, BTreeMap};

pub struct WindowManager {
    window: PWindow,
    sprite_sheets: HashMap<SpriteSheetId, SpriteSheet>,
    sprites: HashMap<SpriteId, Sprite>,
    shaders: HashMap<ShaderId, ShaderProgram>,
    default_fragment: Option<FragmentShader>,
    default_vertex: Option<VertexShader>,
    default_shader: Option<ShaderId>,
    timer: Timer,
    target_frame_time: f32,
    show_fps: bool,
    last_sprite_id: u32,
}

impl WindowManager {
    pub fn new(window: PWindow) -> Self {
        let mut shaders = HashMap::new();
        
        let default_vertex = VertexShader::new(DEFAULT_VERTEX_SHADER);
        let default_fragment = FragmentShader::new(DEFAULT_FRAGMENT_SHADER);

        let mut success = true;

        if let Err(err) = default_vertex.as_ref() {
            eprintln!("Error: Failed to create default vertex shader:\n\t{}", err);
            success = false;
        }
        
        if let Err(err) = default_fragment.as_ref() {
            eprintln!("Error: Failed to create default fragment shader:\n\t{}", err);
            success = false;
        }

        let mut fragment = None;
        let mut vertex = None;
        let mut shader_id = None;
        if success {
            let default_vertex = default_vertex.unwrap();
            let default_fragment = default_fragment.unwrap();
            let default_shader = ShaderProgram::new(
                &default_vertex, 
                &default_fragment,
                vec![
                    Attribute::position("u_position".to_string(), 0),
                    Attribute::texture_uv_from_sprite_sheet("tex_coords".to_string(), 1),
                ],
                vec![
                    Uniform::flip("u_flip".to_string()),
                    Uniform::rotation("u_rotation".to_string()),
                    Uniform::sprite_center("u_sprite_center".to_string()),
                    Uniform::texture_from_sprite_sheet("tex_sample".to_string()),
                    Uniform::aspect_ratio("u_aspect_ratio".to_string()),
                ],
            );

            if let Err(err) = default_shader {
                fragment = None;
                vertex = None;
                eprintln!("Error: Failed to link default shaders:\n\t{}", err);
            }
            else {
                let default_shader = default_shader.unwrap();
                let id = default_shader.id();
                shaders.insert(id, default_shader);
                shader_id = Some(id);
                vertex = Some(default_vertex);
                fragment = Some(default_fragment);
            }
        }

        Self{
            window,
            sprite_sheets: HashMap::new(),
            sprites: HashMap::new(),
            shaders,
            default_vertex: vertex,
            default_fragment: fragment,
            default_shader: shader_id,
            timer: Timer::new(),
            target_frame_time: 0.0,
            show_fps: false,
            last_sprite_id: 0,
        }
    }

    pub fn get_texture_from_sprite_sheet(&self, sprite_sheet_id: SpriteSheetId) -> Option<GLuint> {
        Some(self.sprite_sheets.get(&sprite_sheet_id)?.get_texture())
    }
    
    pub fn get_default_shader(&self) -> Option<ShaderId> {
        self.default_shader
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
        let sprite_sheet = SpriteSheet::from_image(path, sprite_width, sprite_height)?;
        let sheet_id = sprite_sheet.id();
        self.sprite_sheets.insert(sheet_id, sprite_sheet);
        Ok(sheet_id)
    }

    pub fn add_quad(
        &mut self, 
        color: Color, 
        x: i32, y: i32, 
        layer: i32, 
        width: u32, 
        height: u32,
        shader: ShaderId
    ) -> Result<SpriteId, SpriteSheetError> {
        let sprite_sheet = SpriteSheet::from_color(color)?;
        let sheet_id = sprite_sheet.id();
        self.sprite_sheets.insert(sheet_id, sprite_sheet);

        Ok(self.add_sprite(sheet_id, 0, x, y, layer, width, height, shader))
    }

    pub fn get_sprite(&mut self, id: SpriteId) -> Option<&mut Sprite> {
        self.sprites.get_mut(&id)
    }

    pub fn add_shader_program(
        &mut self,
        vertex_shader: &VertexShader,
        fragment_shader: &FragmentShader,
        attributes: Vec<Attribute>,
        uniforms: Vec<Uniform>,
    ) -> Result<ShaderId, ShaderError> {
        let program = ShaderProgram::new(
            vertex_shader,
            fragment_shader,
            attributes,
            uniforms,
        )?;
        let shader_id = program.id();
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
        shader: ShaderId,
    ) -> SpriteId {
        let mut sprite = Sprite::new(
            sprite_sheet, sprite_index, 
            x_position, y_position, layer, 
            width, height, shader,
        );

        sprite.set_id(self.last_sprite_id);
        self.last_sprite_id += 1;
        let sprite_id = sprite.id();
        self.sprites.insert(sprite_id, sprite);
        sprite_id
    }

    pub fn remove_sprite(&mut self, sprite_id: SpriteId) {
        self.sprites.remove(&sprite_id);
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

    pub fn get_default_vertex_shader(&self) -> Option<&VertexShader> {
        if let Some(shader) = self.default_vertex.as_ref() {
            Some(shader)
        } else { None }
    }

    pub fn get_default_fragment_shader(&self) -> Option<&FragmentShader> {
        if let Some(shader) = self.default_fragment.as_ref() {
            Some(shader)
        } else { None }
    }

    pub fn set_fps(&mut self, fps: f32) {
        self.target_frame_time = 1.0 / fps;
    }

    pub fn shutdown(&mut self) {
        self.window.set_should_close(true);
    }

    pub fn swap_buffers(&mut self) {
        if self.target_frame_time != 0.0 {
            while self.timer.get_elapsed_seconds() < self.target_frame_time {
                yield_now();
            }
        }

        if self.show_fps {
            println!("{:.2}", 1.0 / self.timer.get_elapsed_seconds());
        }

        self.timer.reset();
        self.window.swap_buffers();
    }

    pub fn get_uv_from_sprite_sheet(&self, sprite_sheet: SpriteSheetId, index: usize) -> Option<(f32, f32, f32, f32)> {
        Some(self.sprite_sheets.get(&sprite_sheet)?.get_uv(index))
    }

    pub unsafe fn draw_frame(&mut self, engine: *const Engine) {
        gl::Clear(gl::COLOR_BUFFER_BIT);

        // Sort sprites by layer and batch by shader to avoid unnecessary binding
        let mut grouped_sprites: BTreeMap<i32, HashMap<ShaderId, Vec<&Sprite>>> = BTreeMap::new();

        for sprite in self.sprites.values() {
            grouped_sprites.entry(sprite.get_layer()).or_default()
                .entry(sprite.get_shader()).or_default().push(sprite);
        }

        for groups in grouped_sprites.values() {
            for (shader, group) in groups.iter() {
                let program = self.shaders.get(&shader).unwrap();
                program.apply();
                program.apply_uniforms(&*engine, group[0]);

                program.fill_vbo(&*engine, &group, program.bytes_per_sprite());
                program.fill_ebo(group.len());

                gl::DrawElements(gl::TRIANGLES, (group.len() * 6) as i32, gl::UNSIGNED_INT, std::ptr::null());
            }
        }
        self.swap_buffers();
    }
}
