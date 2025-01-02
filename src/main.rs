pub mod audio;
pub mod core;
pub mod input;
pub mod video;

use core::engine::Engine;
use input::input_manager::{Key, Action};
use video::sprite::{Sprite, SpriteId};
use video::texture::Texture;

fn get_new_sprite() -> Sprite {
    Sprite::new(Texture::default(), 0.0, 0.0)
}

fn move_left(engine: &mut Engine, sprite: &SpriteId) {
    engine.get_sprite(sprite.clone()).unwrap().translate(0.5, 0.0);
}

fn move_right(engine: &mut Engine, sprite: &SpriteId) {
    engine.get_sprite(sprite.clone()).unwrap().translate(-0.5, 0.0);
}

fn main() {
    let mut engine: Engine = Engine::default();

    let player: SpriteId = engine.add_sprite(get_new_sprite());
    let enemy: SpriteId = engine.add_sprite(get_new_sprite());

    while engine.is_running() {
        let events = engine.get_key_events();
        for (key, action) in events {
            println!("{:?}: {:?}", key, action);
            match (key, action) {
                (Key::D, Action::Pressed | Action::Held) => move_left(&mut engine, &player), 
                (Key::A, Action::Pressed | Action::Held) => move_right(&mut engine, &player),
                (Key::ArrowRight, Action::Pressed | Action::Held) => move_left(&mut engine, &enemy), 
                (Key::ArrowLeft, Action::Pressed | Action::Held) => move_right(&mut engine, &enemy),
                (Key::F5, Action::Pressed) => engine.toggle_show_fps(),
                (Key::Enter, Action::Pressed) => {
                    engine.toggle_fullscreen();
                    engine.toggle_border();
                }
                (Key::Escape, Action::Pressed) => engine.stop(),
                _ => {},
            }
        }
        engine.draw_frame();
    }
}
