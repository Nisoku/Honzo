#ifndef HonzoErrorCode_D_HPP
#define HonzoErrorCode_D_HPP

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
    enum HonzoErrorCode {
      HonzoErrorCode_Ok = 0,
      HonzoErrorCode_InvalidMagic = 1,
      HonzoErrorCode_ReaderVersionTooOld = 2,
      HonzoErrorCode_BufferTooShort = 3,
      HonzoErrorCode_CrcMismatch = 4,
      HonzoErrorCode_EncryptedChunk = 5,
      HonzoErrorCode_InvalidMathML = 6,
      HonzoErrorCode_Truncated = 7,
      HonzoErrorCode_Unknown = 255,
    };

    typedef struct HonzoErrorCode_option {union { HonzoErrorCode ok; }; bool is_ok; } HonzoErrorCode_option;
} // namespace capi
} // namespace

class HonzoErrorCode {
public:
    enum Value {
        Ok = 0,
        InvalidMagic = 1,
        ReaderVersionTooOld = 2,
        BufferTooShort = 3,
        CrcMismatch = 4,
        EncryptedChunk = 5,
        InvalidMathML = 6,
        Truncated = 7,
        Unknown = 255,
    };

    HonzoErrorCode(): value(Value::Ok) {}

    // Implicit conversions between enum and ::Value
    constexpr HonzoErrorCode(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline diplomat::capi::HonzoErrorCode AsFFI() const;
    inline static HonzoErrorCode FromFFI(diplomat::capi::HonzoErrorCode c_enum);
private:
    Value value;
};


#endif // HonzoErrorCode_D_HPP
