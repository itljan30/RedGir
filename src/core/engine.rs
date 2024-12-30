use crate::assets::asset_manager::AssetManager;
use crate::audio::audio_manager::AudioManager;
use crate::input::input_manager::{InputManager, Key, Action};
use crate::video::window::WindowManager;

use std::collections::HashMap;

pub struct Engine {
    asset_manager: AssetManager,
    audio_manager: AudioManager,
    input_manager: InputManager,
    window: WindowManager,
}

impl Engine {
    pub fn init() -> Self {
        let (glfw, window, events) = WindowManager::glfw_init();
        Engine {
            asset_manager: AssetManager::new(),
            audio_manager: AudioManager::new(),
            input_manager: InputManager::new(glfw, events),
            window: WindowManager::new(window),
        }
    }

    pub fn stop(&mut self) {
        self.window.shutdown();
    }

    pub fn get_key_events(&mut self) -> HashMap<Key, Action> {
        return self.input_manager.read_events();
    }

    pub fn is_running(&self) -> bool {
        return self.window.is_running();
    }

    pub fn draw_frame(&self) {
        todo!()
    }

    pub fn display_frame(&self) {
        todo!()
    }
}
