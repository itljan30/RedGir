use crate::video::shader_manager::ShaderId;
use crate::video::window::ImageType;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Flip {
    None,
    FlipX,
    FlipY,
    FlipXY,
}

pub struct SpriteSheet {
    image_type: ImageType,
    source: String,
    sheet_width: u32,
    sheet_height: u32,
    sprite_width: u32,
    sprite_height: u32,
}

impl SpriteSheet {
    pub fn new(
        image_type: ImageType,
        source: String, 
        sprite_width: u32, 
        sprite_height: u32
    ) -> Self {
        SpriteSheet {
            image_type,
            source,
            sheet_width: 0,
            sheet_height: 0,
            sprite_width,
            sprite_height,
        }
    }
}

pub struct Texture {
    texture_source: String,
    width: u32,
    height: u32,
}

impl Texture {
    pub fn new(texture_source: String, width: u32, height: u32) -> Self {
        Texture {
            texture_source,
            width,
            height,
        }
    }
}

#[derive(Clone, Copy, Eq, Debug, Hash)]
pub struct SpriteId {
    id: u64,
}

impl PartialEq for SpriteId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl SpriteId {
    fn new(id: u64) -> Self {
        SpriteId {
            id,
        }
    }
}

/**
 * Sprites are rectangles with a texture
 * x: i32, and y: i32 are the top left pixel of the sprite
 *  
 */
pub struct Sprite {
    texture: Texture,
    x_position: i32,
    y_position: i32,
    sprite_id: SpriteId,
    scale: f32,
    rotation: f32,
    flip: Flip,
    shader: Option<ShaderId>,
}

impl Sprite {
    pub fn new(
        texture: Texture, 
        shader: Option<ShaderId>
        ) -> Self {

        Sprite {
            x_position: 0,
            y_position: 0,
            texture,
            sprite_id: SpriteId::new(0),
            rotation: 0.0,
            scale: 0.0,
            flip: Flip::None,
            shader,
        }
    }

    pub fn from_sprite_sheet(
        sprite_sheet: SpriteSheet,
        index: u32,
        shader: Option<ShaderId>,
    ) {
        // Sprite {
        //     todo!("Add a way to get texture from spritesheet")
        //     x_position: 0,
        //     y_position: 0,
        //     shader,
        //     rotation: 0.0,
        //     scale: 0.0,
        //     flip: Flip::None,
        //     sprite_id: SpriteId::new(0),
        // }
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

    pub fn set_id(&mut self, id: u64) {
        self.sprite_id = SpriteId::new(id);
    }

    pub fn set_texture(&mut self, texture: Texture) -> &mut Self {
        self.texture = texture;
        self
    }

    pub fn set_position(&mut self, x: i32, y: i32) -> &mut Self {
        self.x_position = x;
        self.y_position = y;
        self
    }

    pub fn set_scale(&mut self, scale: f32) -> &mut Self {
        self.scale = scale;
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
