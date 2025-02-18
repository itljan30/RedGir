#pragma once

#include "shader.h"
#include "color.h"

#include <cstddef>
#include <cstdint>
#include <tuple>
#include <string>

extern "C" {
    typedef struct UVCoordsC {
        float minU;
        float minV;
        float maxU;
        float maxV;
    } UVCoordsC;

    typedef struct SpriteSheetC SpriteSheetC;
    UVCoordsC SpriteSheetC_getUV(const SpriteSheetC *sheet, size_t index);
    uint32_t SpriteSheetC_getTexture(const SpriteSheetC *sheet);
    SpriteSheetC *SpriteSheetC_fromImage(const char *path, uint32_t spriteWidth, uint32_t spriteHeight);
    SpriteSheetC *SpriteSheetC_fromColor(uint8_t r, uint8_t g, uint8_t b, uint8_t a);
    void SpriteSheetC_free(SpriteSheetC *sheet);

    typedef enum FlipC {
        None,
        FlipX,
        FlipY,
        FlipXY
    } FlipC;

    typedef struct PositionC {
        int32_t x;
        int32_t y;
    } PositionC;

    typedef struct SpriteC SpriteC;
    SpriteC *SpriteC_new(
        uint32_t spriteSheet, size_t spriteSheetIndex, 
        int32_t xPosition, int32_t yPosition, int32_t layer, 
        uint32_t width, uint32_t height, uint32_t shader
    );
    void SpriteC_free(SpriteC *sprite);
    float SpriteC_getRotation(const SpriteC *sprite);
    FlipC SpriteC_getFlip(const SpriteC *sprite);
    size_t SpriteC_getSpriteSheetIndex(const SpriteC *sprite);
    uint32_t SpriteC_getSpriteSheet(const SpriteC *sprite);
    PositionC SpriteC_getPosition(const SpriteC *sprite);
    void SpriteC_translate(SpriteC *sprite, int32_t dx, int32_t dy); 
    void SpriteC_setId(SpriteC *sprite, uint32_t id);
    uint32_t SpriteC_getShader(const SpriteC *sprite);
    void SpriteC_setShader(SpriteC *sprite, uint32_t shader);
    void SpriteC_setTexture(SpriteC *sprite, uint32_t spriteSheet, size_t index);
    void SpriteC_setHeight(SpriteC *sprite, uint32_t height);
    void SpriteC_setWidth(SpriteC *sprite, uint32_t width);
    void SpriteC_setPosition(SpriteC *sprite, int32_t x, int32_t y);
    void SpriteC_scale(SpriteC *sprite, float scale_x, float scale_y);
    void SpriteC_setRotation(SpriteC *sprite, float rotation);
    void SpriteC_setFlip(SpriteC *sprite, FlipC flip);
    int32_t SpriteC_getLayer(const SpriteC *sprite);
    uint32_t SpriteC_getWidth(const SpriteC *sprite);
    uint32_t SpriteC_getHeight(const SpriteC *sprite);
}

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
    SpriteC *m_sprite;
};
