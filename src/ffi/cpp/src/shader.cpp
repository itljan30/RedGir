#include "ffi.h"
#include "RedGir/shader.h"

VertexShader::VertexShader(std::string &source) {
    VertexShaderC_new(source.c_str());
}

VertexShader::~VertexShader() {
    VertexShaderC_free(m_shader);
}

FragmentShader::FragmentShader(std::string &source) {
    FragmentShaderC_new(source.c_str());
}

FragmentShader::~FragmentShader() {
    FragmentShaderC_free(m_shader);
}
