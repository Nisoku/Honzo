#ifndef HonzoBuilderHandle_HPP
#define HonzoBuilderHandle_HPP

#include "HonzoBuilderHandle.d.hpp"

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
    extern "C" {

    diplomat::capi::HonzoBuilderHandle* HonzoBuilderHandle_new(void);

    bool HonzoBuilderHandle_add_chunk(diplomat::capi::HonzoBuilderHandle* self, diplomat::capi::DiplomatU8View tag, diplomat::capi::DiplomatU8View data, uint8_t compression, uint8_t content_type_kind, uint8_t content_type_value);

    bool HonzoBuilderHandle_set_language(diplomat::capi::HonzoBuilderHandle* self, diplomat::capi::DiplomatStringView lang);

    bool HonzoBuilderHandle_set_auto_sidx(diplomat::capi::HonzoBuilderHandle* self, bool enable);

    bool HonzoBuilderHandle_add_math_chunk(diplomat::capi::HonzoBuilderHandle* self, diplomat::capi::DiplomatU8View data, uint8_t math_type, uint8_t compression);

    bool HonzoBuilderHandle_set_meta(diplomat::capi::HonzoBuilderHandle* self, diplomat::capi::DiplomatU8View msgpack);

    bool HonzoBuilderHandle_finalize(diplomat::capi::HonzoBuilderHandle* self);

    diplomat::capi::DiplomatU8View HonzoBuilderHandle_get_result(const diplomat::capi::HonzoBuilderHandle* self);

    void HonzoBuilderHandle_destroy(HonzoBuilderHandle* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<HonzoBuilderHandle> HonzoBuilderHandle::new_() {
    auto result = diplomat::capi::HonzoBuilderHandle_new();
    return std::unique_ptr<HonzoBuilderHandle>(HonzoBuilderHandle::FromFFI(result));
}

inline bool HonzoBuilderHandle::add_chunk(diplomat::span<const uint8_t> tag, diplomat::span<const uint8_t> data, uint8_t compression, uint8_t content_type_kind, uint8_t content_type_value) {
    auto result = diplomat::capi::HonzoBuilderHandle_add_chunk(this->AsFFI(),
        {tag.data(), tag.size()},
        {data.data(), data.size()},
        compression,
        content_type_kind,
        content_type_value);
    return result;
}

inline diplomat::result<bool, diplomat::Utf8Error> HonzoBuilderHandle::set_language(std::string_view lang) {
    if (!diplomat::capi::diplomat_is_str(lang.data(), lang.size())) {
    return diplomat::Err<diplomat::Utf8Error>();
  }
    auto result = diplomat::capi::HonzoBuilderHandle_set_language(this->AsFFI(),
        {lang.data(), lang.size()});
    return diplomat::Ok<bool>(result);
}

inline bool HonzoBuilderHandle::set_auto_sidx(bool enable) {
    auto result = diplomat::capi::HonzoBuilderHandle_set_auto_sidx(this->AsFFI(),
        enable);
    return result;
}

inline bool HonzoBuilderHandle::add_math_chunk(diplomat::span<const uint8_t> data, uint8_t math_type, uint8_t compression) {
    auto result = diplomat::capi::HonzoBuilderHandle_add_math_chunk(this->AsFFI(),
        {data.data(), data.size()},
        math_type,
        compression);
    return result;
}

inline bool HonzoBuilderHandle::set_meta(diplomat::span<const uint8_t> msgpack) {
    auto result = diplomat::capi::HonzoBuilderHandle_set_meta(this->AsFFI(),
        {msgpack.data(), msgpack.size()});
    return result;
}

inline bool HonzoBuilderHandle::finalize() {
    auto result = diplomat::capi::HonzoBuilderHandle_finalize(this->AsFFI());
    return result;
}

inline diplomat::span<const uint8_t> HonzoBuilderHandle::get_result() const {
    auto result = diplomat::capi::HonzoBuilderHandle_get_result(this->AsFFI());
    return diplomat::span<const uint8_t>(result.data, result.len);
}

inline const diplomat::capi::HonzoBuilderHandle* HonzoBuilderHandle::AsFFI() const {
    return reinterpret_cast<const diplomat::capi::HonzoBuilderHandle*>(this);
}

inline diplomat::capi::HonzoBuilderHandle* HonzoBuilderHandle::AsFFI() {
    return reinterpret_cast<diplomat::capi::HonzoBuilderHandle*>(this);
}

inline const HonzoBuilderHandle* HonzoBuilderHandle::FromFFI(const diplomat::capi::HonzoBuilderHandle* ptr) {
    return reinterpret_cast<const HonzoBuilderHandle*>(ptr);
}

inline HonzoBuilderHandle* HonzoBuilderHandle::FromFFI(diplomat::capi::HonzoBuilderHandle* ptr) {
    return reinterpret_cast<HonzoBuilderHandle*>(ptr);
}

inline void HonzoBuilderHandle::operator delete(void* ptr) {
    diplomat::capi::HonzoBuilderHandle_destroy(reinterpret_cast<diplomat::capi::HonzoBuilderHandle*>(ptr));
}


#endif // HonzoBuilderHandle_HPP
