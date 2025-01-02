use crate::video::texture::Texture;
use crate::video::color::Color;
use crate::video::shader_manager::ShaderId;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Flip {
    None,
    FlipX,
    FlipY,
    FlipXY,
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

pub struct Sprite {
    texture: Texture,
    pub vertices: Vec<[f32; 3]>,
    x: f32,
    y: f32,
    sprite_id: SpriteId,
    scale: f32,
    rotation: f32,
    flip: Flip,
    color: Color,
    shader: Option<ShaderId>,
}

impl Sprite {
    pub fn new(
        texture: Texture, 
        x: f32, y: f32, 
        vertices: Vec<[f32; 3]>, 
        color: Color, 
        shader: Option<ShaderId>
        ) -> Self {

        Sprite {
            x,
            y,
            texture,
            vertices,
            sprite_id: SpriteId::new(0),
            rotation: 0.0,
            scale: 0.0,
            flip: Flip::None,
            color,
            shader,
        }
    }

    pub fn get_position(&self) -> (&f32, &f32) {
        (&self.x, &self.y)
    }

    pub fn translate(&mut self, dx: f32, dy: f32) -> &mut Self {
        self.x += dx;
        self.y += dy;
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

    pub fn set_position(&mut self, x: f32, y: f32) -> &mut Self {
        self.x = x;
        self.y = y;
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
