#include "sprite.h"
#include "shader.h"
#include "color.h"

#include <cstdint>
#include <cstddef>
#include <stdexcept>
#include <tuple>
#include <string>

SpriteSheet::SpriteSheet(SpriteSheetC *sheet) : m_sheet(sheet) {
    if (!m_sheet) {
        std::runtime_error("Failed to create SpriteSheet instance");
    }
}

SpriteSheet::~SpriteSheet() {
    SpriteSheetC_free(m_sheet);
}

SpriteSheet SpriteSheet::fromImage(const std::string &path, uint32_t spriteWidth, uint32_t spriteHeight) {
    SpriteSheetC *sheet = SpriteSheetC_fromImage(path.c_str(), spriteWidth, spriteHeight);
    return SpriteSheet(sheet);
}

SpriteSheet SpriteSheet::fromColor(Color color) {
    auto [r, g, b, a] = color.toTuple();
    SpriteSheetC *sheet = SpriteSheetC_fromColor(r, g, b, a);
    return SpriteSheet(sheet);
}

std::tuple<float, float, float, float> SpriteSheet::getUV(size_t index) const {
    UVCoordsC coords = SpriteSheetC_getUV(m_sheet, index);
    return std::tuple<float, float, float, float>(coords.minU, coords.minV, coords.maxU, coords.maxV);
}

uint32_t SpriteSheet::getTexture() const {
    return SpriteSheetC_getTexture(m_sheet);
}

Sprite::Sprite(SpriteC *sprite) : m_sprite(sprite) {
    if (!m_sprite) {
        throw std::runtime_error("Failed to create Sprite instance");
    }
}

Sprite::Sprite(
    SpriteSheetId spriteSheet,
    size_t index,
    int32_t xPosition,
    int32_t yPosition,
    int32_t layer,
    uint32_t width,
    uint32_t height,
    ShaderId shader
) {
    m_sprite = SpriteC_new(spriteSheet.id, index, xPosition, yPosition, layer, width, height, shader.id);
    if (!m_sprite) {
        throw std::runtime_error("Failed to create Sprite instance");
    }
}

Sprite::~Sprite() {
    SpriteC_free(m_sprite);
}

float Sprite::getRotation() const {
    return SpriteC_getRotation(m_sprite);
}

Flip Sprite::getFlip() const {
    return static_cast<Flip>(SpriteC_getFlip(m_sprite));
}

size_t Sprite::getSpriteSheetIndex() const {
    return SpriteC_getSpriteSheetIndex(m_sprite);
}

SpriteSheetId Sprite::getSpriteSheet() const {
    return SpriteSheetId { SpriteC_getSpriteSheet(m_sprite) };
}

std::tuple<int32_t, int32_t> Sprite::getPosition() const {
    PositionC pos = SpriteC_getPosition(m_sprite);
    return std::tuple<int32_t, int32_t>(pos.x, pos.y);
}

ShaderId Sprite::getShader() const {
    return ShaderId { SpriteC_getShader(m_sprite) };
}

int32_t Sprite::getLayer() const {
    return SpriteC_getLayer(m_sprite);
}

uint32_t Sprite::getWidth() const {
    return SpriteC_getWidth(m_sprite);
}

uint32_t Sprite::getHeight() const {
    return SpriteC_getHeight(m_sprite);
}

Sprite &Sprite::translate(int32_t dx, int32_t dy) {
    SpriteC_translate(m_sprite, dx, dy);
    return *this;
}

Sprite &Sprite::setId(SpriteId id) {
    SpriteC_setId(m_sprite, id.id);
    return *this;
}

Sprite &Sprite::setShader(ShaderId id) {
    SpriteC_setShader(m_sprite, id.id);
    return *this;
}

Sprite &Sprite::setTexture(SpriteSheetId spriteSheet, size_t index) {
    SpriteC_setTexture(m_sprite, spriteSheet.id, index);
    return *this;
}

Sprite &Sprite::setHeight(uint32_t height) {
    SpriteC_setHeight(m_sprite, height);
    return *this;
}

Sprite &Sprite::setWidth(uint32_t width) {
    SpriteC_setWidth(m_sprite, width);
    return *this;
}

Sprite &Sprite::setPosition(int32_t x, int32_t y) {
    SpriteC_setPosition(m_sprite, x, y);
    return *this;
}

Sprite &Sprite::scale(float xScale, float yScale) {
    SpriteC_scale(m_sprite, xScale, yScale);
    return *this;
}

Sprite &Sprite::setRotation(float rotation) {
    SpriteC_setRotation(m_sprite, rotation);
    return *this;
}

Sprite &Sprite::setFlip(Flip flip) {
    SpriteC_setFlip(m_sprite, static_cast<FlipC>(flip));
    return *this;
}
