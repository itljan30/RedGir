pub mod audio;
pub mod input;
pub mod video;
pub mod engine;
pub mod utility;

use engine::Engine;
use input::input_manager::{Key, Action};
use video::sprite::{SpriteId, SpriteSheetId};
use video::color::Color;

fn draw_text(engine: &mut Engine,
    input: String,
    font_sheet: &SpriteSheetId,
    x: i32, y: i32, layer: i32,
    width: u32, height: u32
) -> Vec<SpriteId> {
    let characters: Vec<usize> = input
        .chars()
        .map(|c| (c as usize).saturating_sub(' ' as usize))
        .collect();

    let mut text_sprite_ids = Vec::new();

    for (i, character) in characters.iter().enumerate() {
        text_sprite_ids.push(engine.add_sprite(
            font_sheet.clone(), 
            *character,
            x + (i as u32 * width) as i32,
            y,
            layer,
            width, height,
            None));
    }

    text_sprite_ids
}

fn main() {
    let mut engine = Engine::new()
        .set_window_size(1920, 1080)
        .set_clear_color(Color::LIGHT_GRAY)
        .set_window_name("My Game")
        .poll_keyboard()
        .borderless()
        .hide_cursor()
        .init();

    let sheet_id = engine.add_sprite_sheet("assets/font.png", 16, 16).unwrap();


    let _ = draw_text(&mut engine, "I'm able to write text in here now! yay!!!!!".to_string(), &sheet_id, 0, 1080 - 32, 0, 32, 32);
    let _ = draw_text(&mut engine, "This is another line I guess".to_string(), &sheet_id, 353, 601, 0, 32, 32);
    let _ = draw_text(&mut engine, "How about this one?".to_string(), &sheet_id, 1063, 800, 0, 32, 32);

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
