use gl::types::{GLuint, GLint, GLenum};
use std::ffi::{CString, NulError};
use std::ptr;
use std::string::FromUtf8Error;
use crate::engine::GetId;

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

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct VertexShader {
    id: GLuint,
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
            id: generate_and_compile_shader(source, gl::VERTEX_SHADER)?
        })
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct FragmentShader {
    id: GLuint,
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
            id: generate_and_compile_shader(source, gl::FRAGMENT_SHADER)?
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShaderId {
    id: GLuint,
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct ShaderProgram {
    id: GLuint,
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
    pub fn new(vertex_shader: &VertexShader, fragment_shader: &FragmentShader) -> Result<Self, ShaderError> {
        let program;
        unsafe {
            program = Self {
                id: gl::CreateProgram(),
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

    pub fn get_program_id(&self) -> u32 {
        self.id
    }

    pub unsafe fn use_program(&self) {
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
