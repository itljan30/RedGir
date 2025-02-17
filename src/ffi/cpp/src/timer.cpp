#include "timer.h"

#include <cstdint>
#include <stdexcept>

Timer::Timer() : m_timer(TimerC_new()) {
    if (!m_timer) {
        throw std::runtime_error("Failed to create Timer instance");
    }
}

Timer::~Timer() {
    TimerC_free(m_timer);
}

void Timer::reset() {
    TimerC_reset(m_timer);
}

uint64_t Timer::getElapsedMilis() const {
    return TimerC_getElapsedMilis(m_timer);
}

float Timer::getElapsedSeconds() const {
    return TimerC_getElapsedSeconds(m_timer);
}
