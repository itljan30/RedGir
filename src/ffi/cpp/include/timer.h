#pragma once

#include "ffi.h"

#include <cstdint>

class Timer {
public: 
    Timer();
    ~Timer();
    
    void reset();
    float getElapsedSeconds() const;
    uint64_t getElapsedMilis() const;

private:
    TimerC *m_timer;
};
