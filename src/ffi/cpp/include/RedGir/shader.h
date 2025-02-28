#pragma once

// #include "ffi.h"

// #include <string>
#include <cstdint>

class Engine;
class Sprite;

struct ShaderId {
    uint32_t id;

    bool operator==(const ShaderId &other) const {
        return id == other.id;
    }
};

// class VertexShader {
// public:
//     VertexShader(std::string &source);
//     ~VertexShader();
//
// private:
//     VertexShaderC *m_shader;
// };
//
// class FragmentShader {
// public:
//     FragmentShader(std::string &source);
//     ~FragmentShader();
//
// private:
//     FragmentShaderC *m_shader;
// };
//
// using UniformDataType = UniformDataTypeC;
// using UniformDataValue = UniformDataValueC;
// using AttributeDataType = AttributeDataTypeC;
// using AttributeDataValue = AttributeDataValueC;
//
// struct UniformData {
//     const UniformDataType kind;
//     const UniformDataValue (*func)(const Engine &engine, const Sprite &sprite);
//
//     UniformData(const UniformDataType kind, const UniformDataValue (*func)(const Engine &engine, const Sprite &sprite))
//         : kind(kind), func(func) {}
// };
//
// struct Uniform {
//     const std::string &name;
//     const UniformData callback;
//
//     Uniform(const std::string &name, const UniformData callback) : name(name), callback(callback) {};
// };
//
// struct AttributeData {
//     const AttributeDataType kind;
//     const AttributeDataValue (*func)(const Engine &engine, const Sprite &sprite);
// };
//
// struct Attribute {
//     const std::string &name;
//     const uint32_t location;
//     const AttributeData callback;
//
//     Attribute(const std::string &name, const uint32_t location, const AttributeData callback)
//         : name(name), location(location), callback(callback) {};
// };
