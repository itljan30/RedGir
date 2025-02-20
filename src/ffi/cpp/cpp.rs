#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::engine::{EngineBuilder, Engine, GetId};
use crate::video::sprite::{Flip, Sprite, SpriteSheet};
use crate::video::shader_manager::{
    Attribute, AttributeData, Uniform, UniformData, VertexShader, FragmentShader, ShaderProgram
};
use crate::video::color::Color;
use crate::utility::timer::Timer;
use crate::input::input_manager::{Key, Action};

use std::ffi::CStr;
use std::os::raw::c_char;
use std::collections::HashMap;
use std::ptr::null_mut;

macro_rules! impl_from_trait_for_identical_enums {
    ($original:ident, $ffi:ident, $( $variant:ident ),* ) => {
        impl From<$original> for $ffi {
            fn from(value: $original) -> Self {
                match value {
                    $( $original::$variant => $ffi::$variant, )*
                }
            }
        }
        
        impl From<$ffi> for $original {
            fn from(value: $ffi) -> Self {
                match value {
                    $( $ffi::$variant => $original::$variant, )*
                }
            }
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KeyC {
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

impl_from_trait_for_identical_enums!{
    KeyC, Key, MouseLeft, MouseRight, MouseMiddle, MouseScrollUp, MouseScrollDown,
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, 
    Period, Comma, ForwardSlash, BackSlash, Space, Equals, Minus, Grave,
    Enter, Escape, Tab, Backspace, LeftBracket, RightBracket, Delete, Apostrophe, SemiColon,
    Number1, Number2, Number3, Number4, Number5, Number6, Number7, Number8, Number9, Number0,
    NumPad1, NumPad2, NumPad3, NumPad4, NumPad5, NumPad6, NumPad7, NumPad8, NumPad9, NumPad0,
    NumPadDecimal, NumPadEquals, NumPadEnter, NumPadMinus, NumPadAdd, NumPadDivide, NumPadMultiply,
    LeftShift, RightShift, LeftControl, RightControl, LeftAlt, RightAlt, LeftSuper, RightSuper,
    CapsLock, NumLock, ScrollLock,
    ArrowRight, ArrowLeft, ArrowDown, ArrowUp, Home, End, PageUp, PageDown, Insert,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    None
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ActionC {
    Pressed,
    Released,
    Held,
    None,
}

impl Default for ActionC {
    fn default() -> Self {
        Self::None
    }
}

impl_from_trait_for_identical_enums!{ActionC, Action, Pressed, Released, Held, None}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum FlipC {
    None,
    FlipX,
    FlipY,
    FlipXY,
}

impl_from_trait_for_identical_enums!{FlipC, Flip, None, FlipX, FlipY, FlipXY}

#[repr(C)]
pub struct SpriteC {
    sprite: *mut Sprite,
}

#[no_mangle]
pub extern "C" fn SpriteC_new(
    sprite_sheet: u32,
    sprite_sheet_index: usize,
    x_position: i32,
    y_position: i32,
    layer: i32,
    width: u32,
    height: u32,
    shader: u32,
) -> *mut SpriteC {
    Box::into_raw(Box::new(
        SpriteC {
            sprite: Box::into_raw(Box::new(
                Sprite::new(
                    sprite_sheet.into(), sprite_sheet_index,
                    x_position, y_position, layer,
                    width, height, shader.into()
                )
            ))
        }
    ))
}

#[no_mangle]
pub extern "C" fn SpriteC_free(sprite: *mut SpriteC) {
    unsafe {
        if let Some(sprite_c) = sprite.as_mut() {
            if let Some(inner_sprite) = sprite_c.sprite.as_mut() {
                drop(Box::from_raw(inner_sprite));
            }
            drop(Box::from_raw(sprite_c));
        }
    }
}

#[no_mangle]
pub extern "C" fn SpriteC_getRotation(sprite: *const SpriteC) -> f32 {
    unsafe {
        if let Some(sprite) = sprite.as_ref().and_then(|s| s.sprite.as_ref()) {
            sprite.get_rotation()
        } else { 0.0 }
    }
}

#[no_mangle]
pub extern "C" fn SpriteC_getFlip(sprite: *const SpriteC) -> FlipC {
    unsafe {
        if let Some(sprite) = sprite.as_ref().and_then(|s| s.sprite.as_ref()) {
            sprite.get_flip().into()
        } else { FlipC::None }
    }
}

#[no_mangle]
pub extern "C" fn SpriteC_getSpriteSheetIndex(sprite: *const SpriteC) -> usize {
    unsafe {
        if let Some(sprite) = sprite.as_ref().and_then(|s| s.sprite.as_ref()) {
            sprite.get_sprite_sheet_index()
        } else { u32::MAX as usize }
    }
}

#[no_mangle]
pub extern "C" fn SpriteC_getSpriteSheet(sprite: *const SpriteC) -> u32 {
    unsafe {
        if let Some(sprite) = sprite.as_ref().and_then(|s| s.sprite.as_ref()) {
            sprite.get_sprite_sheet().id()
        } else { u32::MAX }
    }
}

#[repr(C)]
pub struct PositionC {
    x: i32,
    y: i32,
}

impl Default for PositionC {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }
}

#[no_mangle]
pub extern "C" fn SpriteC_getPosition(sprite: *const SpriteC) -> PositionC {
    unsafe {
        if let Some(sprite) = sprite.as_ref().and_then(|s| s.sprite.as_ref()) {
            let (x, y) = sprite.get_position();
            PositionC {x, y}
        } else { PositionC::default() }
    }
}

#[no_mangle]
pub extern "C" fn SpriteC_translate(sprite: *mut SpriteC, dx: i32, dy: i32) {
    unsafe {
        if let Some(sprite) = sprite.as_mut().and_then(|s| s.sprite.as_mut()) {
            sprite.translate(dx, dy);
        }
    }
}

#[no_mangle]
pub extern "C" fn SpriteC_setId(sprite: *mut SpriteC, id: u32) {
    unsafe {
        if let Some(sprite) = sprite.as_mut().and_then(|s| s.sprite.as_mut()) {
            sprite.set_id(id);
        }
    }
}

#[no_mangle]
pub extern "C" fn SpriteC_getShader(sprite: *const SpriteC) -> u32 {
    unsafe {
        if let Some(sprite) = sprite.as_ref().and_then(|s| s.sprite.as_ref()) {
            sprite.get_shader().id()
        } else { u32::MAX }
    }
}

#[no_mangle]
pub extern "C" fn SpriteC_setShader(sprite: *mut SpriteC, shader: u32) {
    unsafe {
        if let Some(sprite) = sprite.as_mut().and_then(|s| s.sprite.as_mut()) {
            sprite.set_shader(shader.into());
        }
    }
}

#[no_mangle]
pub extern "C" fn SpriteC_setTexture(sprite: *mut SpriteC, sprite_sheet: u32, index: usize) {
    unsafe {
        if let Some(sprite) = sprite.as_mut().and_then(|s| s.sprite.as_mut()) {
            sprite.set_texture(sprite_sheet.into(), index);
        }
    }
}

#[no_mangle]
pub extern "C" fn SpriteC_setHeight(sprite: *mut SpriteC, height: u32) {
    unsafe {
        if let Some(sprite) = sprite.as_mut().and_then(|s| s.sprite.as_mut()) {
            sprite.set_height(height);
        }
    }
}

#[no_mangle]
pub extern "C" fn SpriteC_setWidth(sprite: *mut SpriteC, width: u32) {
    unsafe {
        if let Some(sprite) = sprite.as_mut().and_then(|s| s.sprite.as_mut()) {
            sprite.set_width(width);
        }
    }
}

#[no_mangle]
pub extern "C" fn SpriteC_setPosition(sprite: *mut SpriteC, x: i32, y: i32) {
    unsafe {
        if let Some(sprite) = sprite.as_mut().and_then(|s| s.sprite.as_mut()) {
            sprite.set_position(x, y);
        }
    }
}

#[no_mangle]
pub extern "C" fn SpriteC_scale(sprite: *mut SpriteC, scale_x: f32, scale_y: f32) {
    unsafe {
        if let Some(sprite) = sprite.as_mut().and_then(|s| s.sprite.as_mut()) {
            sprite.scale(scale_x, scale_y);
        }
    }
}

#[no_mangle]
pub extern "C" fn SpriteC_setRotation(sprite: *mut SpriteC, rotation: f32) {
    unsafe {
        if let Some(sprite) = sprite.as_mut().and_then(|s| s.sprite.as_mut()) {
            sprite.set_rotation(rotation);
        }
    }
}

#[no_mangle]
pub extern "C" fn SpriteC_setFlip(sprite: *mut SpriteC, flip: FlipC) {
    unsafe {
        if let Some(sprite) = sprite.as_mut().and_then(|s| s.sprite.as_mut()) {
            sprite.set_flip(flip.into());
        }
    }
}

#[no_mangle]
pub extern "C" fn SpriteC_getLayer(sprite: *const SpriteC) -> i32 {
    unsafe {
        if let Some(sprite) = sprite.as_ref().and_then(|s| s.sprite.as_ref()) {
            sprite.get_layer()
        } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn SpriteC_getHeight(sprite: *const SpriteC) -> u32 {
    unsafe {
        if let Some(sprite) = sprite.as_ref().and_then(|s| s.sprite.as_ref()) {
            sprite.get_height()
        } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn SpriteC_getWidth(sprite: *const SpriteC) -> u32 {
    unsafe {
        if let Some(sprite) = sprite.as_ref().and_then(|s| s.sprite.as_ref()) {
            sprite.get_width()
        } else { 0 }
    }
}

#[repr(C)]
pub struct UVCoordsC {
    min_u: f32,
    min_v: f32,
    max_u: f32,
    max_v: f32,
}

impl Default for UVCoordsC {
    fn default() -> Self {
        Self {
            min_u: 0.0,
            min_v: 0.0,
            max_u: 0.0,
            max_v: 0.0,
        }
    }
}
 
#[repr(C)]
pub struct SpriteSheetC {
    sprite_sheet: *mut SpriteSheet,
}

#[no_mangle]
pub extern "C" fn SpriteSheetC_free(sheet: *mut SpriteSheetC) {
    unsafe {
        if let Some(sheet_c) = sheet.as_mut() {
            if let Some(inner_sheet) = sheet_c.sprite_sheet.as_mut() {
                drop(Box::from_raw(inner_sheet));
            }
            drop(Box::from_raw(sheet_c));
        }
    }
}

#[no_mangle]
pub extern "C" fn SpriteSheetC_getUV(sheet: *const SpriteSheetC, index: usize) -> UVCoordsC {
    unsafe {
        if let Some(sheet) = sheet.as_ref().and_then(|s| s.sprite_sheet.as_ref()) {
            let (min_u, min_v, max_u, max_v) = sheet.get_uv(index);
            UVCoordsC {
                min_u,
                min_v,
                max_u,
                max_v,
            }
        } else { UVCoordsC::default() }
    }
}

#[no_mangle]
pub extern "C" fn SpriteSheetC_getTexture(sheet: *const SpriteSheetC) -> u32 {
    unsafe {
        if let Some(sheet) = sheet.as_ref().and_then(|s| s.sprite_sheet.as_ref()) {
            sheet.get_texture()
        } else { u32::MAX }
    }
}

#[no_mangle]
pub extern "C" fn SpriteSheetC_fromImage(path: *const c_char, sprite_width: u32, sprite_height: u32) -> *mut SpriteSheetC {
    unsafe {
        if let Ok(path) = CStr::from_ptr(path).to_str() {
            Box::into_raw(Box::new(
                SpriteSheetC {
                    sprite_sheet: Box::into_raw(Box::new(
                        SpriteSheet::from_image(path, sprite_width, sprite_height).unwrap_or_default()
                    ))
                }
            ))
        } else { null_mut() } 
    }
}

#[no_mangle]
pub extern "C" fn SpriteSheetC_fromColor(r: u8, g: u8, b: u8, a: u8) -> *mut SpriteSheetC {
    Box::into_raw(Box::new(
        SpriteSheetC {
            sprite_sheet: Box::into_raw(Box::new(
                SpriteSheet::from_color(Color::new(r, g, b, a)).unwrap_or_default()
            )),
        }
    ))
}

#[repr(C)]
pub enum AttributeDataTypeC {
    Float,
    FloatVec2,
    FloatVec3,
    FloatVec4,
    Int,
    Bool,
    UInt,
}

#[repr(C)]
pub union AttributeDataValueC {
    float:     [f32; 4],
    floatVec2: [[f32; 2]; 4],
    floatVec3: [[f32; 3]; 4],
    floatVec4: [[f32; 4]; 4],
    int:       [i32; 4],
    bool:      [bool; 4],
    uInt:      [u32; 4],
}

#[repr(C)]
pub struct AttributeDataC {
    kind: AttributeDataTypeC,
    func: extern "C" fn(*const EngineC, *const SpriteC) -> AttributeDataValueC,
}

#[repr(C)]
pub enum UniformDataTypeC {
    Float,
    FloatVec2,
    FloatVec3,
    FloatVec4,
    FloatMat2,
    FloatMat3,
    FloatMat4,
    FloatMat2x3,
    FloatMat2x4,
    FloatMat3x2,
    FloatMat3x4,
    FloatMat4x2,
    FloatMat4x3,
    Int,
    Bool,
    UInt,
    Sampler2D,
}

#[repr(C)]
pub union UniformDataValueC {
    float:       f32,
    floatVec2:   [f32; 2],
    floatVec3:   [f32; 3],
    floatVec4:   [f32; 4],
    floatMat2:   [[f32; 2]; 2],
    floatMat3:   [[f32; 3]; 3],
    floatMat4:   [[f32; 4]; 4],
    floatMat2x3: [[f32; 3]; 2],
    floatMat2x4: [[f32; 4]; 2],
    floatMat3x2: [[f32; 2]; 3],
    floatMat3x4: [[f32; 4]; 3],
    floatMat4x2: [[f32; 2]; 4],
    floatMat4x3: [[f32; 3]; 4],
    int:         i32,
    bool:        bool,
    uInt:        u32,
    sampler2D:   u32,
}

#[repr(C)]
pub struct UniformDataC {
    kind: UniformDataTypeC,
    func: extern "C" fn(*const EngineC, *const SpriteC) -> UniformDataValueC,
}

#[repr(C)]
pub struct WindowDimensionsC {
    x: i32,
    y: i32,
}

impl Default for WindowDimensionsC {
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

#[no_mangle]
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

#[no_mangle]
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

#[no_mangle]
pub extern "C" fn EngineC_timeSinceInitializationMilis(engine: *const EngineC) -> u64 {
    unsafe {
        if let Some(engine) = engine.as_ref().and_then(|e| e.engine.as_ref()) {
            engine.time_since_initialization_milis() as u64
        } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn EngineC_timeSinceInitializationSeconds(engine: *const EngineC) -> f32 {
    unsafe {
        if let Some(engine) = engine.as_ref().and_then(|e| e.engine.as_ref()) {
            engine.time_since_initialization_seconds()
        } else { 0.0 }
    }
}

#[no_mangle]
pub extern "C" fn EngineC_getWindowDimensions(engine: *const EngineC) -> WindowDimensionsC {
    unsafe {
        if let Some(engine) = engine.as_ref().and_then(|e| e.engine.as_ref()) {
            let (x, y) = engine.get_window_dimensions();
            WindowDimensionsC { x, y }
        } else { WindowDimensionsC::default() }
    }
}

#[no_mangle]
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

#[no_mangle]
pub extern "C" fn EngineC_addShaderProgram(
    // vertex_shader: u32,
    // fragment_shader: u32,
    // attributes: Vec<AttributeC>,
    // global_uniforms: Vec<UniformC>,
    // instance_uniforms: Vec<UniformC>,
) {} // TODO

#[no_mangle]
pub extern "C" fn EngineC_getSprite(engine: *mut EngineC, sprite_id: u32) -> *mut SpriteC {
    unsafe {
        if let Some(engine) = engine.as_mut().and_then(|e| e.engine.as_mut()) {
            if let Some(sprite) = engine.get_sprite(sprite_id.into()) {
                Box::into_raw(Box::new(SpriteC {
                    sprite: sprite as *mut _,
                }))
            } else { null_mut() }
        } else { null_mut() }
    }
}

// TODO complicated to convert &HashMap into C compatible, low priority function right now, skipping
// #[no_mangle]
// pub extern "C" fn EngineC_getAllSprites(engine: *const EngineC) {
//     unsafe {
//         if let Some(engine) = engine.as_ref().and_then(|e| e.engine.as_ref()) {
//
//         }
//     }
// }

#[no_mangle]
pub extern "C" fn EngineC_addSprite(
    engine: *mut EngineC,
    sprite_sheet: u32,
    sprite_index: usize,
    x_position: i32,
    y_position: i32,
    layer: i32,
    width: u32,
    height: u32,
    shader: u32
) -> u32 {
    unsafe {
        if let Some(engine) = engine.as_mut().and_then(|e| e.engine.as_mut()) {
            engine.add_sprite(
                sprite_sheet.into(),
                sprite_index,
                x_position,
                y_position,
                layer,
                width,
                height,
                shader.into()
            ).id()
        } else { u32::MAX }
    }
}

#[no_mangle]
pub extern "C" fn EngineC_removeSprite(engine: *mut EngineC, sprite_id: u32) {
    unsafe {
        if let Some(engine) = engine.as_mut().and_then(|e| e.engine.as_mut()) {
            engine.remove_sprite(sprite_id.into())
        }
    }
}

#[no_mangle]
pub extern "C" fn EngineC_toggleFullscreen(engine: *mut EngineC) {
    unsafe {
        if let Some(engine) = engine.as_mut().and_then(|e| e.engine.as_mut()) {
            engine.toggle_fullscreen()
        }
    }
}

#[no_mangle]
pub extern "C" fn EngineC_setWindowSize(engine: *mut EngineC, width: i32, height: i32) {
    unsafe {
        if let Some(engine) = engine.as_mut().and_then(|e| e.engine.as_mut()) {
            engine.set_window_size(width, height);
        }
    }
}

#[no_mangle]
pub extern "C" fn EngineC_toggleShowFps(engine: *mut EngineC) {
    unsafe {
        if let Some(engine) = engine.as_mut().and_then(|e| e.engine.as_mut()) {
            engine.toggle_show_fps();
        }
    }
}

#[no_mangle]
pub extern "C" fn EngineC_setFps(engine: *mut EngineC, target_fps: f32) {
    unsafe {
        if let Some(engine) = engine.as_mut().and_then(|e| e.engine.as_mut()) {
            engine.set_fps(target_fps);
        }
    }
}

#[no_mangle]
pub extern "C" fn EngineC_stop(engine: *mut EngineC) {
    unsafe {
        if let Some(engine) = engine.as_mut().and_then(|e| e.engine.as_mut()) {
            engine.stop();
        }
    }
}

#[no_mangle]
pub extern "C" fn EngineC_getKeyState(engine: *const EngineC, key: KeyC) -> ActionC {
    unsafe {
        if let Some(engine) = engine.as_ref().and_then(|e| e.engine.as_ref()) {
            engine.get_key_state(key.into()).into()
        } else { ActionC::default() }
    }
}

#[no_mangle]
pub extern "C" fn EngineC_isRunning(engine: *const EngineC) -> bool {
    unsafe {
        if let Some(engine) = engine.as_ref().and_then(|e| e.engine.as_ref()) {
            engine.is_running()
        } else { false }
    }
}

#[no_mangle]
pub extern "C" fn EngineC_toggleBorder(engine: *mut EngineC) {
    unsafe {
        if let Some(engine) = engine.as_mut().and_then(|e| e.engine.as_mut()) {
            engine.toggle_border()
        }
    }
}

#[no_mangle]
pub extern "C" fn EngineC_getDefaultFragmentShader(engine: *mut EngineC) -> u32 {
    unsafe {
        if let Some(engine) = engine.as_mut().and_then(|e| e.engine.as_mut()) {
            if let Some(shader) = engine.get_default_fragment_shader() {
                shader.id()
            } else { u32::MAX }
        } else { u32::MAX }
    }
}

#[no_mangle]
pub extern "C" fn EngineC_getDefaultVertexShader(engine: *mut EngineC) -> u32 {
    unsafe {
        if let Some(engine) = engine.as_mut().and_then(|e| e.engine.as_mut()) {
            if let Some(shader) = engine.get_default_vertex_shader() {
                shader.id()
            } else { u32::MAX }
        } else { u32::MAX }
    }
}

#[no_mangle]
pub extern "C" fn EngineC_drawFrame(engine: *mut EngineC) {
    unsafe {
        if let Some(engine) = engine.as_mut().and_then(|e| e.engine.as_mut()) {
            engine.draw_frame();
        }
    }
}

#[no_mangle]
pub extern "C" fn EngineC_getUVFromSpriteSheet(engine: *const EngineC, sprite_sheet: u32, index: usize) -> UVCoordsC {
    unsafe {
        if let Some(engine) = engine.as_ref().and_then(|e| e.engine.as_ref()) {
            if let Some((min_u, min_v, max_u, max_v)) = engine.get_uv_from_sprite_sheet(sprite_sheet.into(), index) {
                UVCoordsC {
                    min_u,
                    min_v,
                    max_u,
                    max_v,
                }
            } else { UVCoordsC::default() }
        } else { UVCoordsC::default() }
    }
}

#[no_mangle]
pub extern "C" fn EngineC_getTextureFromSpriteSheet(engine: *const EngineC, sprite_sheet: u32) -> u32 {
    unsafe {
        if let Some(engine) = engine.as_ref().and_then(|e| e.engine.as_ref()) {
            engine.get_texture_from_sprite_sheet(sprite_sheet.into()).unwrap_or(u32::MAX)
        } else { u32::MAX }
    }
}

#[no_mangle]
pub extern "C" fn EngineC_free(engine: *mut EngineC) {
    unsafe {
        if let Some(engine_c) = engine.as_mut() {
            if let Some(inner) = engine_c.engine.as_mut() {
                drop(Box::from_raw(inner));
            }
            drop(Box::from_raw(engine_c));
        }
    }
}

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
    unsafe {
        if let Some(timer_c) = timer.as_mut() {
            if let Some(inner) = timer_c.timer.as_mut() {
                drop(Box::from_raw(inner));
            }
            drop(Box::from_raw(timer_c));
        }
    }
}
