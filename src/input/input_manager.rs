use glfw::{Glfw, GlfwReceiver, WindowEvent};
use std::collections::HashMap;

#[derive(std::cmp::PartialEq)]
#[derive(std::cmp::Eq)]
#[derive(std::fmt::Debug)]
#[derive(Hash)]
#[derive(Clone)]
pub enum Key {
    // Mouse
    MouseLeft, MouseRight, MouseMiddle, MouseScrollUp, MouseScrollDown,

    // Alpha
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, 

    // Non-Alpha Chars
    Period, Comma, ForwardSlash, BackSlash, Space, Equals, Minus, Grave,
    Enter, Escape, Tab, Backspace, LeftBracket, RightBracket, Delete, Apostrophe, SemiColon,

    // Top Row Nums
    Number1, Number2, Number3, Number4, Number5, Number6, Number7, Number8, Number9, Number0,

    // Number Pad
    NumPad1, NumPad2, NumPad3, NumPad4, NumPad5, NumPad6, NumPad7, NumPad8, NumPad9, NumPad0,
    NumPadDecimal, NumPadEquals, NumPadEnter, NumPadMinus, NumPadAdd, NumPadDivide, NumPadMultiply,

    // Modifcation Keys
    LeftShift, RightShift, LeftControl, RightControl, LeftAlt, RightAlt, LeftSuper, RightSuper,
    CapsLock, NumLock, ScrollLock,

    // Navigation Keys
    ArrowRight, ArrowLeft, ArrowDown, ArrowUp, Home, End, PageUp, PageDown, Insert,

    // Function Keys
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,

    None,
}

#[derive(Clone)]
#[derive(std::cmp::PartialEq)]
#[derive(std::cmp::Eq)]
#[derive(std::fmt::Debug)]
pub enum Action {
    Pressed,
    Released,
    Held,
    None,
}

pub struct InputManager {
    key_states: HashMap<Key, Action>,
    glfw_context: Glfw,
    event_listener: GlfwReceiver<(f64, WindowEvent)>,
}

impl InputManager {
    pub fn new(glfw_context: Glfw, event_listener: GlfwReceiver<(f64, WindowEvent)>) -> Self {
        InputManager{
            key_states: HashMap::new(),
            glfw_context,
            event_listener,
        }
    }

    pub fn read_events(&mut self) -> HashMap<Key, Action> {
        self.glfw_context.poll_events();

        for value in self.key_states.values_mut() {
            match *value {
                Action::Pressed => *value = Action::Held,
                Action::Released => *value = Action::None,
                _ => {},
            }
        }
        self.key_states.retain(|_, value| *value != Action::None);

        for (_, event) in glfw::flush_messages(&self.event_listener) {
            let mut action: Action = Action::None;
            let mut key: Key = Key::None;

            match event {
                WindowEvent::Key(_, _, glfw::Action::Press, _) => action = Action::Pressed,
                WindowEvent::Key(_, _, glfw::Action::Release, _) => action = Action::Released,

                WindowEvent::MouseButton(_, glfw::Action::Press, _) => action = Action::Pressed,
                WindowEvent::MouseButton(_, glfw::Action::Release, _) => action = Action::Released,
                _ => {},
            }

            match event {
                // mouse
                WindowEvent::MouseButton(glfw::MouseButton::Button1, _, _) => key = Key::MouseLeft,
                WindowEvent::MouseButton(glfw::MouseButton::Button2, _, _) => key = Key::MouseRight,
                WindowEvent::MouseButton(glfw::MouseButton::Button3, _, _) => key = Key::MouseMiddle,

                // scrolls
                WindowEvent::Scroll(_, y_offset) => if y_offset > 0.0001 {key = Key::MouseScrollUp}
                                                    else if y_offset < -0.0001 {key = Key::MouseScrollDown},

                // alpha
                WindowEvent::Key(glfw::Key::A, _, _, _) => key = Key::A,
                WindowEvent::Key(glfw::Key::B, _, _, _) => key = Key::B,
                WindowEvent::Key(glfw::Key::C, _, _, _) => key = Key::C,
                WindowEvent::Key(glfw::Key::D, _, _, _) => key = Key::D,
                WindowEvent::Key(glfw::Key::E, _, _, _) => key = Key::E,
                WindowEvent::Key(glfw::Key::F, _, _, _) => key = Key::F,
                WindowEvent::Key(glfw::Key::G, _, _, _) => key = Key::G,
                WindowEvent::Key(glfw::Key::H, _, _, _) => key = Key::H,
                WindowEvent::Key(glfw::Key::I, _, _, _) => key = Key::I,
                WindowEvent::Key(glfw::Key::J, _, _, _) => key = Key::J,
                WindowEvent::Key(glfw::Key::K, _, _, _) => key = Key::K,
                WindowEvent::Key(glfw::Key::L, _, _, _) => key = Key::L,
                WindowEvent::Key(glfw::Key::M, _, _, _) => key = Key::M,
                WindowEvent::Key(glfw::Key::N, _, _, _) => key = Key::N,
                WindowEvent::Key(glfw::Key::O, _, _, _) => key = Key::O,
                WindowEvent::Key(glfw::Key::P, _, _, _) => key = Key::P,
                WindowEvent::Key(glfw::Key::Q, _, _, _) => key = Key::Q,
                WindowEvent::Key(glfw::Key::R, _, _, _) => key = Key::R,
                WindowEvent::Key(glfw::Key::S, _, _, _) => key = Key::S,
                WindowEvent::Key(glfw::Key::T, _, _, _) => key = Key::T,
                WindowEvent::Key(glfw::Key::U, _, _, _) => key = Key::U,
                WindowEvent::Key(glfw::Key::V, _, _, _) => key = Key::V,
                WindowEvent::Key(glfw::Key::W, _, _, _) => key = Key::W,
                WindowEvent::Key(glfw::Key::X, _, _, _) => key = Key::X,
                WindowEvent::Key(glfw::Key::Y, _, _, _) => key = Key::Y,
                WindowEvent::Key(glfw::Key::Z, _, _, _) => key = Key::Z,

                // function
                WindowEvent::Key(glfw::Key::F1, _, _, _) => key = Key::F1,
                WindowEvent::Key(glfw::Key::F2, _, _, _) => key = Key::F2,
                WindowEvent::Key(glfw::Key::F3, _, _, _) => key = Key::F3,
                WindowEvent::Key(glfw::Key::F4, _, _, _) => key = Key::F4,
                WindowEvent::Key(glfw::Key::F5, _, _, _) => key = Key::F5,
                WindowEvent::Key(glfw::Key::F6, _, _, _) => key = Key::F6,
                WindowEvent::Key(glfw::Key::F7, _, _, _) => key = Key::F7,
                WindowEvent::Key(glfw::Key::F8, _, _, _) => key = Key::F8,
                WindowEvent::Key(glfw::Key::F9, _, _, _) => key = Key::F9,
                WindowEvent::Key(glfw::Key::F10, _, _, _) => key = Key::F10,
                WindowEvent::Key(glfw::Key::F11, _, _, _) => key = Key::F11,
                WindowEvent::Key(glfw::Key::F12, _, _, _) => key = Key::F12,

                // mod
                WindowEvent::Key(glfw::Key::LeftControl, _, _, _) => key = Key::LeftControl,
                WindowEvent::Key(glfw::Key::RightControl, _, _, _) => key = Key::RightControl,
                WindowEvent::Key(glfw::Key::LeftAlt, _, _, _) => key = Key::LeftAlt,
                WindowEvent::Key(glfw::Key::RightAlt, _, _, _) => key = Key::RightAlt,
                WindowEvent::Key(glfw::Key::LeftShift, _, _, _) => key = Key::LeftShift,
                WindowEvent::Key(glfw::Key::RightShift, _, _, _) => key = Key::RightShift,
                WindowEvent::Key(glfw::Key::LeftSuper, _, _, _) => key = Key::LeftSuper,
                WindowEvent::Key(glfw::Key::RightSuper, _, _, _) => key = Key::RightSuper,
                WindowEvent::Key(glfw::Key::CapsLock, _, _, _) => key = Key::CapsLock,
                WindowEvent::Key(glfw::Key::NumLock, _, _, _) => key = Key::NumLock,
                WindowEvent::Key(glfw::Key::ScrollLock, _, _, _) => key = Key::ScrollLock,

                // nums
                WindowEvent::Key(glfw::Key::Num0, _, _, _) => key = Key::Number0,
                WindowEvent::Key(glfw::Key::Num1, _, _, _) => key = Key::Number1,
                WindowEvent::Key(glfw::Key::Num2, _, _, _) => key = Key::Number2,
                WindowEvent::Key(glfw::Key::Num3, _, _, _) => key = Key::Number3,
                WindowEvent::Key(glfw::Key::Num4, _, _, _) => key = Key::Number4,
                WindowEvent::Key(glfw::Key::Num5, _, _, _) => key = Key::Number5,
                WindowEvent::Key(glfw::Key::Num6, _, _, _) => key = Key::Number6,
                WindowEvent::Key(glfw::Key::Num7, _, _, _) => key = Key::Number7,
                WindowEvent::Key(glfw::Key::Num8, _, _, _) => key = Key::Number8,
                WindowEvent::Key(glfw::Key::Num9, _, _, _) => key = Key::Number9,

                // numpad
                WindowEvent::Key(glfw::Key::Kp0, _, _, _) => key = Key::NumPad0,
                WindowEvent::Key(glfw::Key::Kp1, _, _, _) => key = Key::NumPad1,
                WindowEvent::Key(glfw::Key::Kp2, _, _, _) => key = Key::NumPad2,
                WindowEvent::Key(glfw::Key::Kp3, _, _, _) => key = Key::NumPad3,
                WindowEvent::Key(glfw::Key::Kp4, _, _, _) => key = Key::NumPad4,
                WindowEvent::Key(glfw::Key::Kp5, _, _, _) => key = Key::NumPad5,
                WindowEvent::Key(glfw::Key::Kp6, _, _, _) => key = Key::NumPad6,
                WindowEvent::Key(glfw::Key::Kp7, _, _, _) => key = Key::NumPad7,
                WindowEvent::Key(glfw::Key::Kp8, _, _, _) => key = Key::NumPad8,
                WindowEvent::Key(glfw::Key::Kp9, _, _, _) => key = Key::NumPad9,
                WindowEvent::Key(glfw::Key::KpMultiply, _, _, _) => key = Key::NumPadMultiply,
                WindowEvent::Key(glfw::Key::KpDivide, _, _, _) => key = Key::NumPadDivide,
                WindowEvent::Key(glfw::Key::KpAdd, _, _, _) => key = Key::NumPadAdd,
                WindowEvent::Key(glfw::Key::KpSubtract, _, _, _) => key = Key::NumPadMinus,
                WindowEvent::Key(glfw::Key::KpEqual, _, _, _) => key = Key::NumPadEquals,
                WindowEvent::Key(glfw::Key::KpEnter, _, _, _) => key = Key::NumPadEnter,
                WindowEvent::Key(glfw::Key::KpDecimal, _, _, _) => key = Key::NumPadDecimal,

                // movement
                WindowEvent::Key(glfw::Key::Up, _, _, _) => key = Key::ArrowUp,
                WindowEvent::Key(glfw::Key::Down, _, _, _) => key = Key::ArrowDown,
                WindowEvent::Key(glfw::Key::Left, _, _, _) => key = Key::ArrowLeft,
                WindowEvent::Key(glfw::Key::Right, _, _, _) => key = Key::ArrowRight,
                WindowEvent::Key(glfw::Key::PageUp, _, _, _) => key = Key::PageUp,
                WindowEvent::Key(glfw::Key::PageDown, _, _, _) => key = Key::PageDown,
                WindowEvent::Key(glfw::Key::Home, _, _, _) => key = Key::Home,
                WindowEvent::Key(glfw::Key::End, _, _, _) => key = Key::End,
                WindowEvent::Key(glfw::Key::Insert, _, _, _) => key = Key::Insert,

                // non-aplha chars
                WindowEvent::Key(glfw::Key::Escape, _, _, _) => key = Key::Escape,
                WindowEvent::Key(glfw::Key::Tab, _, _, _) => key = Key::Tab,
                WindowEvent::Key(glfw::Key::Backspace, _, _, _) => key = Key::Backspace,
                WindowEvent::Key(glfw::Key::Enter, _, _, _) => key = Key::Enter,
                WindowEvent::Key(glfw::Key::Space, _, _, _) => key = Key::Space,
                WindowEvent::Key(glfw::Key::Backslash, _, _, _) => key = Key::BackSlash,
                WindowEvent::Key(glfw::Key::Slash, _, _, _) => key = Key::ForwardSlash,
                WindowEvent::Key(glfw::Key::Comma, _, _, _) => key = Key::Comma,
                WindowEvent::Key(glfw::Key::GraveAccent, _, _, _) => key = Key::Grave,
                WindowEvent::Key(glfw::Key::Minus, _, _, _) => key = Key::Minus,
                WindowEvent::Key(glfw::Key::Equal, _, _, _) => key = Key::Equals,
                WindowEvent::Key(glfw::Key::Period, _, _, _) => key = Key::Period,
                WindowEvent::Key(glfw::Key::LeftBracket, _, _, _) => key = Key::LeftBracket,
                WindowEvent::Key(glfw::Key::RightBracket, _, _, _) => key = Key::RightBracket,
                WindowEvent::Key(glfw::Key::Delete, _, _, _) => key = Key::Delete,
                WindowEvent::Key(glfw::Key::Apostrophe, _, _, _) => key = Key::Apostrophe,
                WindowEvent::Key(glfw::Key::Semicolon, _, _, _) => key = Key::SemiColon,

                _ => {},
            }

            if key != Key::None && action != Action::None {
                self.key_states.insert(key, action);
            }
        }
        self.key_states.clone()
    }
}
