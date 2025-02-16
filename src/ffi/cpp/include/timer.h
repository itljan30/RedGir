#pragma once

#include <cstdint>

extern "C" {
    typedef struct TimerC TimerC;
    TimerC *TimerC_new();
    void TimerC_reset(TimerC *timer);
    float TimerC_getElapsedSeconds(const TimerC *timer);
    uint64_t TimerC_getElapsedMilis(const TimerC *timer);

}   

class Timer {
public: 
    Timer();
    ~Timer();
    
    void reset();
    float getElapsedSeconds() const;
    uint64_t getElapsedMilis() const;

private:
    TimerC *timer;
};
