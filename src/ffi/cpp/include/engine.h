#pragma once

#include "color.h"

#include <cstdint>
#include <string>

extern "C" {
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

    // EngineC functions
}

class Engine;

class EngineBuilder {
public:
    EngineBuilder(EngineBuilderC *engineBuilder);
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
    EngineBuilderC *m_engineBuilder;
};

class Engine {
public:
    Engine(EngineC *engine);

private:
    EngineC *m_engine;
};
