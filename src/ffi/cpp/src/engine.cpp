#include "engine.h"
#include "color.h"
#include "sprite.h"

#include <cstddef>
#include <cstdint>
#include <stdexcept>
#include <string>

EngineBuilder::EngineBuilder(EngineBuilderC *engineBuilder) : m_engineBuilder(engineBuilder) {
    if (!m_engineBuilder) {
        throw std::runtime_error("Failed to create EngineBuilder instance");
    }
}

EngineBuilder::~EngineBuilder() {
    EngineBuilderC_free(m_engineBuilder);
}

Engine EngineBuilder::init() {
    return Engine { EngineBuilderC_init(m_engineBuilder) };
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

EngineBuilder &EngineBuilder::setWindowSize(uint32_t width, uint32_t height)  {
    EngineBuilderC_setWindowSize(m_engineBuilder, width, height);
    return *this;
}

EngineBuilder &EngineBuilder::setClearColor(Color color)  {
    auto [r, g, b, a] = color.toTuple();
    EngineBuilderC_setClearColor(m_engineBuilder, r, g, b, a);
    return *this;
}

EngineBuilder &EngineBuilder::setWindowName(const std::string &name) {
    EngineBuilderC_setWindowName(m_engineBuilder, name.c_str());
    return *this;
}


Engine::Engine(EngineC *engine) : m_engine(engine) {
    if (!m_engine) {
        throw std::runtime_error("Failed to create Engine instance");
    }
}

bool Engine::isRunning() const {
    return EngineC_isRunning(m_engine);
}

Sprite Engine::getSprite(SpriteSheetId sheet, size_t index) const {
    return Sprite { EngineC_getSprite(m_engine, sheet.id, index) };
}

uint32_t Engine::getTextureFromSpriteSheet(SpriteSheetId sheet) const {
    return EngineC_getTextureFromSpriteSheet(m_engine, sheet.id);
}

ShaderId Engine::defaultShader() const {
    return ShaderId { EngineC_defaultShader(m_engine) };
}

uint64_t Engine::timeSinceInitializationMilis() const {
    return EngineC_timeSinceInitializationMilis(m_engine);
}

float Engine::timeSinceInitializationSeconds() const {
    return EngineC_timeSinceInitializationSeconds(m_engine);
}

std::tuple<int32_t, int32_t> Engine::getWindowDimensions() const {
    WindowDimensionsC dim = EngineC_getWindowDimensions(m_engine);
    return std::tuple<int32_t, int32_t>(dim.x, dim.y);
}

std::tuple<float, float, float, float> Engine::getUVFromSpriteSheet(SpriteSheetId sheet, size_t index) const {
    UVCoordsC coords = EngineC_getUVFromSpriteSheet(m_engine, sheet.id, index);
    return std::tuple<float, float, float, float>(coords.minU, coords.minV, coords.maxU, coords.maxV);
}

// TODO
// FragmentShader Engine::getDefaultFragmentShader() const {
//
// }

// TODO
// VertexShasder Engine::getDefaultVertexShader() const {
//
// }

EngineBuilder Engine::create() {
    return EngineBuilder{ EngineC_new() };
}

SpriteId Engine::addQuad(
    Color color, int32_t xPos, int32_t yPos, int32_t layer,
    uint32_t width, uint32_t height, ShaderId shader
) {
    auto [r, g, b, a] = color.toTuple();
    return SpriteId { EngineC_addQuad(m_engine, r, g, b, a, xPos, yPos, layer, width, height, shader.id) };
}

SpriteSheetId Engine::addSpriteSheet(const std::string &path, uint32_t spriteWidth, uint32_t spriteHeight) {
    return SpriteSheetId { EngineC_addSpriteSheet(m_engine, path.c_str(), spriteWidth, spriteHeight) };
}

SpriteId Engine::addSprite(
    SpriteSheetId sheet, size_t spriteIndex,
    int32_t xPos, int32_t yPos, int32_t layer,
    uint32_t width, uint32_t height, ShaderId shader
) {
    return SpriteId { EngineC_addSprite(m_engine, sheet.id, spriteIndex, xPos, yPos, layer, width, height, shader.id) };
}

void Engine::removeSprite(SpriteId spriteId) {
    EngineC_removeSprite(m_engine, spriteId.id);
}

void Engine::toggleFullscreen() {
    EngineC_toggleFullscreen(m_engine);
}

void Engine::setWindowSize(int32_t width, int32_t height) {
    EngineC_setWindowSize(m_engine, width, height);
}

void Engine::toggleShowFps() {
    EngineC_toggleShowFps(m_engine);
}

void Engine::setFps(float targetFps) {
    EngineC_setFps(m_engine, targetFps);
}

void Engine::stop() {
    EngineC_stop(m_engine);
}

void Engine::toggleBorder() {
    EngineC_toggleBorder(m_engine);
}

void Engine::drawFrame() {
    EngineC_drawFrame(m_engine);
}
