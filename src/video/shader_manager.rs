use crate::video::sprite::{Sprite, Flip};
use crate::engine::{GetId, Engine};

use gl::types::{GLuint, GLint, GLenum};
use std::ffi::{CString, NulError};
use std::ptr;
use std::string::FromUtf8Error;

// FIXME make it so the shader doesn't have to take in the aspect ratio
pub const DEFAULT_VERTEX_SHADER: &str = r#"
#version 330 core

layout (location = 0) in vec2 u_position;
layout (location = 1) in vec2 tex_coords;

uniform float u_rotation;
uniform vec2 u_sprite_center;
uniform float u_aspect_ratio;
uniform vec2 u_flip;

out vec2 frag_tex_coords;

void main() {
    vec2 new_position = u_position - u_sprite_center;

    new_position *= mat2(
        cos(u_rotation), -sin(u_rotation),
        sin(u_rotation), cos(u_rotation)
    );

    if (u_flip.x == 1.0) {
        new_position.x *= -1.0;
    }
    if (u_flip.y == 1.0) {
        new_position.y *= -1.0;
    }

    new_position.y *= u_aspect_ratio;
    new_position += u_sprite_center;

    gl_Position = vec4(new_position, 0.0f, 1.0f);
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
            ShaderError::LinkingError(e)     => write!(f, "LinkingError {}", e),
            ShaderError::CompilationError(e) => write!(f, "CompilationError {}", e),
            ShaderError::NulError(e)         => write!(f, "NulError {}", e),
            ShaderError::FromUtf8Error(e)    => write!(f, "FromUtf8Error {}", e),
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
pub enum AttributeDataType {
    Float,
    FloatVec2,
    FloatVec3,
    FloatVec4,
    Int,
    Bool,
    UInt,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum UniformDataType {
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
    // TODO add a TextureId wrapper or something, u32 is OpenGL texture id
    /// NOTE u32 is OpenGL texture id
    Sampler2D,
}

// FIXME implement own PartialEq function instead of just ignoring warning
#[allow(unpredictable_function_pointer_comparisons)]
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Uniform {
    name: String,
    callback: fn(&Engine, &Sprite, &mut Vec<u8>),
    data_type: UniformDataType,
}

impl Uniform {
    pub fn new(name: String, callback: fn(&Engine, &Sprite, &mut Vec<u8>), data_type: UniformDataType) -> Self {
        Self {
            name,
            callback,
            data_type,
        }
    }

    pub fn push_data_to_result<T: Copy>(result: &mut Vec<u8>, data: &[T]) {
        let ptr = data.as_ptr() as *const u8;
        let size = data.len() * std::mem::size_of::<T>();
        unsafe {
            result.extend_from_slice(std::slice::from_raw_parts(ptr, size));
        }
    }

    pub fn time_since_initialization(name: String) -> Self {
        Self {
            name,
            callback: |engine: &Engine, _sprite: &Sprite, buffer: &mut Vec<u8>| {
                Uniform::push_data_to_result(buffer, &[engine.time_since_initialization_seconds()]);
            },
            data_type: UniformDataType::Float
        }
    }

    pub fn aspect_ratio(name: String) -> Self {
        Self {
            name, 
            callback: |engine: &Engine, _sprite: &Sprite, buffer: &mut Vec<u8>| {
                let (width, height) = engine.get_window_dimensions();
                Uniform::push_data_to_result(buffer, &[width as f32 / height as f32]);
            },
            data_type: UniformDataType::Float,
        }
    }

    /// A preset Uniform that retuns a FloatVec2 of [x, y] position of center of sprite in NDC
    pub fn sprite_center(name: String) -> Self {
        Self {
            name,
            callback: |engine: &Engine, sprite: &Sprite, buffer: &mut Vec<u8>| {
                let (w_width, w_height) = engine.get_window_dimensions();
                let (s_width, s_height) = (sprite.get_width(), sprite.get_height());
                let (x, y) = sprite.get_position();
                let aspect_ratio = w_width as f32 / w_height as f32;

                let data = [
                    2.0 * (x as f32 + (s_width as f32 / 2.0)) / w_width as f32 - 1.0,
                    2.0 * ((y as f32 + (s_height as f32 / 2.0)) / aspect_ratio) / w_height as f32 - 1.0
                ];
                Uniform::push_data_to_result(buffer, &data);
            },
            data_type: UniformDataType::FloatVec2,
        }
    }

    /// A preset Uniform that returns a float representing the rotation of the sprite in radians.
    pub fn rotation(name: String) -> Self {
        Self {
            name,
            callback: |_engine: &Engine, sprite: &Sprite, buffer: &mut Vec<u8>| {
                Uniform::push_data_to_result(buffer, &[sprite.get_rotation()]);
            },
            data_type: UniformDataType::Float,
        }
    }

    /// A preset Uniform that returns a vec2 of float representing [horizontal, vertical], 0.0 for
    /// false, 1.0 for true
    pub fn flip(name: String) -> Self {
        Self {
            name,
            callback: |_engine: &Engine, sprite: &Sprite, result: &mut Vec<u8>| {
                let flip = sprite.get_flip();
                let data = match flip {
                    Flip::None   => [0.0, 0.0],
                    Flip::FlipX  => [1.0, 0.0],
                    Flip::FlipY  => [0.0, 1.0],
                    Flip::FlipXY => [1.0, 1.0],
                };
                Uniform::push_data_to_result(result, &data);
            },
            data_type: UniformDataType::FloatVec2,
        }
    }

    pub fn bind(&self, shader_id: GLuint, engine: &Engine, sprite: &Sprite) {
        let location;
        unsafe {
            location = gl::GetUniformLocation(shader_id, self.name.as_ptr() as *const i8);
        }
        match self.data_type {
            UniformDataType::Float => {
                let mut buffer: Vec<u8> = Vec::with_capacity(std::mem::size_of::<f32>());
                (self.callback)(engine, sprite, &mut buffer);
                unsafe {
                    let value = *(buffer.as_ptr() as *const f32);
                    gl::Uniform1f(location, value);
                }
            }
            UniformDataType::FloatVec2 => {
                let mut buffer: Vec<u8> = Vec::with_capacity(std::mem::size_of::<[f32; 2]>());
                (self.callback)(engine, sprite, &mut buffer);
                unsafe {
                    let value = *(buffer.as_ptr() as *const [f32; 2]);
                    gl::Uniform2f(location, value[0], value[1]);
                }
            },
            UniformDataType::FloatVec3 => {
                let mut buffer: Vec<u8> = Vec::with_capacity(std::mem::size_of::<[f32; 3]>());
                (self.callback)(engine, sprite, &mut buffer);
                unsafe {
                    let value = *(buffer.as_ptr() as *const [f32; 3]);
                    gl::Uniform3f(location, value[0], value[1], value[2]);
                }
            }
            UniformDataType::FloatVec4 => {
                let mut buffer: Vec<u8> = Vec::with_capacity(std::mem::size_of::<[f32; 4]>());
                (self.callback)(engine, sprite, &mut buffer);
                unsafe {
                    let value = *(buffer.as_ptr() as *const [f32; 4]);
                    gl::Uniform4f(location, value[0], value[1], value[2], value[3]);
                }
            }
            UniformDataType::FloatMat2 => {
                let mut buffer: Vec<u8> = Vec::with_capacity(std::mem::size_of::<[f32; 4]>());
                (self.callback)(engine, sprite, &mut buffer);
                unsafe {
                    gl::UniformMatrix2fv(location, 1, gl::FALSE, buffer.as_ptr() as *const f32);
                }
            }
            UniformDataType::FloatMat3 => {
                let mut buffer: Vec<u8> = Vec::with_capacity(std::mem::size_of::<[f32; 9]>());
                (self.callback)(engine, sprite, &mut buffer);
                unsafe {
                    gl::UniformMatrix3fv(location, 1, gl::FALSE, buffer.as_ptr() as *const f32);
                }
            }
            UniformDataType::FloatMat4 => {
                let mut buffer: Vec<u8> = Vec::with_capacity(std::mem::size_of::<[f32; 16]>());
                (self.callback)(engine, sprite, &mut buffer);
                unsafe {
                    gl::UniformMatrix4fv(location, 1, gl::FALSE, buffer.as_ptr() as *const f32);
                }
            }
            UniformDataType::FloatMat2x3 => {
                let mut buffer: Vec<u8> = Vec::with_capacity(std::mem::size_of::<[f32; 6]>());
                (self.callback)(engine, sprite, &mut buffer);
                unsafe {
                    gl::UniformMatrix2x3fv(location, 1, gl::FALSE, buffer.as_ptr() as *const f32);
                }
            }
            UniformDataType::FloatMat2x4 => {
                let mut buffer: Vec<u8> = Vec::with_capacity(std::mem::size_of::<[f32; 8]>());
                (self.callback)(engine, sprite, &mut buffer);
                unsafe {
                    gl::UniformMatrix2x4fv(location, 1, gl::FALSE, buffer.as_ptr() as *const f32);
                }
            }
            UniformDataType::FloatMat3x2 => {
                let mut buffer: Vec<u8> = Vec::with_capacity(std::mem::size_of::<[f32; 6]>());
                (self.callback)(engine, sprite, &mut buffer);
                unsafe {
                    gl::UniformMatrix3x2fv(location, 1, gl::FALSE, buffer.as_ptr() as *const f32);
                }
            }
            UniformDataType::FloatMat3x4 => {
                let mut buffer: Vec<u8> = Vec::with_capacity(std::mem::size_of::<[f32; 12]>());
                (self.callback)(engine, sprite, &mut buffer);
                unsafe {
                    gl::UniformMatrix3x4fv(location, 1, gl::FALSE, buffer.as_ptr() as *const f32);
                }
            }
            UniformDataType::FloatMat4x2 => {
                let mut buffer: Vec<u8> = Vec::with_capacity(std::mem::size_of::<[f32; 8]>());
                (self.callback)(engine, sprite, &mut buffer);
                unsafe {
                    gl::UniformMatrix4x2fv(location, 1, gl::FALSE, buffer.as_ptr() as *const f32);
                }
            }
            UniformDataType::FloatMat4x3 => {
                let mut buffer: Vec<u8> = Vec::with_capacity(std::mem::size_of::<[f32; 12]>());
                (self.callback)(engine, sprite, &mut buffer);
                unsafe {
                    gl::UniformMatrix4x3fv(location, 1, gl::FALSE, buffer.as_ptr() as *const f32);
                }
            }
            UniformDataType::Int => {
                let mut buffer: Vec<u8> = Vec::with_capacity(std::mem::size_of::<i32>());
                (self.callback)(engine, sprite, &mut buffer);
                unsafe {
                    let value = *(buffer.as_ptr() as *const i32);
                    gl::Uniform1i(location, value);
                }
            }
            UniformDataType::Bool => {
                let mut buffer: Vec<u8> = Vec::with_capacity(std::mem::size_of::<bool>());
                (self.callback)(engine, sprite, &mut buffer);
                unsafe {
                    let value = *(buffer.as_ptr() as *const bool);
                    gl::Uniform1i(location, value as GLint);
                }
            }
            UniformDataType::UInt => {
                let mut buffer: Vec<u8> = Vec::with_capacity(std::mem::size_of::<u32>());
                (self.callback)(engine, sprite, &mut buffer);
                unsafe {
                    let value = *(buffer.as_ptr() as *const u32);
                    gl::Uniform1ui(location, value);
                }
            }
            UniformDataType::Sampler2D => {
                let mut buffer: Vec<u8> = Vec::with_capacity(std::mem::size_of::<GLuint>());
                (self.callback)(engine, sprite, &mut buffer);
                unsafe {
                    let tex = *(buffer.as_ptr() as *const GLuint);
                    gl::ActiveTexture(gl::TEXTURE0);
                    gl::BindTexture(gl::TEXTURE_2D, tex);
                }
            }
        };
    }

    pub fn texture_from_sprite_sheet(name: String) -> Self {
        Self {
            name,
            callback: |engine: &Engine, sprite: &Sprite, result: &mut Vec<u8>| {
                let sprite_sheet_id = sprite.get_sprite_sheet();
                let data = engine.get_texture_from_sprite_sheet(sprite_sheet_id).unwrap();
                Uniform::push_data_to_result(result, &[data]);
            },
            data_type: UniformDataType::Sampler2D,
        }
    }
}
 
// FIXME implement own PartialEq function instead of just ignoring warning
#[allow(unpredictable_function_pointer_comparisons)]
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Attribute {
    name: String,
    location: u32,
    callback: fn(&Engine, &Sprite, &mut Vec<u8>),
    data_type: AttributeDataType,
}

impl Attribute {
    pub fn new(name: String, location: u32, callback: fn(&Engine, &Sprite, &mut Vec<u8>), data_type: AttributeDataType) -> Self {
        Self {
            name,
            location,
            callback,
            data_type,
        }
    }

    fn write_to_buffer(&self, engine: &Engine, sprite: &Sprite, buffer: &mut Vec<u8>) {
        (self.callback)(engine, sprite, buffer);
    }

    pub fn write_data_to_buffer<T: Copy>(buffer: &mut Vec<u8>, data: &[T]) {
        let ptr = data.as_ptr() as *const u8;
        let size = data.len() * std::mem::size_of::<T>();
        unsafe {
            buffer.extend_from_slice(std::slice::from_raw_parts(ptr, size));
        }
    }

    /// A preset Attribute that gets the position for each vertex of a sprite in NDC.
    pub fn position(name: String, location: u32) -> Self {
        Self::new(
            name,
            location,
            |engine: &Engine, sprite: &Sprite, buffer: &mut Vec<u8>| {
                let (w_width, w_height) = engine.get_window_dimensions();
                let (s_width, s_height) = (sprite.get_width(), sprite.get_height());
                let pos = sprite.get_position();
                let bottom_left = pos;
                let bottom_right = (pos.0 + s_width as i32, pos.1);
                let top_left = (pos.0, pos.1 + s_height as i32);
                let top_right = (pos.0 + s_width as i32, pos.1 + s_height as i32);
                let aspect_ratio = w_width as f32 / w_height as f32;

                let data = [
                    [
                        2.0 * bottom_left.0 as f32 / w_width as f32 - 1.0,
                        2.0 * (bottom_left.1 as f32 / aspect_ratio) / w_height as f32 - 1.0
                    ],
                    [
                        2.0 * bottom_right.0 as f32 / w_width as f32 - 1.0,
                        2.0 * (bottom_right.1 as f32 / aspect_ratio) / w_height as f32 - 1.0
                    ],
                    [
                        2.0 * top_left.0 as f32 / w_width as f32 - 1.0,
                        2.0 * (top_left.1 as f32 / aspect_ratio) / w_height as f32 - 1.0
                    ],
                    [
                        2.0 * top_right.0 as f32 / w_width as f32 - 1.0,
                        2.0 * (top_right.1 as f32 / aspect_ratio) / w_height as f32 - 1.0
                    ],
                ];
                Attribute::write_data_to_buffer(buffer, &data);
            },
            AttributeDataType::FloatVec2,
        )
    }

    /// A preset Attribute that returns the (u, v) texture position for each vertex of a sprite.
    pub fn texture_uv_from_sprite_sheet(name: String, location: u32) -> Self {
        Self::new(
            name,
            location,
            |engine: &Engine, sprite: &Sprite, buffer: &mut Vec<u8>| {
                let sprite_sheet = sprite.get_sprite_sheet();
                let index = sprite.get_sprite_sheet_index();
                let (u_min, v_min, u_max, v_max) = engine.get_uv_from_sprite_sheet(sprite_sheet, index).unwrap();
                let data = [
                    [u_min, v_min],
                    [u_max, v_min],
                    [u_min, v_max],
                    [u_max, v_max],
                ];
                Attribute::write_data_to_buffer(buffer, &data);
            },
            AttributeDataType::FloatVec2,
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
    fn set_attribute(&self, attribute: &Attribute, offset: usize) -> usize {
        let (len, size, gl_type) = match attribute.data_type {
            AttributeDataType::Float     => (1, size_of::<f32>(), gl::FLOAT),
            AttributeDataType::FloatVec2 => (2, size_of::<f32>(), gl::FLOAT),
            AttributeDataType::FloatVec3 => (3, size_of::<f32>(), gl::FLOAT),
            AttributeDataType::FloatVec4 => (4, size_of::<f32>(), gl::FLOAT),
            AttributeDataType::Int       => (1, size_of::<i32>(), gl::INT),
            AttributeDataType::Bool      => (1, size_of::<bool>(), gl::UNSIGNED_BYTE),
            AttributeDataType::UInt      => (1, size_of::<u32>(), gl::UNSIGNED_INT),
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
        (len as usize * size * 4) + offset
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
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

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.target, self.id);
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct ShaderProgram {
    id: GLuint,
    attributes: Vec<Attribute>,
    uniforms: Vec<Uniform>,
    vao: VertexArray,
    vbo: VertexBuffer,
    ebo: VertexBuffer,
    sprite_size_bytes: usize,
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
        uniforms: Vec<Uniform>,
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
                
                let mut log = vec![0u8; log_length as usize];
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

            let ebo = VertexBuffer::new(gl::ELEMENT_ARRAY_BUFFER);
            ebo.bind();

            let mut offset: usize = 0;
            let vao = VertexArray::new();
            vao.bind();
            for attribute in attributes.iter() {
                offset = vao.set_attribute(attribute, offset);
            }

            let bytes_per_sprite = offset as usize;
            let sprite_capacity: usize = 10_000;
            vbo.bind();

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (sprite_capacity * bytes_per_sprite) as isize,
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            );

            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (sprite_capacity * 6 * std::mem::size_of::<u32>()) as isize,
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            );

            Ok(Self {
                id,
                attributes,
                uniforms,
                vao,
                vbo,
                ebo,
                sprite_size_bytes: bytes_per_sprite,
            })
        }
    }

    pub fn sprite_size_bytes(&self) -> usize {
        self.sprite_size_bytes
    }

    pub fn apply_uniforms(&self, engine: &Engine, sprite: &Sprite) {
        for uniform in self.uniforms() {
            uniform.bind(self.id, engine, sprite)
        }
    }

    pub fn fill_ebo(&self, total_sprites: usize) {
        let mut indices: Vec<u32> = Vec::with_capacity(total_sprites * 6);

        for i in 0..total_sprites {
            let base: u32 = i as u32 * 4;
            indices.push(base);
            indices.push(base + 1);
            indices.push(base + 2);

            indices.push(base + 2);
            indices.push(base + 1);
            indices.push(base + 3);
        }

        unsafe {
            gl::BufferSubData(
                gl::ELEMENT_ARRAY_BUFFER,
                0,
                (indices.len() * std::mem::size_of::<u32>()) as isize,
                indices.as_ptr() as *const _,
            );
        }
    }

    pub fn fill_vbo(&self, engine: &Engine, sprites: &Vec<&Sprite>, sprite_size: usize) {
        let mut buffer = Vec::with_capacity(sprite_size as usize * sprites.len());

        for sprite in sprites {
            for attribute in self.attributes() {
                attribute.write_to_buffer(engine, sprite, &mut buffer);
            }
        }

        unsafe {
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                buffer.len() as isize,
                buffer.as_ptr() as *const _,
            );
        }
    }

    pub fn attributes(&self) -> &Vec<Attribute> {
        &self.attributes
    }

    pub fn uniforms(&self) -> &Vec<Uniform> {
        &self.uniforms
    }

    pub fn apply(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }

        self.vao.bind();
        self.vbo.bind();
        self.ebo.bind();
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

            let mut log = vec![0u8; log_length as usize];

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
