#include "engine.h"
#include "color.h"

#include <cstdint>
#include <string>

EngineBuilder::EngineBuilder(EngineBuilderC *engineBuilder) : m_engineBuilder(engineBuilder) {}

EngineBuilder &EngineBuilder::init() {
    // TODO
    return *this;
}

EngineBuilder &EngineBuilder::hideCursor() {
    EngineBuilderC_hideCursor(m_engineBuilder);
    return *this;
}

EngineBuilder &EngineBuilder::borderless() {
    EngineBuilderC_borderless(m_engineBuilder);
    return *this;
}

EngineBuilder &EngineBuilder::notResizable() {
    EngineBuilderC_notResizable(m_engineBuilder);
    return *this;
}

EngineBuilder &EngineBuilder::pollMouseButtons() {
    EngineBuilderC_pollMouseButtons(m_engineBuilder);
    return *this;
}

EngineBuilder &EngineBuilder::pollCursor() {
    EngineBuilderC_pollCursor(m_engineBuilder);
    return *this;
}

EngineBuilder &EngineBuilder::pollKeyboard() {
    EngineBuilderC_pollKeyboard(m_engineBuilder);
    return *this;
}

EngineBuilder &EngineBuilder::setWindowSize(uint32_t width, uint32_t height) {
    EngineBuilderC_setWindowSize(m_engineBuilder, width, height);
    return *this;
}

EngineBuilder &EngineBuilder::setClearColor(Color color) {
    auto [r, g, b, a] = color.toTuple();
    EngineBuilderC_setClearColor(m_engineBuilder, r, g, b, a);
    return *this;
}

EngineBuilder &EngineBuilder::setWindowName(const std::string &name) {
    EngineBuilderC_setWindowName(m_engineBuilder, name.c_str());
    return *this;
}
