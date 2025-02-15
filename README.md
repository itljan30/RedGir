# RedGir

RedGir is a minimalistic 2d game engine designed to abstract away the low-level details of graphics, audio, and input management.
RedGir utilizes Rust's glfw and gl crates to handle graphics in a flexible and portable manner.

## Features
### Core Features
- **Graphics Management:**
    - Sprite based system that allows for easy manipulation of sprite data.
    - Flexible shader management system that allows the user to easily load shaders and supply custom 
    uniforms and attributes via callback functions.

- **Audio Integration:**
   - Abstraction over OpenAL for audio playback and management. *(Planned)*

- **Input Handling:**
    - Easily track which keys have been pressed, held, or released each frame.
    - Mouse position polling. *(Planned)*

### Planned Features
TODO

## Engine Structure

### `Engine`
The main entry point for the game engine, responsible for managing:
- **Window Management**: Handles the window and its properties via `WindowManager`.
- **Input Management**: Polls and processes input events using `InputManager`.
- **Audio Management**: Manages audio playback with `AudioManager`.
- Initialized through a builder pattern by calling `Engine::new().init()` allowing for customization of:
    - Window properties like size, name, cursor visibility, borders, and resizability.
    - Input polling settings for keys, cursor position, mouse buttons, and scroll.

### Example Usage

```rust
use redgir::{Engine, Key, Action, Color, SpriteId};

fn move_sprite(engine: &mut Engine, sprite_id: SpriteId, dx: i32, dy: i32) {
    if let Some(sprite) = engine.get_sprite(sprite_id) {
        sprite.translate(dx, dy);
    }
}

fn main() {
    let mut engine = Engine::new()
        .set_window_size(1920, 1080)
        .set_clear_color(Color::DARK_GRAY)
        .poll_keyboard()
        .borderless()
        .hide_cursor()
        .init();

    engine.set_fps(144.0);

    // Add a red quad at position (0, 0) on layer 1 with a size of 32x32 pixels
    let quad: Result<SpriteId, _> = engine.add_quad(Color::RED, 0, 0, 1, 32, 32, engine.default_shader().unwrap());
    if let Err(e) = quad {
        panic!("{}", e);
    }
    let quad = quad.unwrap();

    while engine.is_running() {
        let events = engine.get_key_events();
        for (key, action) in events {
            match (key, action) {
                (Key::ArrowUp, Action::Pressed | Action::Held) => move_sprite(&mut engine, quad, 0, 10),
                (Key::ArrowDown, Action::Pressed | Action::Held) => move_sprite(&mut engine, quad, 0, -10),
                (Key::ArrowLeft, Action::Pressed | Action::Held) => move_sprite(&mut engine, quad, -10, 0),
                (Key::ArrowRight, Action::Pressed | Action::Held) => move_sprite(&mut engine, quad, 10, 0),
                (Key::Escape, Action::Pressed) => engine.stop(),
                _ => {},
            }
        }
        engine.draw_frame();
    }
}
```
