#pragma once

#include "ffi.h"
#include "shader.h"
#include "color.h"

#include <cstddef>
#include <cstdint>
#include <tuple>
#include <string>

struct SpriteSheetId {
    uint32_t id;
};

struct SpriteId {
    uint32_t id;
};

enum class Flip {
    None,
    FlipX,
    FlipY,
    FlipXY,
};

class SpriteSheet {
public:
    ~SpriteSheet();

    static SpriteSheet fromImage(const std::string &path, uint32_t spriteWidth, uint32_t spriteHeight);
    static SpriteSheet fromColor(Color color);

    std::tuple<float, float, float, float> getUV(size_t index) const;
    uint32_t getTexture() const;

private: 
    SpriteSheet(SpriteSheetC *sheet);

private:
    friend class Engine;
    SpriteSheetC *m_sheet;
};

class Sprite {
public:
    Sprite(
        SpriteSheetId spriteSheet,
        size_t index,
        int32_t xPosition,
        int32_t yPosition,
        int32_t layer,
        uint32_t width,
        uint32_t height,
        ShaderId shader
    );
    ~Sprite();

    float getRotation() const;
    Flip getFlip() const;
    size_t getSpriteSheetIndex() const;
    SpriteSheetId getSpriteSheet() const;
    std::tuple<int32_t, int32_t> getPosition() const;
    ShaderId getShader() const;
    int32_t getLayer() const;
    uint32_t getWidth() const;
    uint32_t getHeight() const;

    Sprite &translate(int32_t dx, int32_t dy);
    Sprite &setId(SpriteId id);
    Sprite &setShader(ShaderId id);
    Sprite &setTexture(SpriteSheetId spriteSheet, size_t index);
    Sprite &setHeight(uint32_t height);
    Sprite &setWidth(uint32_t width);
    Sprite &setPosition(int32_t x, int32_t y);
    Sprite &scale(float xScale, float yScale);
    Sprite &setRotation(float rotation);
    Sprite &setFlip(Flip flip);

private:
    Sprite(SpriteC *sprite);

private:
    friend class Engine;
    SpriteC *m_sprite;
};
