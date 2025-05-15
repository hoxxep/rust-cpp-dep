#include <cstdint>

extern "C" {
    uint64_t example_cpp(uint64_t input) noexcept {
        return 64 * input;
    }
}
