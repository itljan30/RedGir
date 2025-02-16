#include "timer.h"

#include <cstdint>

Timer::Timer() : timer(TimerC_new()) {}
Timer::~Timer() {}

void Timer::reset() {
    TimerC_reset(timer);
}

float Timer::getElapsedSeconds() const {
    return TimerC_getElapsedSeconds(timer);
}

uint64_t Timer::getElapsedMilis() const {
    return TimerC_getElapsedMilis(timer);
}
