#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::engine::{EngineBuilder, Engine, GetId};
use crate::video::sprite::{Flip, SpriteSheetError, Sprite, SpriteSheet, SpriteId, SpriteSheetId};
use crate::video::shader_manager::{
    Attribute, Uniform, UniformData, AttributeData, DEFAULT_VERTEX_SHADER, DEFAULT_FRAGMENT_SHADER,
    ShaderId, VertexShader, FragmentShader, ShaderProgram, ShaderError, 
};
use crate::video::color::Color;
use crate::utility::timer::Timer;
use crate::video::window::WindowManager;
use crate::input::input_manager::{InputManager, Key, Action};

use std::ffi::CStr;
use std::os::raw::c_char;
use std::collections::HashMap;
use std::ptr::null_mut;

#[repr(C)]
pub struct WindowDimensions {
    x: i32,
    y: i32,
}

impl Default for WindowDimensions {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }
}

#[repr(C)]
pub struct EngineBuilderC {
    engine_builder: *mut EngineBuilder,
}

#[no_mangle]
pub extern "C" fn EngineBuilderC_init(engine_builder: *mut EngineBuilderC) -> *mut EngineC {
    unsafe {
        if let Some(engine_builder) = engine_builder.as_mut().and_then(|e| e.engine_builder.as_mut()) {
            let engine = Box::new(engine_builder.init());
            return Box::into_raw(Box::new(
                EngineC {
                    engine: Box::into_raw(engine),
                }
            ));
        }
        null_mut()
    }
}

#[no_mangle]
pub extern "C" fn EngineBuilderC_hideCursor(engine_builder: *mut EngineBuilderC) {
    unsafe {
        if let Some(engine_builder) = engine_builder.as_mut().and_then(|e| e.engine_builder.as_mut()) {
            engine_builder.hide_cursor();
        }
    }
}

#[no_mangle]
pub extern "C" fn EngineBuilderC_borderless(engine_builder: *mut EngineBuilderC) {
    unsafe {
        if let Some(engine_builder) = engine_builder.as_mut().and_then(|e| e.engine_builder.as_mut()) {
            engine_builder.borderless();
        }
    }
}

#[no_mangle]
pub extern "C" fn EngineBuilderC_notResizable(engine_builder: *mut EngineBuilderC) {
    unsafe {
        if let Some(engine_builder) = engine_builder.as_mut().and_then(|e| e.engine_builder.as_mut()) {
            engine_builder.not_resizable();
        }
    }
}

#[no_mangle]
pub extern "C" fn EngineBuilderC_pollMouseButtons(engine_builder: *mut EngineBuilderC) {
    unsafe {
        if let Some(engine_builder) = engine_builder.as_mut().and_then(|e| e.engine_builder.as_mut()) {
            engine_builder.poll_mouse_buttons();
        }
    }
}

#[no_mangle]
pub extern "C" fn EngineBuilderC_pollCursor(engine_builder: *mut EngineBuilderC) {
    unsafe {
        if let Some(engine_builder) = engine_builder.as_mut().and_then(|e| e.engine_builder.as_mut()) {
            engine_builder.poll_cursor();
        }
    }
}

#[no_mangle]
pub extern "C" fn EngineBuilderC_pollKeyboard(engine_builder: *mut EngineBuilderC) {
    unsafe {
        if let Some(engine_builder) = engine_builder.as_mut().and_then(|e| e.engine_builder.as_mut()) {
            engine_builder.poll_keyboard();
        }
    }
}

#[no_mangle]
pub extern "C" fn EngineBuilderC_setWindowSize(engine_builder: *mut EngineBuilderC, width: u32, height: u32) {
    unsafe {
        if let Some(engine_builder) = engine_builder.as_mut().and_then(|e| e.engine_builder.as_mut()) {
            engine_builder.set_window_size(width, height);
        }
    }
}

#[no_mangle]
pub extern "C" fn EngineBuilderC_setClearColor(engine_builder: *mut EngineBuilderC, r: u8, g: u8, b: u8, a: u8) {
    unsafe {
        if let Some(engine_builder) = engine_builder.as_mut().and_then(|e| e.engine_builder.as_mut()) {
            engine_builder.set_clear_color(Color::new(r, g, b, a));
        }
    }
}

#[no_mangle]
pub extern "C" fn EngineBuilderC_setWindowName(engine_builder: *mut EngineBuilderC, name: *const c_char) {
    unsafe {
        if let Ok(name) = CStr::from_ptr(name).to_str() {
            if let Some(engine_builder) = engine_builder.as_mut().and_then(|e| e.engine_builder.as_mut()) {
                engine_builder.set_window_name(name);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn EngineBuilderC_free(engine_builder: *mut EngineBuilderC) {
    if engine_builder.is_null() {
        return
    }

    unsafe {
        if let Some(engine_builder) = engine_builder.as_mut().and_then(|e| e.engine_builder.as_mut()) {
            drop(Box::from_raw(engine_builder));
        }
        drop(Box::from_raw(engine_builder));
    }
}

#[repr(C)]
pub struct EngineC {
    engine: *mut Engine,
}

#[no_mangle]
pub extern "C" fn EngineC_new() -> *mut EngineBuilderC {
    Box::into_raw(Box::new(
        EngineBuilderC {
            engine_builder: Box::into_raw(Box::new(Engine::new())),
        }
    ))
}

pub extern "C" fn EngineC_defaultShader(engine: *mut EngineC) -> u32 {
    unsafe {
        if let Some(engine) = engine.as_ref().and_then(|e| e.engine.as_ref()) {
            match engine.default_shader() {
                Some(shader_id) => shader_id.id(),
                None => u32::MAX,
            }
        } else { u32::MAX }
    }
}

pub extern "C" fn EngineC_addQuad(
    engine: *mut EngineC,
    r: u8, g: u8, b: u8, a: u8,
    x_position: i32, y_position: i32,
    layer: i32, width: u32, height: u32,
    shader: u32) -> u32 {
    unsafe {
        if let Some(engine) = engine.as_mut().and_then(|e| e.engine.as_mut()) {
            match engine.add_quad(Color::new(r, g, b, a), x_position, y_position, layer, width, height, shader.into()) {
                Ok(val) => val.id(),
                Err(_) => u32::MAX,
            }
        } else { u32::MAX }
    }
}

pub extern "C" fn EngineC_timeSinceInitializationMilis(engine: *const EngineC) -> u64 {
    unsafe {
        if let Some(engine) = engine.as_ref().and_then(|e| e.engine.as_ref()) {
            engine.time_since_initialization_milis() as u64
        } else { 0 }
    }
}

pub extern "C" fn EngineC_timeSinceInitializationSeconds(engine: *const EngineC) -> f32 {
    unsafe {
        if let Some(engine) = engine.as_ref().and_then(|e| e.engine.as_ref()) {
            engine.time_since_initialization_seconds()
        } else { 0.0 }
    }
}

pub extern "C" fn EngineC_getWindowDimensions(engine: *const EngineC) -> WindowDimensions {
    unsafe {
        if let Some(engine) = engine.as_ref().and_then(|e| e.engine.as_ref()) {
            let (x, y) = engine.get_window_dimensions();
            WindowDimensions { x, y }
        } else { WindowDimensions::default() }
    }
}

pub extern "C" fn EngineC_addSpriteSheet(
    engine: *mut EngineC,
    path: *const c_char,
    sprite_width: u32,
    sprite_height: u32,
) -> u32 {
    unsafe {
        if let Some(engine) = engine.as_mut().and_then(|e| e.engine.as_mut()) {
            if let Ok(path) = CStr::from_ptr(path).to_str() {
                match engine.add_sprite_sheet(path, sprite_width, sprite_height) {
                    Ok(val) => val.id(),
                    Err(_) => u32::MAX,
                }
            } else { u32::MAX }
        } else { u32::MAX }
    }
}

pub extern "C" fn EngineC_addShaderProgram() {} // TODO

pub extern "C" fn EngineC_getSprite() {} // TODO








#[repr(C)]
pub struct TimerC {
    timer: *mut Timer,
}

#[no_mangle]
pub extern "C" fn TimerC_new() -> *mut TimerC {
    Box::into_raw(Box::new(
        TimerC {
            timer: Box::into_raw(Box::new(Timer::new())),
        }
    ))
}

#[no_mangle]
pub extern "C" fn TimerC_reset(timer: *mut TimerC) {
    unsafe {
        if let Some(timer) = timer.as_mut().and_then(|t| t.timer.as_mut()) {
            timer.reset();
        }
    }
}

#[no_mangle]
pub extern "C" fn TimerC_getElapsedSeconds(timer: *const TimerC) -> f32 {
    unsafe {
        if let Some(timer) = timer.as_ref().and_then(|t| t.timer.as_ref()) {
            timer.get_elapsed_seconds()
        } else { 0.0 }
    }
}

#[no_mangle]
pub extern "C" fn TimerC_getElapsedMilis(timer: *const TimerC) -> u64 {
    unsafe {
        if let Some(timer) = timer.as_ref().and_then(|t| t.timer.as_ref()) {
            timer.get_elapsed_milis() as u64
        } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn TimerC_free(timer: *mut TimerC) {
    if timer.is_null() {
        return;
    }

    unsafe {
        if let Some(timer) = timer.as_mut().and_then(|t| t.timer.as_mut()) {
            drop(Box::from_raw(timer));
        }
        drop(Box::from_raw(timer));
    }
}
