mod video;
mod input;
mod audio;
mod engine;
mod utility;

mod ffi;

pub use engine::Engine;
pub use input::input_manager::{Key, Action};
pub use video::color::Color;
pub use video::sprite::{Sprite, SpriteSheet, SpriteId, SpriteSheetId, Flip, SpriteSheetError};
pub use video::shader_manager::{
    ShaderId, FragmentShader, VertexShader, ShaderError,
    Attribute, AttributeData, Uniform, UniformData, ShaderProgram,
};
pub use utility::timer::Timer;
