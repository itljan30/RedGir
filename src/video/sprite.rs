use crate::video::texture::Texture;

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
    x: f64,
    y: f64,
    sprite_id: SpriteId,
    scale: f64,
    rotation: f64,
    flip: Flip,
}

impl Sprite {
    pub fn new(texture: Texture, x: f64, y: f64) -> Self {
        Sprite {
            texture,
            x,
            y,
            sprite_id: SpriteId::new(0),
            rotation: 0.0,
            scale: 0.0,
            flip: Flip::None,
        }
    }

    pub fn get_position(&self) -> (&f64, &f64) {
        (&self.x, &self.y)
    }

    pub fn translate(&mut self, dx: f64, dy: f64) -> &mut Self {
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

    pub fn set_position(&mut self, x: f64, y: f64) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn set_scale(&mut self, scale: f64) -> &mut Self {
        self.scale = scale;
        self
    }

    pub fn set_rotation(&mut self, rotation: f64) -> &mut Self {
        self.rotation = rotation;
        self
    }

    pub fn set_flip(&mut self, flip: Flip) -> &mut Self {
        self.flip = flip;
        self
    }
}
