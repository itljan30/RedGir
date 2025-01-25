pub mod audio;
pub mod input;
pub mod video;
pub mod engine;
pub mod utility;

use engine::Engine;
use input::input_manager::{Key, Action};
use video::color::Color;
use video::sprite::{SpriteId, SpriteSheetId};

fn draw_text(engine: &mut Engine, sprite_sheet_id: SpriteSheetId, text: String, x: i32, y: i32) -> Vec<SpriteId> {
    let indices: Vec<usize> = text.chars().map(|c| c as usize - (' ' as usize)).collect();

    let mut characters: Vec<SpriteId> = Vec::new();
    for (i, index) in indices.iter().enumerate() {
        characters.push(engine.add_sprite(sprite_sheet_id, *index, x + (64 * i as i32), y, 0, 64, 64, None));
    }
    characters
}

fn move_quad(engine: &mut Engine, quad_id: SpriteId, dx: i32, dy: i32) {
    let quad = engine.get_sprite(quad_id).unwrap();
    let (x, y) = quad.get_position();

    quad.set_position(x + dx, y + dy);
}

fn main() {
    let mut engine = Engine::new()
        .set_window_size(1920, 1080)
        .set_clear_color(Color::DARK_GRAY)
        .set_window_name("My Game")
        .poll_keyboard()
        .borderless()
        .hide_cursor()
        .init();

    let sprite_sheet_id = engine.add_sprite_sheet("assets/font.png", 16, 16).unwrap();
    // let quad = engine.add_sprite(sprite_sheet_id, 0, 0, 0, 0, 800, 600, None);

    let text = draw_text(&mut engine, sprite_sheet_id, "Hello, world!".to_string(), 0, 1080 - 64);

    while engine.is_running() {
        let events = engine.get_key_events();
        for (key, action) in events {
            println!("{:?}: {:?}", key, action);
            match (key, action) {
                // (Key::ArrowUp, Action::Pressed | Action::Held) => move_quad(&mut engine, quad, 0, 10),
                // (Key::ArrowDown, Action::Pressed | Action::Held) => move_quad(&mut engine, quad, 0, -10),
                // (Key::ArrowLeft, Action::Pressed | Action::Held) => move_quad(&mut engine, quad, -10, 0),
                // (Key::ArrowRight, Action::Pressed | Action::Held) => move_quad(&mut engine, quad, 10, 0),
                (Key::F5, Action::Pressed) => engine.toggle_show_fps(),
                (Key::Escape, Action::Pressed) => engine.stop(),
                _ => {},
            }
        }
        engine.draw_frame();
    }
}
