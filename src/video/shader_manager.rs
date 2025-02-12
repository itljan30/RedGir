use crate::video::sprite::{SpriteId, SpriteSheetId};
use crate::engine::GetId;

use gl::types::{GLuint, GLint, GLenum};
use std::ffi::{CString, NulError};
use std::ptr;
use std::string::FromUtf8Error;

pub const DEFAULT_VERTEX_SHADER: &str = r#"
#version 330 core

layout (location = 0) in vec2 position;
layout (location = 1) in vec2 tex_coords;

out vec2 frag_tex_coords;

void main() {
    gl_Position = vec4(position, 0.0f, 1.0f);
    frag_tex_coords = tex_coords;
}
"#;

pub const DEFAULT_FRAGMENT_SHADER: &str = r#"
#version 330 core

in vec2 frag_tex_coords;

uniform sampler2D tex_sample;

out vec4 frag_color;

void main() {
    frag_color = texture(tex_sample, frag_tex_coords);
}
"#;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ShaderError {
    LinkingError(String),
    CompilationError(String),
    NulError(NulError),
    FromUtf8Error(FromUtf8Error),
}

impl std::fmt::Display for ShaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShaderError::LinkingError(e)      => write!(f, "Shader Linking Error: {}", e),
            ShaderError::CompilationError(e)  => write!(f, "Shader Compilation Error: {}", e),
            ShaderError::NulError(e)          => write!(f, "Nul byte found in shader source: {}", e),
            ShaderError::FromUtf8Error(e)     => write!(f, "Invalid UTF-8 in shader log: {}", e),
        }
    }
}

impl From<FromUtf8Error> for ShaderError {
    fn from(value: FromUtf8Error) -> Self {
        ShaderError::FromUtf8Error(value)
    }
}

impl From<NulError> for ShaderError {
    fn from(value: NulError) -> Self {
        ShaderError::NulError(value)
    }
}

#[derive(Hash, Eq, Debug)]
pub struct VertexShader {
    id: GLuint,
}

impl PartialEq for VertexShader {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

    fn ne(&self, other: &Self) -> bool {
        self.id != other.id
    }
}

impl Drop for VertexShader {
     fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

impl VertexShader {
    pub fn new(source: &str) -> Result<Self, ShaderError> {
        Ok(Self {
            id: generate_and_compile_shader(source, gl::VERTEX_SHADER)?,
        })
    }
}

#[derive(Hash, Eq, Debug)]
pub struct FragmentShader {
    id: GLuint,
}

impl PartialEq for FragmentShader {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

    fn ne(&self, other: &Self) -> bool {
        self.id != other.id
    }
}

impl Drop for FragmentShader {
     fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

impl FragmentShader {
    pub fn new(source: &str) -> Result<Self, ShaderError> {
        Ok(Self {
            id: generate_and_compile_shader(source, gl::FRAGMENT_SHADER)?,
        })
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum ShaderDataType {
    Float       (fn(SpriteId) -> f32),
    FloatVec2   (fn(SpriteId) -> [f32; 2]),
    FloatVec3   (fn(SpriteId) -> [f32; 3]),
    FloatVec4   (fn(SpriteId) -> [f32; 4]),
    FloatMat2   (fn(SpriteId) -> [[f32; 2]; 2]),
    FloatMat3   (fn(SpriteId) -> [[f32; 3]; 3]),
    FloatMat4   (fn(SpriteId) -> [[f32; 4]; 4]),
    FloatMat2x3 (fn(SpriteId) -> [[f32; 3]; 2]),
    FloatMat2x4 (fn(SpriteId) -> [[f32; 4]; 2]),
    FloatMat3x2 (fn(SpriteId) -> [[f32; 2]; 3]),
    FloatMat3x4 (fn(SpriteId) -> [[f32; 4]; 3]),
    FloatMat4x2 (fn(SpriteId) -> [[f32; 2]; 4]),
    FloatMat4x3 (fn(SpriteId) -> [[f32; 3]; 4]),
    Int         (fn(SpriteId) -> i32),
    IntVec2     (fn(SpriteId) -> [i32; 2]),
    IntVec3     (fn(SpriteId) -> [i32; 3]),
    IntVec4     (fn(SpriteId) -> [i32; 4]),
    Bool        (fn(SpriteId) -> bool),
    BoolVec2    (fn(SpriteId) -> [bool; 2]),
    BoolVec3    (fn(SpriteId) -> [bool; 3]),
    BoolVec4    (fn(SpriteId) -> [bool; 4]),
    UInt        (fn(SpriteId) -> u32),
    UIntVec2    (fn(SpriteId) -> [u32; 2]),
    UIntVec3    (fn(SpriteId) -> [u32; 3]),
    UIntVec4    (fn(SpriteId) -> [u32; 4]),
    Sampler2D   (fn(SpriteId) -> (SpriteSheetId, usize)),
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Uniform {
    name: String,
    data: ShaderDataType,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Attribute {
    name: String,
    data: ShaderDataType,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShaderId {
    id: GLuint,
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct ShaderProgram {
    id: GLuint,
    shared_uniforms: Vec<Uniform>,
    per_sprite_uniforms: Vec<Uniform>,
    attributes: Vec<Attribute>,
}

impl GetId for ShaderProgram {
    type Id = ShaderId;
    fn id(&self) -> ShaderId {
        ShaderId { id: self.id }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl ShaderProgram {
    pub fn new(
        vertex_shader: &VertexShader,
        fragment_shader: &FragmentShader,
        attributes: Vec<Attribute>,
        shared_uniforms: Vec<Uniform>,
        per_sprite_uniforms: Vec<Uniform>,
    ) -> Result<Self, ShaderError> {
        let program;
        unsafe {
            program = Self {
                id: gl::CreateProgram(),
                attributes,
                shared_uniforms,
                per_sprite_uniforms,
            };

            gl::AttachShader(program.id, vertex_shader.id);
            gl::AttachShader(program.id, fragment_shader.id);

            gl::LinkProgram(program.id);

            let mut success = 0;
            gl::GetProgramiv(program.id, gl::LINK_STATUS, &mut success);

            if success == 0 {
                let mut log_length = 0;
                gl::GetProgramiv(program.id, gl::INFO_LOG_LENGTH, &mut log_length);
                
                let mut log = Vec::with_capacity(log_length as usize);
                gl::GetProgramInfoLog(
                    program.id,
                    log_length,
                    &mut log_length,
                    log.as_mut_ptr() as *mut _,
                );

                let log = String::from_utf8(log)?;
                return Err(ShaderError::LinkingError(log));
            }
        }
        Ok(program)
    }

    pub unsafe fn apply(&self) {
        gl::UseProgram(self.id)
    }
}

fn generate_and_compile_shader(source: &str, shader_type: GLenum) -> Result<GLuint, ShaderError> {
    let source = CString::new(source)?;
    let shader_id: GLuint;
    unsafe {
        shader_id = gl::CreateShader(shader_type);
        
        gl::ShaderSource(shader_id, 1, &source.as_ptr(), ptr::null());
        gl::CompileShader(shader_id);

        let mut success: GLint = 0;
        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);

        if success == 0 {
            let mut log_length = 0;
            gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut log_length);

            let mut log = Vec::with_capacity(log_length as usize);
            gl::GetShaderInfoLog(
                shader_id,
                log_length,
                ptr::null_mut(),
                log.as_mut_ptr() as *mut _
            );

            let log = String::from_utf8(log)?;
            gl::DeleteShader(shader_id);
            return Err(ShaderError::CompilationError(log));
        }
    }

    Ok(shader_id)
}
