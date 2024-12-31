pub mod assets;
pub mod audio;
pub mod core;
pub mod input;
pub mod video;

use core::engine::Engine;
use input::input_manager::{Key, Action};

fn main() {
    let mut engine: Engine = Engine::init();

    while engine.is_running() {
        let events = engine.get_key_events();
        engine.set_fps(144.0);

        for (key, action) in events {
            println!("{:?}: {:?}", key, action);
            match (key, action) {
                (Key::Escape, Action::Held) => engine.stop(),
                _ => {},
            }
        }
        engine.draw_frame();
        engine.display_frame();
    }
}
