#include "RedGir/color.h"

#include <cstdint>

Color::Color(uint8_t r, uint8_t g, uint8_t b, uint8_t a) : m_r(r), m_g(g), m_b(b), m_a(a) {}

std::tuple<uint8_t, uint8_t, uint8_t, uint8_t> Color::toTuple() const {
    return std::tuple<uint8_t, uint8_t, uint8_t, uint8_t>(m_r, m_g, m_b, m_a);
}
