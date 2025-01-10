pub mod audio;
pub mod input;
pub mod video;
pub mod engine;
pub mod utility;

use engine::Engine;
use input::input_manager::{Key, Action};
use video::color::Color;
use video::sprite::{Sprite, SpriteId, SpriteSheet, SpriteSheetId};

fn draw_text(
    engine: &mut Engine,
    sprite_sheet: &SpriteSheetId,
    text: &str, x: usize, y: usize, size: usize
) -> Vec<SpriteId> {
    let mut chars = Vec::new();
    for (i, c) in text.chars().enumerate() {
        chars.push(engine.add_sprite(Sprite::new(
            sprite_sheet,
            c as usize - 'a' as usize,
            (x + (i * size)) as u32,
            (y + (i * size)) as u32, 
            None
        )))
    }
    chars
}

fn remove_sprites(engine: &mut Engine, sprites: &Vec<SpriteId>) {
    for sprite in sprites.iter() {
        engine.remove_sprite(sprite);
    }
}

fn move_sprite(engine: &mut Engine, sprite_id: &SpriteId, dx: i32, dy: i32) {
    engine.get_sprite(sprite_id).unwrap()
        .translate(dx, dy);
}

fn main() {
    let mut engine: Engine = Engine::new(
        1080, 720,
        Color::DARK_GRAY,
        "Test Game",
        true, true, true, true, 
        false, false, false,
    );

    let text_sheet = engine.add_sprite_sheet(SpriteSheet::from_png("assets/font.png", 16, 16));
    let text = draw_text(&mut engine, &text_sheet, "hello", 0, 0, 100);

    while engine.is_running() {
        let events = engine.get_key_events();
        for (key, action) in events {
            println!("{:?}: {:?}", key, action);
            match (key, action) {
                (Key::F5, Action::Pressed) => engine.toggle_show_fps(),
                (Key::Escape, Action::Pressed) => engine.stop(),
                (Key::Enter, Action::Pressed) => remove_sprites(&mut engine,&text),
                _ => {},
            }
        }
        engine.draw_frame();
    }
}
