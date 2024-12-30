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
        for (key, action) in events {
            println!("{:?}: {:?}", key, action);
            match (key, action) {
                // (Key::Escape, Action::Pressed) => engine.stop(),
                _ => {},
            }
        }
    }
}
