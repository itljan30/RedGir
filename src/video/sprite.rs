use crate::video::shader_manager::ShaderId;
use crate::utility::file_parser;

use gl::types::GLuint;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Flip {
    None,
    FlipX,
    FlipY,
    FlipXY,
}

pub enum ImageType {
    JPEG,
    PNG,
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
    pub sprites_uv: Vec<(f32, f32, f32, f32)>,
    pub texture_id: u32,
    sheet_id: SpriteSheetId,
}

impl SpriteSheet {
    pub fn from_png(png_path: &str, sprite_width: u32, sprite_height: u32) -> Self {
        match file_parser::get_rbga_from_png(png_path) {
            Ok((width, height, pixel_data)) => {
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
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // Wrap x-axis
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32); // Wrap y-axis
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32); // Minification filter
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32); // Magnification filter

                    gl::TexImage2D(
                        gl::TEXTURE_2D,
                        0, // Level of detail (0 is the base level)
                        gl::RGBA as i32, // Internal format
                        width as i32,
                        height as i32,
                        0, // Border
                        gl::RGBA, // Format of the pixel data
                        gl::UNSIGNED_BYTE, // Data type of the pixel data
                        pixel_data.as_ptr() as *const _, // Pointer to the pixel data
                    );
                }

                SpriteSheet {
                    sprites_uv,
                    sheet_id: SpriteSheetId::new(0),
                    texture_id,
                }
            },
            Err(e) => panic!("{}", e)
        }
    }

    pub fn from_jpeg(jpeg_path: &str, sprite_width: u32, sprite_height: u32) -> SpriteSheet {
        todo!("Cannot import sprites from jpeg yet")
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
    pub sprite_sheet: SpriteSheetId,
    pub sprite_sheet_index: usize,
    pub sprite_id: SpriteId,
    pub x_position: i32,
    pub y_position: i32,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
    pub flip: Flip,
    pub shader: Option<ShaderId>,
}

impl Sprite {
    pub fn new(
        sprite_sheet: &SpriteSheetId,
        sprite_sheet_index: usize,
        width: u32,
        height: u32,
        shader: Option<ShaderId>
    ) -> Self {
        Sprite {
            x_position: 0,
            sprite_sheet: sprite_sheet.clone(),
            sprite_sheet_index,
            y_position: 0,
            width: width as f32,
            height: height as f32,
            sprite_id: SpriteId::new(0),
            rotation: 0.0,
            flip: Flip::None,
            shader,
        }
    }

    pub fn get_vertices(&self) -> [i32; 12] {
    [   
        self.x_position, self.y_position,
        self.x_position + self.width as i32, self.y_position,
        self.x_position, self.y_position + self.height as i32,

        self.x_position + self.width as i32, self.y_position,
        self.x_position, self.y_position + self.height as i32,
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
}
