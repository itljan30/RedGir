#pragma once
#include "ffi.h"
#include "color.h"
#include "sprite.h"

#include <cstddef>
#include <cstdint>
#include <string>

// FIXME this is the 4th time I've redefined this enum so it's very fragile. It should be fine for now,
// since it's pretty stable, but could be the source of annoying bugs in the future.
enum class Key {
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

enum class Action {
    Pressed,
    Released,
    Held,
    None,
};

class EngineBuilder;

class Engine {
public:
    ~Engine();

    static EngineBuilder create();
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


    Action getKeyState(Key key) const;
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
