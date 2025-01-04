use crate::video::shader_manager::ShaderId;
use crate::utility::file_parser;

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
    sprites_uv: Vec<(f32, f32, f32, f32)>,
    texture_id: u32,
    sheet_id: SpriteSheetId,
}

impl SpriteSheet {
    pub fn from_png(png_path: &str, sprite_width: u32, sprite_height: u32) -> Self {
        match file_parser::get_rbga_from_png(png_path) {
            Ok((width, height, pixel_data)) => {
                let sprites_uv = Vec::new();
                // split sprites_uv properly using sprite dimensions and file dimensions
                // send pixel data to openGL to get back texture_id
                SpriteSheet {
                    sprites_uv,
                    sheet_id: SpriteSheetId::new(0),
                    texture_id: 0,
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
    sprite_sheet: SpriteSheetId,
    sprite_sheet_index: usize,
    sprite_id: SpriteId,
    x_position: i32,
    y_position: i32,
    width: f32,
    height: f32,
    rotation: f32,
    flip: Flip,
    shader: Option<ShaderId>,
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
