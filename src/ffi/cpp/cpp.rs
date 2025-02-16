#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::engine::{EngineBuilder, Engine};
use crate::video::sprite::{Flip, SpriteSheetError, Sprite, SpriteSheet, SpriteId, SpriteSheetId};
use crate::video::shader_manager::{
    Attribute, Uniform, UniformData, AttributeData, DEFAULT_VERTEX_SHADER, DEFAULT_FRAGMENT_SHADER,
    ShaderId, VertexShader, FragmentShader, ShaderProgram, ShaderError, 
};
use crate::video::color::Color;
use crate::utility::timer::Timer;
use crate::video::window::WindowManager;
use crate::input::input_manager::InputManager;

use std::ffi::CStr;
use std::os::raw::c_char;

#[repr(C)]
pub struct EngineC {
    engine: Engine,
}

impl EngineC {
    pub fn new() -> EngineBuilderC {
        EngineBuilderC {
            engine_builder: Engine::new()
        }
    }

    pub fn init(engine_builder: &mut EngineBuilderC) -> Self {
        todo!()
    }
}

// Method chaining requires it to be heap allocated for 
#[repr(C)]
pub struct EngineBuilderC {
    engine_builder: EngineBuilder,
}

impl EngineBuilderC {
    pub fn init(&mut self) {
        todo!()
    }

    pub fn hide_cursor(&mut self) {
        self.engine_builder.hide_cursor();
    }

    pub fn borderless(&mut self) {
        self.engine_builder.borderless();
    }

    pub fn not_resizable(&mut self) {
        self.engine_builder.not_resizable();
    }

    pub fn poll_mouse_buttons(&mut self) {
        self.engine_builder.poll_mouse_buttons();
    }

    pub fn poll_cursor(&mut self) {
        self.engine_builder.poll_cursor();
    }

    pub fn poll_keyboard(&mut self) {
        self.engine_builder.poll_keyboard();
    }

    pub fn set_window_size(&mut self, width: u32, height: u32) {
        self.engine_builder.set_window_size(width, height);
    }

    pub fn set_clear_color(&mut self, color: Color) {
        self.engine_builder.set_clear_color(color);
    }

    pub fn set_window_name(&mut self, name: &str) {
        self.engine_builder.set_window_name(name);
    }
}

#[no_mangle]
pub extern "C" fn EngineBuilderC_init(_engine_builder: *mut EngineBuilderC) {
    todo!()
}

#[no_mangle]
pub extern "C" fn EngineBuilderC_hideCursor(engine_builder: *mut EngineBuilderC) {
    if let Some(engine_builder) = unsafe { engine_builder.as_mut() } {
        engine_builder.hide_cursor();
    }
}

#[no_mangle]
pub extern "C" fn EngineBuilderC_borderless(engine_builder: *mut EngineBuilderC) {
    if let Some(engine_builder) = unsafe { engine_builder.as_mut() } {
        engine_builder.borderless();
    }

}

#[no_mangle]
pub extern "C" fn EngineBuilderC_notResizable(engine_builder: *mut EngineBuilderC) {
    if let Some(engine_builder) = unsafe { engine_builder.as_mut() } {
        engine_builder.not_resizable();
    }

}

#[no_mangle]
pub extern "C" fn EngineBuilderC_pollMouseButtons(engine_builder: *mut EngineBuilderC) {
    if let Some(engine_builder) = unsafe { engine_builder.as_mut() } {
        engine_builder.poll_mouse_buttons();
    }

}

#[no_mangle]
pub extern "C" fn EngineBuilderC_pollCursor(engine_builder: *mut EngineBuilderC) {
    if let Some(engine_builder) = unsafe { engine_builder.as_mut() } {
        engine_builder.poll_cursor();
    }

}

#[no_mangle]
pub extern "C" fn EngineBuilderC_pollKeyboard(engine_builder: *mut EngineBuilderC) {
    if let Some(engine_builder) = unsafe { engine_builder.as_mut() } {
        engine_builder.poll_keyboard();
    }

}

#[no_mangle]
pub extern "C" fn EngineBuilderC_set_windowSize(engine_builder: *mut EngineBuilderC, width: u32, height: u32) {
    if let Some(engine_builder) = unsafe { engine_builder.as_mut() } {
        engine_builder.set_window_size(width, height);
    }

}

#[no_mangle]
pub extern "C" fn EngineBuilderC_setClearColor(engine_builder: *mut EngineBuilderC, r: u8, g: u8, b: u8, a: u8) {
    if let Some(engine_builder) = unsafe { engine_builder.as_mut() } {
        engine_builder.set_clear_color(Color::new(r, g, b, a));
    }

}

#[no_mangle]
pub extern "C" fn EngineBuilderC_setWindowName(engine_builder: *mut EngineBuilderC, name: *const c_char) {
    if let Some(engine_builder) = unsafe { engine_builder.as_mut() } {
        if let Ok(name) = unsafe { CStr::from_ptr(name).to_str() } {
            engine_builder.set_window_name(name);
        }
    }
}

#[repr(C)]
pub struct TimerC {
    timer: Timer,
}

impl TimerC {
    pub fn new() -> Self {
        Self {
            timer: Timer::new(),
        }
    }

    pub fn reset(&mut self) {
        self.timer.reset();
    }

    pub fn getElapsedSeconds(&self) -> f32 {
        self.timer.get_elapsed_seconds()
    }

    pub fn getElapsedMilis(&self) -> u128 {
        self.timer.get_elapsed_milis()
    }
}

#[no_mangle]
pub extern "C" fn TimerC_reset(timer: *mut TimerC) {
    if let Some(timer) = unsafe { timer.as_mut() } {
        timer.reset();
    }
}

#[no_mangle]
pub extern "C" fn TimerC_getElapsedSeconds(timer: *const TimerC) -> f32 {
    if let Some(timer) = unsafe { timer.as_ref() } {
        timer.getElapsedSeconds()
    }
    else {
        0.0
    }
}

#[no_mangle]
pub extern "C" fn TimerC_getElapsedMilis(timer: *const TimerC) -> u64 {
    if let Some(timer) = unsafe { timer.as_ref() } {
        timer.getElapsedMilis() as u64
    }
    else {
        0
    }
}
