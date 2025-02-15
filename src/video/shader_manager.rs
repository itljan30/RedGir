use crate::video::sprite::Sprite;
use crate::engine::{GetId, Engine};

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
            ShaderError::LinkingError(e)     => write!(f, "Shader Linking Error: {}", e),
            ShaderError::CompilationError(e) => write!(f, "Shader Compilation Error: {}", e),
            ShaderError::NulError(e)         => write!(f, "Nul byte found in shader source: {}", e),
            ShaderError::FromUtf8Error(e)    => write!(f, "Invalid UTF-8 in shader log: {}", e),
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
            id: generate_and_compile_shader(source, gl::VERTEX_SHADER)?,
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
            id: generate_and_compile_shader(source, gl::FRAGMENT_SHADER)?,
        })
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
/// Attributes require data per vertex instead of per sprite.
/// Therefore, a vec2 is really 4 vec2s, one per vertex
/// Vertices should be returned in this order:
/// [[bottom_left], [bottom_right], [top_left], [top_right]]
pub enum AttributeData {
    Float       (fn(&Engine, &Sprite) -> [f32; 4]),
    FloatVec2   (fn(&Engine, &Sprite) -> [[f32; 2]; 4]),
    FloatVec3   (fn(&Engine, &Sprite) -> [[f32; 3]; 4]),
    FloatVec4   (fn(&Engine, &Sprite) -> [[f32; 4]; 4]),
    Int         (fn(&Engine, &Sprite) -> [i32; 4]),
    IntVec2     (fn(&Engine, &Sprite) -> [[i32; 2]; 4]),
    IntVec3     (fn(&Engine, &Sprite) -> [[i32; 3]; 4]),
    IntVec4     (fn(&Engine, &Sprite) -> [[i32; 4]; 4]),
    Bool        (fn(&Engine, &Sprite) -> [bool; 4]),
    BoolVec2    (fn(&Engine, &Sprite) -> [[bool; 2]; 4]),
    BoolVec3    (fn(&Engine, &Sprite) -> [[bool; 3]; 4]),
    BoolVec4    (fn(&Engine, &Sprite) -> [[bool; 4]; 4]),
    UInt        (fn(&Engine, &Sprite) -> [u32; 4]),
    UIntVec2    (fn(&Engine, &Sprite) -> [[u32; 2]; 4]),
    UIntVec3    (fn(&Engine, &Sprite) -> [[u32; 3]; 4]),
    UIntVec4    (fn(&Engine, &Sprite) -> [[u32; 4]; 4]),
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum UniformData {
    Float       (fn(&Engine, &Sprite) -> f32),
    FloatVec2   (fn(&Engine, &Sprite) -> [f32; 2]),
    FloatVec3   (fn(&Engine, &Sprite) -> [f32; 3]),
    FloatVec4   (fn(&Engine, &Sprite) -> [f32; 4]),
    FloatMat2   (fn(&Engine, &Sprite) -> [[f32; 2]; 2]),
    FloatMat3   (fn(&Engine, &Sprite) -> [[f32; 3]; 3]),
    FloatMat4   (fn(&Engine, &Sprite) -> [[f32; 4]; 4]),
    FloatMat2x3 (fn(&Engine, &Sprite) -> [[f32; 3]; 2]),
    FloatMat2x4 (fn(&Engine, &Sprite) -> [[f32; 4]; 2]),
    FloatMat3x2 (fn(&Engine, &Sprite) -> [[f32; 2]; 3]),
    FloatMat3x4 (fn(&Engine, &Sprite) -> [[f32; 4]; 3]),
    FloatMat4x2 (fn(&Engine, &Sprite) -> [[f32; 2]; 4]),
    FloatMat4x3 (fn(&Engine, &Sprite) -> [[f32; 3]; 4]),
    Int         (fn(&Engine, &Sprite) -> i32),
    IntVec2     (fn(&Engine, &Sprite) -> [i32; 2]),
    IntVec3     (fn(&Engine, &Sprite) -> [i32; 3]),
    IntVec4     (fn(&Engine, &Sprite) -> [i32; 4]),
    Bool        (fn(&Engine, &Sprite) -> bool),
    BoolVec2    (fn(&Engine, &Sprite) -> [bool; 2]),
    BoolVec3    (fn(&Engine, &Sprite) -> [bool; 3]),
    BoolVec4    (fn(&Engine, &Sprite) -> [bool; 4]),
    UInt        (fn(&Engine, &Sprite) -> u32),
    UIntVec2    (fn(&Engine, &Sprite) -> [u32; 2]),
    UIntVec3    (fn(&Engine, &Sprite) -> [u32; 3]),
    UIntVec4    (fn(&Engine, &Sprite) -> [u32; 4]),
    // TODO add a TextureId wrapper or something, u32 is OpenGL texture id
    /// NOTE u32 is OpenGL texture id
    Sampler2D   (fn(&Engine, &Sprite) -> u32),
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Uniform {
    name: String,
    data: UniformData,
}

impl Uniform {
    pub fn new(name: String, data: UniformData) -> Self {
        Self {
            name,
            data,
        }
    }

    pub fn bind(&self, shader_id: GLuint, engine: &Engine, sprite: &Sprite) {
        let location;
        unsafe {
            location = gl::GetUniformLocation(shader_id, self.name.as_ptr() as *const i8);
            match self.data {
                UniformData::Float(func) => {
                    let data = func(engine, sprite);
                    gl::Uniform1f(location, data);
                }
                UniformData::FloatVec2(func) => {
                    let data = func(engine, sprite);
                    gl::Uniform2f(location, data[0], data[1]);
                },
                UniformData::FloatVec3(func) => {
                    let data = func(engine, sprite);
                    gl::Uniform3f(location, data[0], data[1], data[2])
                },
                UniformData::FloatVec4(func) => {
                    let data = func(engine, sprite);
                    gl::Uniform4f(location, data[0], data[1], data[2], data[3])
                },
                UniformData::FloatMat2(func) => {
                    let data = func(engine, sprite);
                    gl::UniformMatrix2fv(location, 1, gl::FALSE, data.as_ptr() as *const f32)
                },
                UniformData::FloatMat3(func) => {
                    let data = func(engine, sprite);
                    gl::UniformMatrix3fv(location, 1, gl::FALSE, data.as_ptr() as *const f32)
                },
                UniformData::FloatMat4(func) => {
                    let data = func(engine, sprite);
                    gl::UniformMatrix4fv(location, 1, gl::FALSE, data.as_ptr() as *const f32)
                },
                UniformData::FloatMat2x3(func) => {
                    let data = func(engine, sprite);
                    gl::UniformMatrix2x3fv(location, 1, gl::FALSE, data.as_ptr() as *const f32)
                },
                UniformData::FloatMat2x4(func) => {
                    let data = func(engine, sprite);
                    gl::UniformMatrix2x4fv(location, 1, gl::FALSE, data.as_ptr() as *const f32)
                },
                UniformData::FloatMat3x2(func) => {
                    let data = func(engine, sprite);
                    gl::UniformMatrix3x2fv(location, 1, gl::FALSE, data.as_ptr() as *const f32)
                },
                UniformData::FloatMat3x4(func) => {
                    let data = func(engine, sprite);
                    gl::UniformMatrix3x4fv(location, 1, gl::FALSE, data.as_ptr() as *const f32)
                },
                UniformData::FloatMat4x2(func) => {
                    let data = func(engine, sprite);
                    gl::UniformMatrix4x2fv(location, 1, gl::FALSE, data.as_ptr() as *const f32)
                },
                UniformData::FloatMat4x3(func) => {
                    let data = func(engine, sprite);
                    gl::UniformMatrix4x3fv(location, 1, gl::FALSE, data.as_ptr() as *const f32)
                },
                UniformData::Int(func) => {
                    let data = func(engine, sprite);
                    gl::Uniform1i(location, data);
                },
                UniformData::IntVec2(func) => {
                    let data = func(engine, sprite);
                    gl::Uniform2i(location, data[0], data[1]);
                },
                UniformData::IntVec3(func) => {
                    let data = func(engine, sprite);
                    gl::Uniform3i(location, data[0], data[1], data[2])
                },
                UniformData::IntVec4(func) => {
                    let data = func(engine, sprite);
                    gl::Uniform4i(location, data[0], data[1], data[2], data[3])
                },
                UniformData::Bool(func) => {
                    let data = func(engine, sprite);
                    gl::Uniform1i(location, data as GLint);
                },
                UniformData::BoolVec2(func) => {
                    let data = func(engine, sprite);
                    gl::Uniform2i(location, data[0] as GLint, data[1] as GLint);
                },
                UniformData::BoolVec3(func) => {
                    let data = func(engine, sprite);
                    gl::Uniform3i(location, data[0] as GLint, data[1] as GLint, data[2] as GLint);
                },
                UniformData::BoolVec4(func) => {
                    let data = func(engine, sprite);
                    gl::Uniform4i(location, data[0] as GLint, data[1] as GLint, data[2] as GLint, data[3] as GLint)
                },
                UniformData::UInt(func) => {
                    let data = func(engine, sprite);
                    gl::Uniform1ui(location, data);
                },
                UniformData::UIntVec2(func) => {
                    let data = func(engine, sprite);
                    gl::Uniform2ui(location, data[0], data[1]);
                },
                UniformData::UIntVec3(func) => {
                    let data = func(engine, sprite);
                    gl::Uniform3ui(location, data[0], data[1], data[2]);
                },
                UniformData::UIntVec4(func) => {
                    let data = func(engine, sprite);
                    gl::Uniform4ui(location, data[0], data[1], data[2], data[3]);
                },
                UniformData::Sampler2D(func) => {
                    let texture_id = func(engine, sprite);
                    gl::ActiveTexture(gl::TEXTURE0);
                    gl::BindTexture(gl::TEXTURE_2D, texture_id);
                },
            };
        }
    }

    pub fn texture_from_sprite_sheet(name: String) -> Self {
        Self {
            name,
            data: UniformData::Sampler2D(|engine: &Engine, sprite: &Sprite| {
                let sprite_sheet_id = sprite.get_sprite_sheet();
                engine.get_texture_from_sprite_sheet(sprite_sheet_id).unwrap()
            }),
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Attribute {
    name: String,
    location: u32,
    data: AttributeData,
}

impl Attribute {
    pub fn new(name: String, location: u32, data: AttributeData) -> Self {
        Self {
            name,
            location,
            data,
        }
    }

    pub fn position(name: String, location: u32) -> Self {
        Self::new(
            name,
            location,
            AttributeData::FloatVec2(|engine: &Engine, sprite: &Sprite| {
                let (w_width, w_height) = engine.get_window_dimensions();
                let (s_width, s_height) = (sprite.get_width(), sprite.get_height());
                let pos = sprite.get_position();
                let bottom_left = pos;
                let bottom_right = (pos.0 + s_width as i32, pos.1);
                let top_left = (pos.0, pos.1 + s_height as i32);
                let top_right = (pos.0 + s_width as i32, pos.1 + s_height as i32);

                [
                    [
                        2.0 * bottom_left.0 as f32 / w_width as f32 - 1.0,
                        2.0 * bottom_left.1 as f32 / w_height as f32 - 1.0
                    ],
                    [
                        2.0 * bottom_right.0 as f32 / w_width as f32 - 1.0,
                        2.0 * bottom_right.1 as f32 / w_height as f32 - 1.0
                    ],
                    [
                        2.0 * top_left.0 as f32 / w_width as f32 - 1.0,
                        2.0 * top_left.1 as f32 / w_height as f32 - 1.0
                    ],
                    [
                        2.0 * top_right.0 as f32 / w_width as f32 - 1.0,
                        2.0 * top_right.1 as f32 / w_height as f32 - 1.0
                    ],
                ]
            }),
        )
    }

    pub fn texture_uv_from_sprite_sheet(name: String, location: u32) -> Self {
        Self::new(
            name,
            location,
            AttributeData::FloatVec2(|engine: &Engine, sprite: &Sprite| {
                let sprite_sheet = sprite.get_sprite_sheet();
                let index = sprite.get_sprite_sheet_index();
                let (u_min, v_min, u_max, v_max) = engine.get_uv_from_sprite_sheet(sprite_sheet, index).unwrap();
                [
                    [u_min, v_min],
                    [u_max, v_min],
                    [u_min, v_max],
                    [u_max, v_max],
                ]
            }),
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShaderId {
    id: GLuint,
}

impl GetId for ShaderId {
    type Id = u32;
    fn id(&self) -> u32 {
        self.id
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct VertexArray {
    id: GLuint
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, [self.id].as_ptr());
        }
    }
}

impl VertexArray {
    fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        Self { id }
    }

    /// Sets the attribute to the vao.
    /// Returns the offset for next the attribute.
    fn set_attribute(&self, attribute: &Attribute, offset: u32) -> u32 {
        let (len, size, gl_type) = match attribute.data {
            AttributeData::Float(_)     => (1, size_of::<f32>(), gl::FLOAT),
            AttributeData::FloatVec2(_) => (2, size_of::<f32>(), gl::FLOAT),
            AttributeData::FloatVec3(_) => (3, size_of::<f32>(), gl::FLOAT),
            AttributeData::FloatVec4(_) => (4, size_of::<f32>(), gl::FLOAT),
            AttributeData::Int(_)       => (1, size_of::<i32>(), gl::INT),
            AttributeData::IntVec2(_)   => (2, size_of::<i32>(), gl::INT),
            AttributeData::IntVec3(_)   => (3, size_of::<i32>(), gl::INT),
            AttributeData::IntVec4(_)   => (4, size_of::<i32>(), gl::INT),
            AttributeData::Bool(_)      => (1, size_of::<bool>(), gl::UNSIGNED_BYTE),
            AttributeData::BoolVec2(_)  => (2, size_of::<bool>(), gl::UNSIGNED_BYTE),
            AttributeData::BoolVec3(_)  => (3, size_of::<bool>(), gl::UNSIGNED_BYTE),
            AttributeData::BoolVec4(_)  => (4, size_of::<bool>(), gl::UNSIGNED_BYTE),
            AttributeData::UInt(_)      => (1, size_of::<u32>(), gl::UNSIGNED_INT),
            AttributeData::UIntVec2(_)  => (2, size_of::<u32>(), gl::UNSIGNED_INT),
            AttributeData::UIntVec3(_)  => (3, size_of::<u32>(), gl::UNSIGNED_INT),
            AttributeData::UIntVec4(_)  => (4, size_of::<u32>(), gl::UNSIGNED_INT),
        };
        unsafe {
            gl::EnableVertexAttribArray(attribute.location);
            gl::VertexAttribPointer(
                attribute.location,
                len,
                gl_type,
                gl::FALSE,
                0,
                offset as *const _,
            );
        }
        (len as u32 * size as u32 * 6) + offset
    }

    pub unsafe fn bind(&self) {
        gl::BindVertexArray(self.id);
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct VertexBuffer {
    id: GLuint,
    target: GLuint,
}

impl Drop for VertexBuffer {
    fn drop(&mut self) { 
        unsafe {
            gl::DeleteBuffers(1, [self.id].as_ptr());
        }
    }
}

impl VertexBuffer {
    fn new(target: GLuint) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Self {
            id,
            target,
        }
    }

    pub unsafe fn bind(&self) {
        gl::BindBuffer(self.target, self.id);
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct ShaderProgram {
    id: GLuint,
    attributes: Vec<Attribute>,
    global_uniforms: Vec<Uniform>,
    instance_uniforms: Vec<Uniform>,
    vao: VertexArray,
    vbo: VertexBuffer,
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
        global_uniforms: Vec<Uniform>,
        instance_uniforms: Vec<Uniform>,
    ) -> Result<Self, ShaderError> {
        unsafe {
            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex_shader.id);
            gl::AttachShader(id, fragment_shader.id);

            gl::LinkProgram(id);

            let mut success = 0;
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);

            if success == 0 {
                let mut log_length = 0;
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut log_length);
                
                let mut log = Vec::with_capacity(log_length as usize);
                gl::GetProgramInfoLog(
                    id,
                    log_length,
                    &mut log_length,
                    log.as_mut_ptr() as *mut _,
                );

                let log = String::from_utf8(log)?;
                return Err(ShaderError::LinkingError(log));
            }

            let vbo = VertexBuffer::new(gl::ARRAY_BUFFER);
            vbo.bind();

            let mut offset = 0;
            let vao = VertexArray::new();
            vao.bind();
            for attribute in attributes.iter() {
                offset = vao.set_attribute(attribute, offset);
            }

            Ok(Self {
                id,
                attributes,
                global_uniforms,
                instance_uniforms,
                vao,
                vbo,
            })
        }
    }

    pub unsafe fn apply_global_uniforms(&self, engine: &Engine, sprite: &Sprite) {
        for uniform in self.global_uniforms() {
            uniform.bind(self.id, engine, sprite)
        }
    }

    pub unsafe fn apply_instance_uniforms(&self, engine: &Engine, sprite: &Sprite) {
        for uniform in self.instance_uniforms() {
            uniform.bind(self.id, engine, sprite);
        }
    }

    pub unsafe fn fill_vbo(&self, engine: &Engine, sprite: &Sprite) {
        let mut buffer_data = Vec::new();

        for attribute in self.attributes() {
            match attribute.data{ // Call the function to get the data
                AttributeData::Float(func)     => push_callback_result_as_slice(&mut buffer_data, &func(engine, sprite)),
                AttributeData::FloatVec2(func) => push_callback_result_as_slice(&mut buffer_data, &func(engine, sprite)),
                AttributeData::FloatVec3(func) => push_callback_result_as_slice(&mut buffer_data, &func(engine, sprite)),
                AttributeData::FloatVec4(func) => push_callback_result_as_slice(&mut buffer_data, &func(engine, sprite)),
                AttributeData::Int(func)       => push_callback_result_as_slice(&mut buffer_data, &func(engine, sprite)),
                AttributeData::IntVec2(func)   => push_callback_result_as_slice(&mut buffer_data, &func(engine, sprite)),
                AttributeData::IntVec3(func)   => push_callback_result_as_slice(&mut buffer_data, &func(engine, sprite)),
                AttributeData::IntVec4(func)   => push_callback_result_as_slice(&mut buffer_data, &func(engine, sprite)),
                AttributeData::Bool(func)      => push_callback_result_as_slice(&mut buffer_data, &func(engine, sprite)),
                AttributeData::BoolVec2(func)  => push_callback_result_as_slice(&mut buffer_data, &func(engine, sprite)),
                AttributeData::BoolVec3(func)  => push_callback_result_as_slice(&mut buffer_data, &func(engine, sprite)),
                AttributeData::BoolVec4(func)  => push_callback_result_as_slice(&mut buffer_data, &func(engine, sprite)),
                AttributeData::UInt(func)      => push_callback_result_as_slice(&mut buffer_data, &func(engine, sprite)),
                AttributeData::UIntVec2(func)  => push_callback_result_as_slice(&mut buffer_data, &func(engine, sprite)),
                AttributeData::UIntVec3(func)  => push_callback_result_as_slice(&mut buffer_data, &func(engine, sprite)),
                AttributeData::UIntVec4(func)  => push_callback_result_as_slice(&mut buffer_data, &func(engine, sprite)),
            }

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo.id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                buffer_data.len() as isize,
                buffer_data.as_ptr() as *const _,
                gl::DYNAMIC_DRAW,
            );
        }
    }

    pub fn attributes(&self) -> &Vec<Attribute> {
        &self.attributes
    }

    pub fn global_uniforms(&self) -> &Vec<Uniform> {
        &self.global_uniforms
    }

    pub fn instance_uniforms(&self) -> &Vec<Uniform> {
        &self.instance_uniforms
    }

    pub unsafe fn apply(&self) {
        gl::UseProgram(self.id);
        self.vao.bind();
        self.vbo.bind();
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

/// [[bottom_left], [bottom_right], [top_left], [top_right]]
unsafe fn push_callback_result_as_slice<T: Sized + Copy>(buffer: &mut Vec<u8>, data: &[T]) {
    let new_data = [
        data[0], data[1], data[2],
        data[1], data[2], data[3],
    ];
    let byte_slice = std::slice::from_raw_parts(
        new_data.as_ptr() as *const u8,
        new_data.len() * std::mem::size_of::<T>()
    );
    buffer.extend_from_slice(byte_slice);
}
