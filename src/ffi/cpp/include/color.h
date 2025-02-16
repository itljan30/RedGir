#pragma once

#include <cstdint>
#include <tuple>

class Color {
public:
    Color(uint8_t r, uint8_t g, uint8_t b, uint8_t a);

    std::tuple<uint8_t, uint8_t, uint8_t, uint8_t> toTuple() const;

private:
    uint8_t m_r;
    uint8_t m_g;
    uint8_t m_b;
    uint8_t m_a;
};
