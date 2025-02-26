#pragma once

#include <cstdint>

struct ShaderId {
    uint32_t id;

    bool operator==(const ShaderId &other) const {
        return id == other.id;
    }
};
