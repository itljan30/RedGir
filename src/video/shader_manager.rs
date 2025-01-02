use gl::types::{GLuint, GLenum, GLint};
use std::ffi::CString;
use std::ptr;

const DEFAULT_VERTEX_SHADER: &'static str = r#"
#version 330 core
layout (location = 0) in vec3 aPos;

void main() {
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
"#;

const DEFAULT_FRAGMENT_SHADER: &'static str = r#"
#version 330
out vec4 FragColor;

void main() {
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}
"#;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

pub struct VertexShader {
    source: CString,
    id: GLuint,
}

impl Default for VertexShader {
    fn default() -> Self {
        VertexShader {
            source: CString::new(DEFAULT_VERTEX_SHADER).unwrap(),
            id: 0,
        }
    }
}

impl VertexShader {
    pub fn new(source: &str) -> Self {
        VertexShader {
            source: CString::new(source).unwrap(),
            id: 0,
        }
    }

    unsafe fn compile(&mut self) {
        self.id = compile_shader(&self.source, gl::VERTEX_SHADER);
    }
}

pub struct FragmentShader {
    source: CString,
    id: GLuint,
}

impl Default for FragmentShader {
    fn default() -> Self {
        FragmentShader {
            source: CString::new(DEFAULT_FRAGMENT_SHADER).unwrap(),
            id: 0,
        }
    }
}

impl FragmentShader {
    pub fn new(source: &str) -> Self {
        FragmentShader {
            source: CString::new(source).unwrap(),
            id: 0,
        }
    }

    unsafe fn compile(&mut self) {
        self.id = compile_shader(&self.source, gl::FRAGMENT_SHADER);
    }

}

pub struct ShaderProgram {
    pub id: GLuint,
    pub vertex_shader: VertexShader,
    pub fragment_shader: FragmentShader,
}

impl Default for ShaderProgram {
    fn default() -> Self {
        ShaderProgram {
            id: 0,
            vertex_shader: VertexShader::default(),
            fragment_shader: FragmentShader::default(),
        }
    }
}

impl ShaderProgram {
    pub fn new(vertex_shader: VertexShader, fragment_shader: FragmentShader) -> Self {
        ShaderProgram {
            id: 0,
            vertex_shader,
            fragment_shader,
        }
    }

    pub fn compile_and_link(&mut self) {
        unsafe {
            self.vertex_shader.compile();
            self.fragment_shader.compile();
            self.id = gl::CreateProgram();
            gl::AttachShader(self.id, self.vertex_shader.id);
            gl::AttachShader(self.id, self.fragment_shader.id);
            gl::LinkProgram(self.id);

            // TODO check for errors

            gl::DeleteShader(self.vertex_shader.id);
            gl::DeleteShader(self.fragment_shader.id);
        }
    }

    pub fn use_shader(&mut self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

unsafe fn compile_shader(source: &CString, shader_type: GLenum) -> GLuint {
    let id = gl::CreateShader(shader_type);
    gl::ShaderSource(id, 1, &source.as_ptr(), ptr::null());
    gl::CompileShader(id);

    let mut success: GLint = 0;
    gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);

    id
}
