use crate::video::shader_manager::ShaderId;
use crate::video::color::Color;
use crate::utility::file_parser;

use gl::types::GLuint;

#[derive(Debug)]
pub enum SpriteSheetError {
    IOError(String),
    TextureCreationError(String),
    InvalidSpriteDimensions(String),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Flip {
    None,
    FlipX,
    FlipY,
    FlipXY,
}

#[derive(Clone, Copy, Eq, Debug, Hash)]
pub struct SpriteSheetId {
    id: u32,
}

impl SpriteSheetId {
    pub fn new(id: u32) -> Self {
        SpriteSheetId {
            id,
        }
    }
}

impl PartialEq for SpriteSheetId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub struct SpriteSheet {
    sprites_uv: Vec<(f32, f32, f32, f32)>,
    texture_id: u32,
    sheet_id: SpriteSheetId,
}

impl SpriteSheet {
    pub fn get_uv(&self, index: usize) -> (f32, f32, f32, f32) {
        self.sprites_uv[index]
    }

    pub fn get_texture(&self) -> u32 {
        self.texture_id
    }
    
    pub fn from_image(png_path: &str, sprite_width: u32, sprite_height: u32) -> Result<Self, SpriteSheetError> {
        match file_parser::get_rbga_from_image(png_path) {
            Ok((width, height, pixel_data)) => {
                if width % sprite_width != 0 || height % sprite_height != 0 {
                    return Err(SpriteSheetError::InvalidSpriteDimensions(
                        format!("Error: {} was given invalid dimensions of {}, {}", png_path, sprite_width, sprite_height)
                    ));
                }

                let mut sprites_uv = Vec::new();

                for row in 0..(height / sprite_height) {
                    for col in 0..(width / sprite_width) {
                        let u_min = col as f32 * sprite_width as f32 / width as f32;
                        let v_min = row as f32 * sprite_height as f32 / height as f32;
                        let u_max = (col + 1) as f32 * sprite_width as f32 / width as f32;
                        let v_max = (row + 1) as f32 * sprite_height as f32 / height as f32;

                        sprites_uv.push((u_min, v_min, u_max, v_max));
                    }
                }

                let mut texture_id: GLuint = 0;

                unsafe {
                    gl::GenTextures(1, &mut texture_id);
                    gl::BindTexture(gl::TEXTURE_2D, texture_id);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

                    gl::TexImage2D(
                        gl::TEXTURE_2D,
                        0,
                        gl::RGBA as i32,
                        width as i32,
                        height as i32,
                        0,
                        gl::RGBA,
                        gl::UNSIGNED_BYTE,
                        pixel_data.as_ptr() as *const _,
                    );

                    if gl::GetError() != gl::NO_ERROR {
                        println!("OpenGL Error: {}", gl::GetError());
                        panic!("Failed to create texture");
                    }
                }

                Ok(SpriteSheet {
                    sprites_uv,
                    sheet_id: SpriteSheetId::new(0),
                    texture_id,
                })
            },
            Err(e) => Err(SpriteSheetError::IOError(e))
        }
    }

    pub fn set_id(&mut self, id: u32) {
        self.sheet_id = SpriteSheetId::new(id);
    }

    pub fn get_id(&self) -> SpriteSheetId {
        self.sheet_id
    }
}

#[derive(Clone, Copy, Eq, Debug, Hash)]
pub struct SpriteId {
    id: u32,
}

impl PartialEq for SpriteId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl SpriteId {
    fn new(id: u32) -> Self {
        SpriteId {
            id,
        }
    }
}

pub struct Sprite {
    sprite_id: SpriteId,
    x_position: i32,
    y_position: i32,
    width: f32,
    height: f32,
    layer: i32,
    rotation: f32,
    flip: Flip,
    sprite_sheet: Option<SpriteSheetId>,
    sprite_sheet_index: Option<usize>,
    color: Option<Color>,
    shader: Option<ShaderId>,
}

impl Sprite {
    pub fn new(
        sprite_sheet: Option<SpriteSheetId>,
        sprite_sheet_index: Option<usize>,
        x_position: i32,
        y_position: i32,
        layer: i32,
        width: u32,
        height: u32,
        color: Option<Color>,
        shader: Option<ShaderId>,
    ) -> Self {
        Sprite {
            sprite_sheet,
            sprite_sheet_index,
            x_position,
            y_position,
            layer,
            width: width as f32,
            height: height as f32,
            sprite_id: SpriteId::new(0),
            rotation: 0.0,
            flip: Flip::None,
            color,
            shader,
        }
    }

    pub fn get_sprite_sheet_index(&self) -> Option<usize> {
        self.sprite_sheet_index
    }

    pub fn get_sprite_sheet(&self) -> Option<SpriteSheetId> {
        self.sprite_sheet
    }

    pub fn get_vertices(&self) -> [i32; 12] {
    [   
        // bottom left
        self.x_position, self.y_position,
        // bottom right
        self.x_position + self.width as i32, self.y_position,
        // top left
        self.x_position, self.y_position + self.height as i32,

        // bottom right
        self.x_position + self.width as i32, self.y_position,
        // top left
        self.x_position, self.y_position + self.height as i32,
        // top right
        self.x_position + self.width as i32, self.y_position + self.height as i32
    ]}

    pub fn get_position(&self) -> (&i32, &i32) {
        (&self.x_position, &self.y_position)
    }

    pub fn translate(&mut self, dx: i32, dy: i32) -> &mut Self {
        self.x_position += dx;
        self.y_position += dy;
        self
    }

    pub fn get_id(&self) -> SpriteId {
        self.sprite_id.clone()
    }

    pub fn set_id(&mut self, id: u32) {
        self.sprite_id = SpriteId::new(id);
    }

    pub fn set_shader(&mut self, shader: Option<ShaderId>) -> &mut Self {
        self.shader = shader;
        self
    }

    pub fn set_texture(&mut self, sprite_sheet: SpriteSheetId, sprite_sheet_index: usize) -> &mut Self {
        self.sprite_sheet = Some(sprite_sheet);
        self.sprite_sheet_index = Some(sprite_sheet_index);
        self
    }

    pub fn set_height(&mut self, height: u32) -> &mut Self {
        self.height = height as f32;
        self
    }

    pub fn set_width(&mut self, width: u32) -> &mut Self {
        self.width = width as f32;
        self
    }

    pub fn set_position(&mut self, x: i32, y: i32) -> &mut Self {
        self.x_position = x;
        self.y_position = y;
        self
    }

    pub fn set_scale(&mut self, scale_x: f32, scale_y: f32) -> &mut Self {
        self.width *= scale_x;
        self.height *= scale_y;
        self
    }

    pub fn set_rotation(&mut self, rotation: f32) -> &mut Self {
        self.rotation = rotation;
        self
    }

    pub fn set_flip(&mut self, flip: Flip) -> &mut Self {
        self.flip = flip;
        self
    }
    
    pub fn get_layer(&self) -> i32 {
        self.layer
    }

    pub fn get_height(&self) -> u32 {
        self.height as u32
    }

    pub fn get_width(&self) -> u32 {
        self.width as u32
    }
}
