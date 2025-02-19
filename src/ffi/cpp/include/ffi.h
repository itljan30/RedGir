#pragma once

#include <cstddef>
#include <cstdint>

extern "C" {
    typedef struct TimerC TimerC;
    TimerC *TimerC_new();
    void TimerC_free(TimerC *timer);
    void TimerC_reset(TimerC *timer);
    float TimerC_getElapsedSeconds(const TimerC *timer);
    uint64_t TimerC_getElapsedMilis(const TimerC *timer);

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

    typedef struct WindowDimensionsC {
        int32_t x;
        int32_t y;
    } WindowDimensionsC;

    enum class ActionC {
        Pressed,
        Released,
        Held,
        None,
    };

    enum class KeyC {
    MouseLeft, MouseRight, MouseMiddle, MouseScrollUp, MouseScrollDown,
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, 
    Period, Comma, ForwardSlash, BackSlash, Space, Equals, Minus, Grave,
    Enter, Escape, Tab, Backspace, LeftBracket, RightBracket, Delete, Apostrophe, SemiColon,
    Number1, Number2, Number3, Number4, Number5, Number6, Number7, Number8, Number9, Number0,
    NumPad1, NumPad2, NumPad3, NumPad4, NumPad5, NumPad6, NumPad7, NumPad8, NumPad9, NumPad0,
    NumPadDecimal, NumPadEquals, NumPadEnter, NumPadMinus, NumPadAdd, NumPadDivide, NumPadMultiply,
    LeftShift, RightShift, LeftControl, RightControl, LeftAlt, RightAlt, LeftSuper, RightSuper,
    CapsLock, NumLock, ScrollLock,
    ArrowRight, ArrowLeft, ArrowDown, ArrowUp, Home, End, PageUp, PageDown, Insert,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    None,
    };

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
    ActionC EngineC_getKeyState(EngineC *engine, KeyC key);
    bool EngineC_isRunning(const EngineC *engine);
    void EngineC_toggleBorder(EngineC *engine);
    uint32_t EngineC_getDefaultFragmentShader(const EngineC *engine);
    uint32_t EngineC_getDefaultVertexShader(const EngineC *engine);
    void EngineC_drawFrame(EngineC *engine);
    UVCoordsC EngineC_getUVFromSpriteSheet(const EngineC *engine, uint32_t spriteSheet, size_t index);
    uint32_t EngineC_getTextureFromSpriteSheet(const EngineC *engine, uint32_t spriteSheet);
    void EngineC_free(EngineC *engine);
}
