#pragma once
#include "ffi.h"
#include "color.h"
#include "sprite.h"

#include <cstddef>
#include <cstdint>
#include <string>

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
