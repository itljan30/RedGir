use crate::video::shader_manager::ShaderId;
use crate::video::color::Color;
use crate::utility::file_parser;
use crate::engine::GetId;

use image::ImageError;
use gl::types::GLuint;

#[derive(Debug)]
pub enum SpriteSheetError {
    IOError(ImageError),
    TextureCreationError(String),
    InvalidSpriteDimensions(String),
}

impl std::fmt::Display for SpriteSheetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpriteSheetError::IOError(e)                 => write!(f, "IOError: {}", e),
            SpriteSheetError::TextureCreationError(e)    => write!(f, "TextureCreationError: {}", e),
            SpriteSheetError::InvalidSpriteDimensions(e) => write!(f, "InvalidSpriteDimensions: {}", e),
        }
    }
}

impl From<ImageError> for SpriteSheetError {
    fn from(value: ImageError) -> Self {
        SpriteSheetError::IOError(value)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Flip {
    None,
    FlipX,
    FlipY,
    FlipXY,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct SpriteSheetId {
    id: GLuint,
}

pub struct SpriteSheet {
    sprites_uv: Vec<(f32, f32, f32, f32)>,
    texture_id: u32,
}

impl GetId for SpriteSheet {
    type Id = SpriteSheetId;
    fn id(&self) -> SpriteSheetId {
        SpriteSheetId { id: self.texture_id }
    }
}

impl SpriteSheet {
    pub fn get_uv(&self, index: usize) -> (f32, f32, f32, f32) {
        self.sprites_uv[index]
    }

    pub fn get_texture(&self) -> GLuint {
        self.texture_id
    }
    
    pub fn from_image(png_path: &str, sprite_width: u32, sprite_height: u32) -> Result<Self, SpriteSheetError> {
        let (width, height, pixel_data) = file_parser::get_rbga_from_image(png_path)?;

        if width % sprite_width != 0 || height % sprite_height != 0 {
            return Err(SpriteSheetError::InvalidSpriteDimensions(
                format!("SpriteSheet {} was given invalid dimensions: width={}, height={}", png_path, sprite_width, sprite_height)
            ));
        }

        let mut sprites_uv = Vec::new();

        for row in 0..(height / sprite_height) {
            for col in 0..(width / sprite_width) {
                // we're iterating from top to bottom, so we need to invert the v calculations
                let u_min = col as f32 * sprite_width as f32 / width as f32;
                let v_min = (row + 1) as f32 * sprite_height as f32 / height as f32;
                let u_max = (col + 1) as f32 * sprite_width as f32 / width as f32;
                let v_max = row as f32 * sprite_height as f32 / height as f32;

                sprites_uv.push((u_min, v_min, u_max, v_max));
            }
        }

        let texture_id = get_texture_id(width, height, pixel_data)?;

        Ok(SpriteSheet {
            sprites_uv,
            texture_id,
        })
    }

    pub fn from_color(color: Color) -> Result<Self, SpriteSheetError> {
        let pixel_data = vec![color.r, color.b, color.g, color.a];
        let texture_id = get_texture_id(1, 1, pixel_data)?;

        Ok(SpriteSheet {
            sprites_uv: vec![(0.0, 0.0, 1.0, 1.0)],
            texture_id,
        })
    }
}

fn get_texture_id(width: u32, height: u32, pixel_data: Vec<u8>) -> Result<GLuint, SpriteSheetError> {
    let mut texture_id: GLuint = 0;

    unsafe {
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

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

        // TODO improve error message
        let error = gl::GetError();
        if error != gl::NO_ERROR {
            let info = match error {
                gl::INVALID_ENUM => "Invalid Enum",
                gl::INVALID_VALUE => "Invalid Value",
                gl::INVALID_OPERATION => "Invalid Operation",
                gl::OUT_OF_MEMORY => "Out of memory",
                _ => "Unkown error",
            };
            return Err(SpriteSheetError::TextureCreationError(format!("Failed to create texture: {}", info)));
        }
    }
    Ok(texture_id)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct SpriteId {
    id: u32,
}

pub struct Sprite {
    sprite_id: u32,
    x_position: i32,
    y_position: i32,
    width: f32,
    height: f32,
    layer: i32,
    rotation: f32,
    flip: Flip,
    sprite_sheet: SpriteSheetId,
    sprite_sheet_index: usize,
    shader: ShaderId,
}

impl GetId for Sprite {
    type Id = SpriteId;
    fn id(&self) -> SpriteId {
        SpriteId { id: self.sprite_id }
    }
}

impl Sprite {
    pub fn new(
        sprite_sheet: SpriteSheetId,
        sprite_sheet_index: usize,
        x_position: i32,
        y_position: i32,
        layer: i32,
        width: u32,
        height: u32,
        shader: ShaderId,
    ) -> Self {
        Sprite {
            sprite_sheet,
            sprite_sheet_index,
            x_position,
            y_position,
            layer,
            width: width as f32,
            height: height as f32,
            sprite_id: 0,
            rotation: 0.0,
            flip: Flip::None,
            shader,
        }
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    pub fn get_flip(&self) -> Flip {
        self.flip
    }

    pub fn get_sprite_sheet_index(&self) -> usize {
        self.sprite_sheet_index
    }

    pub fn get_sprite_sheet(&self) -> SpriteSheetId {
        self.sprite_sheet
    }

    pub fn get_position(&self) -> (i32, i32) {
        (self.x_position, self.y_position)
    }

    pub fn translate(&mut self, dx: i32, dy: i32) -> &mut Self {
        self.x_position += dx;
        self.y_position += dy;
        self
    }

    pub fn set_id(&mut self, id: u32) {
        self.sprite_id = id;
    }

    pub fn get_shader(&self) -> ShaderId {
        self.shader
    }

    pub fn set_shader(&mut self, shader: ShaderId) -> &mut Self {
        self.shader = shader;
        self
    }

    pub fn set_texture(&mut self, sprite_sheet: SpriteSheetId, sprite_sheet_index: usize) -> &mut Self {
        self.sprite_sheet = sprite_sheet;
        self.sprite_sheet_index = sprite_sheet_index;
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
