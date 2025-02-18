#pragma once

#include "color.h"
#include "sprite.h"

#include <cstddef>
#include <cstdint>
#include <string>

extern "C" {
    typedef struct WindowDimensionsC {
        int32_t x;
        int32_t y;
    } WindowDimensionsC;

    typedef struct EngineBuilderC EngineBuilderC;
    typedef struct EngineC EngineC;

    EngineC *EngineBuilderC_init(EngineBuilderC *engineBuilder);
    void EngineBuilderC_free(EngineBuilderC *engineBuilder);
    void EngineBuilderC_hideCursor(EngineBuilderC *engineBuilder);
    void EngineBuilderC_borderless(EngineBuilderC *engineBuilder);
    void EngineBuilderC_notResizable(EngineBuilderC *engineBuilder);
    void EngineBuilderC_pollMouseButtons(EngineBuilderC *engineBuilder);
    void EngineBuilderC_pollCursor(EngineBuilderC *engineBuilder);
    void EngineBuilderC_pollKeyboard(EngineBuilderC *engineBuilder);
    void EngineBuilderC_setWindowSize(EngineBuilderC *engineBuilder, uint32_t width, uint32_t height);
    void EngineBuilderC_setClearColor(EngineBuilderC *engineBuilder, uint8_t r, uint8_t g, uint8_t b, uint8_t a);
    void EngineBuilderC_setWindowName(EngineBuilderC *engineBuilder, const char *name);

    EngineBuilderC *EngineC_new();
    uint32_t EngineC_defaultShader(const EngineC *engine);
    uint32_t EngineC_addQuad(
        EngineC *engine, uint8_t r, uint8_t g, uint8_t b, uint8_t a,
        int32_t xPos, int32_t yPos, int32_t layer, uint32_t width, uint32_t height, uint32_t shader
    );
    uint64_t EngineC_timeSinceInitializationMilis(const EngineC *engine);
    float EngineC_timeSinceInitializationSeconds(const EngineC *engine);
    WindowDimensionsC EngineC_getWindowDimensions(const EngineC *engine);
    uint32_t EngineC_addSpriteSheet(EngineC *engine, const char *path, uint32_t spriteWidth, uint32_t spriteHeight);
    // TODO EngineC_addShaderProgram
    SpriteC *EngineC_getSprite(const EngineC *engine, uint32_t sheet, size_t index);
    // TODO EngineC_getAllSprites
    uint32_t EngineC_addSprite(
        EngineC *engine, uint32_t spriteSheet, size_t spriteindex,
        int32_t xPos, int32_t yPos, int32_t layer,
        uint32_t width, uint32_t height, uint32_t shader
    );
    void EngineC_removeSprite(EngineC *engine, uint32_t spriteId);
    void EngineC_toggleFullscreen(EngineC *engine);
    void EngineC_setWindowSize(EngineC *engine,int32_t width, int32_t height);
    void EngineC_toggleShowFps(EngineC *engine);
    void EngineC_setFps(EngineC *engine, float targetFps);
    void EngineC_stop(EngineC *engine);
    // TODO EngineC_getKeyEvents
    bool EngineC_isRunning(const EngineC *engine);
    void EngineC_toggleBorder(EngineC *engine);
    uint32_t EngineC_getDefaultFragmentShader(const EngineC *engine);
    uint32_t EngineC_getDefaultVertexShader(const EngineC *engine);
    void EngineC_drawFrame(EngineC *engine);
    UVCoordsC EngineC_getUVFromSpriteSheet(const EngineC *engine, uint32_t spriteSheet, size_t index);
    uint32_t EngineC_getTextureFromSpriteSheet(const EngineC *engine, uint32_t spriteSheet);
    void EngineC_free(EngineC *engine);
}

class EngineBuilder;

class Engine {
public:
    ~Engine();

    EngineBuilder create();
    SpriteId addQuad(
        Color color, int32_t xPos, int32_t yPos, int32_t layer,
        uint32_t width, uint32_t height, ShaderId shader
    );
    SpriteSheetId addSpriteSheet(const std::string &path, uint32_t spriteWidth, uint32_t spriteHeight);
    SpriteId addSprite(
        SpriteSheetId sheet, size_t spriteIndex,
        int32_t xPos, int32_t yPos, int32_t layer,
        uint32_t width, uint32_t height, ShaderId shader
    );
    void removeSprite(SpriteId spriteId);
    void toggleFullscreen();
    void setWindowSize(int32_t width, int32_t height);
    void toggleShowFps();
    void setFps(float targetFps);
    void stop();
    void toggleBorder();
    void drawFrame();

    bool isRunning() const;
    Sprite getSprite(SpriteSheetId sheet, size_t index) const;
    uint32_t getTextureFromSpriteSheet(SpriteSheetId sheet) const;
    ShaderId defaultShader() const;
    uint64_t timeSinceInitializationMilis() const;
    float timeSinceInitializationSeconds() const;
    std::tuple<int32_t, int32_t> getWindowDimensions() const;
    std::tuple<float, float, float, float> getUVFromSpriteSheet(SpriteSheetId sheet, size_t index) const;
    // TODO FragmentShader getDefaultFragmentShader() const;
    // TODO VertexShader getDefaultVertexShader() const;

private:
    Engine(EngineC *engine);

private:
    friend class EngineBuilder;

    EngineC *m_engine;
};

class EngineBuilder {
public:
    ~EngineBuilder();

    Engine init();
    EngineBuilder &hideCursor();
    EngineBuilder &borderless();
    EngineBuilder &notResizable();
    EngineBuilder &pollMouseButtons();
    EngineBuilder &pollCursor();
    EngineBuilder &pollKeyboard();
    EngineBuilder &setWindowSize(uint32_t width, uint32_t height);
    EngineBuilder &setClearColor(Color color);
    EngineBuilder &setWindowName(const std::string &name);

private:
    EngineBuilder(EngineBuilderC *engineBuilder);

private:
    friend class Engine;

    EngineBuilderC *m_engineBuilder;
};
