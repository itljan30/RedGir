pub mod audio;
pub mod input;
pub mod video;
pub mod engine;
pub mod utility;

use engine::Engine;
use input::input_manager::{Key, Action};
use video::color::Color;
use video::sprite::{Sprite, Texture, ImageType, Flip};

fn get_triangle() -> Vec<[f32; 3]> {
    vec![
        [0.0,  0.5, 1.0],
        [-0.5, -0.5, 1.0],
        [0.5, -0.5, 1.0],
    ]
}

fn main() {
    let mut engine: Engine = Engine::new(
        800, 600,
        Color::DARK_GRAY,
        "Test Game",
        true, true, true, true, 
        false, false, false,
    );

    let sprite_sheet_id = engine.add_sprite_sheet(ImageType::PNG, "../sprite_sheet.png".to_string(), 10, 10);

    let enemy = engine.add_sprite(
        Sprite::new(Texture::from_sprite_sheet(&sprite_sheet_id, 3), None)
    );

    engine.get_sprite(&enemy).unwrap()
        .translate(10, 10)
        .set_flip(Flip::FlipX)
        .set_scale(1.0, 3.0)
        .set_texture(Texture::from_sprite_sheet(&sprite_sheet_id, 4))
        .set_shader(None)
        .set_rotation(3.5);

    while engine.is_running() {
        let events = engine.get_key_events();
        for (key, action) in events {
            println!("{:?}: {:?}", key, action);
            match (key, action) {
                (Key::F5, Action::Pressed) => engine.toggle_show_fps(),
                (Key::Escape, Action::Pressed) => engine.stop(),
                _ => {},
            }
        }
        engine.draw_frame();
    }
}
