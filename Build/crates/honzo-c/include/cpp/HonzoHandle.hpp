#ifndef HonzoHandle_HPP
#define HonzoHandle_HPP

#include "HonzoHandle.d.hpp"

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

    diplomat::capi::HonzoHandle* HonzoHandle_parse(diplomat::capi::DiplomatU8View data, uint16_t _reader_version);

    uint32_t HonzoHandle_chunk_count(const diplomat::capi::HonzoHandle* self);

    uint8_t HonzoHandle_layout_mode(const diplomat::capi::HonzoHandle* self);

    bool HonzoHandle_has_drm(const diplomat::capi::HonzoHandle* self);

    bool HonzoHandle_has_sidx(const diplomat::capi::HonzoHandle* self);

    typedef struct HonzoHandle_get_chunk_result {union {diplomat::capi::DiplomatU8View ok; }; bool is_ok;} HonzoHandle_get_chunk_result;
    HonzoHandle_get_chunk_result HonzoHandle_get_chunk(const diplomat::capi::HonzoHandle* self, uint32_t index);

    diplomat::capi::DiplomatU8View HonzoHandle_get_meta(const diplomat::capi::HonzoHandle* self);

    void HonzoHandle_destroy(HonzoHandle* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<HonzoHandle> HonzoHandle::parse(diplomat::span<const uint8_t> data, uint16_t _reader_version) {
    auto result = diplomat::capi::HonzoHandle_parse({data.data(), data.size()},
        _reader_version);
    return std::unique_ptr<HonzoHandle>(HonzoHandle::FromFFI(result));
}

inline uint32_t HonzoHandle::chunk_count() const {
    auto result = diplomat::capi::HonzoHandle_chunk_count(this->AsFFI());
    return result;
}

inline uint8_t HonzoHandle::layout_mode() const {
    auto result = diplomat::capi::HonzoHandle_layout_mode(this->AsFFI());
    return result;
}

inline bool HonzoHandle::has_drm() const {
    auto result = diplomat::capi::HonzoHandle_has_drm(this->AsFFI());
    return result;
}

inline bool HonzoHandle::has_sidx() const {
    auto result = diplomat::capi::HonzoHandle_has_sidx(this->AsFFI());
    return result;
}

inline std::optional<diplomat::span<const uint8_t>> HonzoHandle::get_chunk(uint32_t index) const {
    auto result = diplomat::capi::HonzoHandle_get_chunk(this->AsFFI(),
        index);
    return result.is_ok ? std::optional<diplomat::span<const uint8_t>>(diplomat::span<const uint8_t>(result.ok.data, result.ok.len)) : std::nullopt;
}

inline diplomat::span<const uint8_t> HonzoHandle::get_meta() const {
    auto result = diplomat::capi::HonzoHandle_get_meta(this->AsFFI());
    return diplomat::span<const uint8_t>(result.data, result.len);
}

inline const diplomat::capi::HonzoHandle* HonzoHandle::AsFFI() const {
    return reinterpret_cast<const diplomat::capi::HonzoHandle*>(this);
}

inline diplomat::capi::HonzoHandle* HonzoHandle::AsFFI() {
    return reinterpret_cast<diplomat::capi::HonzoHandle*>(this);
}

inline const HonzoHandle* HonzoHandle::FromFFI(const diplomat::capi::HonzoHandle* ptr) {
    return reinterpret_cast<const HonzoHandle*>(ptr);
}

inline HonzoHandle* HonzoHandle::FromFFI(diplomat::capi::HonzoHandle* ptr) {
    return reinterpret_cast<HonzoHandle*>(ptr);
}

inline void HonzoHandle::operator delete(void* ptr) {
    diplomat::capi::HonzoHandle_destroy(reinterpret_cast<diplomat::capi::HonzoHandle*>(ptr));
}


#endif // HonzoHandle_HPP
