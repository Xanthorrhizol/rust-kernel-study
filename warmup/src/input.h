#pragma once

#include <cstdint>
#include <sstream>
#include <stdint.h>
#include <string>

class Test {
    public:
        Test() : cnt(0) {}
        void inc();
        std::string to_string() const;
    private:
        uint32_t cnt;
};

inline uint32_t x3(uint32_t a) {
    return a * 3;
}

inline void Test::inc() { cnt++; }
inline std::string Test::to_string() const {
    std::ostringstream oss;
    oss << "Test: Called " << cnt << " times.";
    return oss.str();
}
