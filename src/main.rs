pub mod audio;
pub mod input;
pub mod video;
pub mod engine;
pub mod utility;

use engine::Engine;
use input::input_manager::{Key, Action};
use video::sprite::{SpriteId, ImageType};
use video::color::Color;

fn move_sprite(engine: &mut Engine, sprite_id: SpriteId, dx: i32, dy: i32, screen_width: i32, screen_height: i32) {
    let sprite = engine.get_sprite(sprite_id).unwrap();
    let (x, y) = sprite.get_position();

    sprite.set_position(
        (x + dx).clamp(0, screen_width - sprite.get_width() as i32),
        (y + dy).clamp(0, screen_height - sprite.get_height() as i32)
    );
}

fn apply_gravity(engine: &mut Engine, sprite_id: SpriteId, screen_width: i32, screen_height: i32) {
    move_sprite(engine, sprite_id, 0, -10, screen_width, screen_height);
}

fn main() {
    let mut engine = Engine::new()
        .set_window_size(1920, 1080)
        .set_clear_color(Color::DARK_GRAY)
        .set_window_name("My Game")
        .poll_keyboard()
        .hide_cursor()
        .borderless()
        .init();

    // engine.toggle_fullscreen();

    let text_sheet = engine.add_sprite_sheet(ImageType::PNG("assets/font.png"), 16, 16);
    
    let sprite_id = engine.add_sprite(text_sheet, 0, 0, 0, 0, 40, 40, None);

    while engine.is_running() {
        let events = engine.get_key_events();
        let (width, height) = engine.get_window_dimensions();
        // apply_gravity(&mut engine, sprite_id, width, height);
        for (key, action) in events {
            println!("{:?}: {:?}", key, action);
            match (key, action) {
                (Key::ArrowUp, Action::Pressed | Action::Held) => move_sprite(&mut engine, sprite_id, 0, 20, width, height),
                (Key::ArrowDown, Action::Pressed | Action::Held) => move_sprite(&mut engine, sprite_id, 0, -20, width, height),
                (Key::ArrowRight, Action::Pressed | Action::Held) => move_sprite(&mut engine, sprite_id, 20, 0, width, height),
                (Key::ArrowLeft, Action::Pressed | Action::Held) => move_sprite(&mut engine, sprite_id, -20, 0, width, height),
                (Key::F5, Action::Pressed) => engine.toggle_show_fps(),
                (Key::Escape, Action::Pressed) => engine.stop(),
                _ => {},
            }
        }
        engine.draw_frame();
    }
}
