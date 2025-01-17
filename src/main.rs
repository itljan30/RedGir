pub mod audio;
pub mod input;
pub mod video;
pub mod engine;
pub mod utility;

use engine::Engine;
use input::input_manager::{Key, Action};
use video::color::Color;

fn main() {
    let mut engine = Engine::new()
        .set_window_size(800, 600)
        .set_clear_color(Color::DARK_GRAY)
        .set_window_name("My Game")
        .poll_keyboard()
        // .borderless()
        .hide_cursor()
        .init();

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
