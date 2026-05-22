#ifndef HonzoErrorCode_HPP
#define HonzoErrorCode_HPP

#include "HonzoErrorCode.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {

} // namespace capi
} // namespace

inline diplomat::capi::HonzoErrorCode HonzoErrorCode::AsFFI() const {
    return static_cast<diplomat::capi::HonzoErrorCode>(value);
}

inline HonzoErrorCode HonzoErrorCode::FromFFI(diplomat::capi::HonzoErrorCode c_enum) {
    switch (c_enum) {
        case diplomat::capi::HonzoErrorCode_Ok:
        case diplomat::capi::HonzoErrorCode_InvalidMagic:
        case diplomat::capi::HonzoErrorCode_ReaderVersionTooOld:
        case diplomat::capi::HonzoErrorCode_BufferTooShort:
        case diplomat::capi::HonzoErrorCode_CrcMismatch:
        case diplomat::capi::HonzoErrorCode_EncryptedChunk:
        case diplomat::capi::HonzoErrorCode_InvalidMathML:
        case diplomat::capi::HonzoErrorCode_Truncated:
        case diplomat::capi::HonzoErrorCode_Unknown:
            return static_cast<HonzoErrorCode::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // HonzoErrorCode_HPP
