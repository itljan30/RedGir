pub mod audio;
pub mod core;
pub mod input;
pub mod video;

use core::engine::Engine;
use input::input_manager::{Key, Action};
use video::color::Color;

fn get_triangle() -> Vec<[f32; 3]> {
    vec![
        [0.0,  0.5, 1.0], // Top vertex
        [-0.5, -0.5, 1.0], // Bottom-left vertex
        [0.5, -0.5, 1.0], // Bottom-right vertex
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

    while engine.is_running() {
        let events = engine.get_key_events();
        for (key, action) in events {
            println!("{:?}: {:?}", key, action);
            match (key, action) {
                (Key::F5, Action::Pressed) => engine.toggle_show_fps(),
                (Key::Enter, Action::Pressed) => _ = engine.draw_shape(400.0, 300.0, get_triangle(), Color::RED, None),
                (Key::Escape, Action::Pressed) => engine.stop(),
                _ => {},
            }
        }
        engine.draw_frame();
    }
}
