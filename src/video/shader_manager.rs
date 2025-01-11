use gl::types::{GLuint, GLint};
use std::ffi::{CString, NulError};
use std::ptr;
use std::string::FromUtf8Error;

pub const DEFAULT_VERTEX_SHADER: &str = r#"
#version 330 core

layout (location = 0) in vec2 aPos;

void main() {
    gl_Position = vec4(aPos.x, aPos.y, 0.0, 1.0);
}
"#;

pub const DEFAULT_FRAGMENT_SHADER: &str = r#"
#version 330 core

out vec4 fragColor;

void main() {
    fragColor = vec4(0.5f, 1.0f, 0.2f, 1.0f);
}
"#;

#[derive(Debug)]
pub enum ShaderType {
    VertexShader,
    FragmentShader,
}

#[derive(Debug)]
pub enum ShaderError {
    NulError(NulError),
    Utf8Error(FromUtf8Error),
    CompilationError(String),
    LinkingError(String),
}

impl std::fmt::Display for ShaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShaderError::NulError(e) => write!(f, "{}", e),
            ShaderError::Utf8Error(e) => write!(f, "{}", e),
            ShaderError::CompilationError(e) => write!(f, "{}", e),
            ShaderError::LinkingError(e) => write!(f, "{}", e),
        }
    }
}

impl From<NulError> for ShaderError {
    fn from(error: NulError) -> Self {
        ShaderError::NulError(error)
    }
}

impl From<FromUtf8Error> for ShaderError {
    fn from(error: FromUtf8Error) -> Self {
        ShaderError::Utf8Error(error)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ShaderId {
    id: GLuint,
}

impl ShaderId {
    pub fn new(id: GLuint) -> Self {
        ShaderId {
            id,
        }
    }
}

#[derive(Clone)]
pub struct Shader {
    id: GLuint,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

impl Shader {
    pub fn new(source: &str, shader_type: ShaderType) -> Result<Shader, ShaderError> {
        unsafe {
            let source = CString::new(source)?;
            let id = match shader_type {
                ShaderType::FragmentShader => gl::CreateShader(gl::FRAGMENT_SHADER),
                ShaderType::VertexShader => gl::CreateShader(gl::VERTEX_SHADER),
            };

            let shader = Shader {
                id,
            };

            gl::ShaderSource(shader.id, 1, &source.as_ptr(), ptr::null());
            gl::CompileShader(shader.id);

            let mut success: GLint = 0;
            gl::GetShaderiv(shader.id, gl::COMPILE_STATUS, &mut success);

            if success == 1 {
                Ok(shader)
            }
            else {
                let mut error_log_size: GLint = 0;
                gl::GetShaderiv(shader.id, gl::INFO_LOG_LENGTH, &mut error_log_size);

                let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
                gl::GetShaderInfoLog(
                    shader.id,
                    error_log_size,
                    &mut error_log_size,
                    error_log.as_mut_ptr() as *mut _,
                );

                error_log.set_len(error_log_size as usize);
                let log = String::from_utf8(error_log)?;
                Err(ShaderError::CompilationError(log))
            }
        }
    }
}

pub struct ShaderProgram {
    id: GLuint,
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl ShaderProgram {
    pub fn get_id(&self) -> GLuint {
        self.id
    }
}

impl ShaderProgram {
    pub fn new(shaders: &[Shader]) -> Result<Self, ShaderError> {
        unsafe {
            let program = ShaderProgram {
                id: gl::CreateProgram(),
            };

            for shader in shaders {
                gl::AttachShader(program.id, shader.id);
            }

            gl::LinkProgram(program.id);

            let mut success: GLint = 0;
            gl::GetProgramiv(program.id, gl::LINK_STATUS, &mut success);

            if success == 1 {
                Ok(program)
            }
            else {
                let mut error_log_size: GLint = 0;
                gl::GetProgramiv(program.id, gl::INFO_LOG_LENGTH, &mut error_log_size);

                let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
                gl::GetProgramInfoLog(
                    program.id,
                    error_log_size,
                    &mut error_log_size,
                    error_log.as_mut_ptr() as *mut _,
                );

                error_log.set_len(error_log_size as usize);
                let log = String::from_utf8(error_log)?;
                Err(ShaderError::LinkingError(log))
            }
        }
    }

    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id);
    }
}
