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

    bool HonzoBuilderHandle_add_chunk(diplomat::capi::HonzoBuilderHandle* self, diplomat::capi::DiplomatU8View tag, diplomat::capi::DiplomatU8View data, uint8_t compression, uint8_t content_type_kind, uint8_t content_type_value, uint8_t cover_type, diplomat::capi::DiplomatStringView alt_text, int32_t font_embedding, diplomat::capi::DiplomatStringView font_license_url);

    bool HonzoBuilderHandle_set_language(diplomat::capi::HonzoBuilderHandle* self, diplomat::capi::DiplomatStringView lang);

    bool HonzoBuilderHandle_set_auto_sidx(diplomat::capi::HonzoBuilderHandle* self, bool enable);

    bool HonzoBuilderHandle_set_auto_covt(diplomat::capi::HonzoBuilderHandle* self, bool enable);

    bool HonzoBuilderHandle_set_layout(diplomat::capi::HonzoBuilderHandle* self, uint8_t layout);

    bool HonzoBuilderHandle_set_flags(diplomat::capi::HonzoBuilderHandle* self, uint32_t flags);

    bool HonzoBuilderHandle_set_min_reader_version(diplomat::capi::HonzoBuilderHandle* self, uint16_t version);

    bool HonzoBuilderHandle_add_pmap_entry(diplomat::capi::HonzoBuilderHandle* self, uint32_t print_page, uint32_t chunk_id, uint32_t byte_offset);

    bool HonzoBuilderHandle_add_math_chunk(diplomat::capi::HonzoBuilderHandle* self, diplomat::capi::DiplomatU8View data, uint8_t math_type, uint8_t compression);

    bool HonzoBuilderHandle_set_meta(diplomat::capi::HonzoBuilderHandle* self, diplomat::capi::DiplomatU8View msgpack);

    bool HonzoBuilderHandle_set_extra(diplomat::capi::HonzoBuilderHandle* self, diplomat::capi::DiplomatU8View extra);

    bool HonzoBuilderHandle_add_extra_entry(diplomat::capi::HonzoBuilderHandle* self, diplomat::capi::DiplomatU8View tag, diplomat::capi::DiplomatStringView namespace_, diplomat::capi::DiplomatU8View body);

    bool HonzoBuilderHandle_add_annotation(diplomat::capi::HonzoBuilderHandle* self, diplomat::capi::DiplomatU8View body);

    bool HonzoBuilderHandle_add_sync_cue(diplomat::capi::HonzoBuilderHandle* self, diplomat::capi::DiplomatU8View body);

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

inline diplomat::result<bool, diplomat::Utf8Error> HonzoBuilderHandle::add_chunk(diplomat::span<const uint8_t> tag, diplomat::span<const uint8_t> data, uint8_t compression, uint8_t content_type_kind, uint8_t content_type_value, uint8_t cover_type, std::string_view alt_text, int32_t font_embedding, std::string_view font_license_url) {
    if (!diplomat::capi::diplomat_is_str(alt_text.data(), alt_text.size())) {
    return diplomat::Err<diplomat::Utf8Error>();
  }
    if (!diplomat::capi::diplomat_is_str(font_license_url.data(), font_license_url.size())) {
    return diplomat::Err<diplomat::Utf8Error>();
  }
    auto result = diplomat::capi::HonzoBuilderHandle_add_chunk(this->AsFFI(),
        {tag.data(), tag.size()},
        {data.data(), data.size()},
        compression,
        content_type_kind,
        content_type_value,
        cover_type,
        {alt_text.data(), alt_text.size()},
        font_embedding,
        {font_license_url.data(), font_license_url.size()});
    return diplomat::Ok<bool>(result);
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

inline bool HonzoBuilderHandle::set_auto_covt(bool enable) {
    auto result = diplomat::capi::HonzoBuilderHandle_set_auto_covt(this->AsFFI(),
        enable);
    return result;
}

inline bool HonzoBuilderHandle::set_layout(uint8_t layout) {
    auto result = diplomat::capi::HonzoBuilderHandle_set_layout(this->AsFFI(),
        layout);
    return result;
}

inline bool HonzoBuilderHandle::set_flags(uint32_t flags) {
    auto result = diplomat::capi::HonzoBuilderHandle_set_flags(this->AsFFI(),
        flags);
    return result;
}

inline bool HonzoBuilderHandle::set_min_reader_version(uint16_t version) {
    auto result = diplomat::capi::HonzoBuilderHandle_set_min_reader_version(this->AsFFI(),
        version);
    return result;
}

inline bool HonzoBuilderHandle::add_pmap_entry(uint32_t print_page, uint32_t chunk_id, uint32_t byte_offset) {
    auto result = diplomat::capi::HonzoBuilderHandle_add_pmap_entry(this->AsFFI(),
        print_page,
        chunk_id,
        byte_offset);
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

inline bool HonzoBuilderHandle::set_extra(diplomat::span<const uint8_t> extra) {
    auto result = diplomat::capi::HonzoBuilderHandle_set_extra(this->AsFFI(),
        {extra.data(), extra.size()});
    return result;
}

inline diplomat::result<bool, diplomat::Utf8Error> HonzoBuilderHandle::add_extra_entry(diplomat::span<const uint8_t> tag, std::string_view namespace_, diplomat::span<const uint8_t> body) {
    if (!diplomat::capi::diplomat_is_str(namespace_.data(), namespace_.size())) {
    return diplomat::Err<diplomat::Utf8Error>();
  }
    auto result = diplomat::capi::HonzoBuilderHandle_add_extra_entry(this->AsFFI(),
        {tag.data(), tag.size()},
        {namespace_.data(), namespace_.size()},
        {body.data(), body.size()});
    return diplomat::Ok<bool>(result);
}

inline bool HonzoBuilderHandle::add_annotation(diplomat::span<const uint8_t> body) {
    auto result = diplomat::capi::HonzoBuilderHandle_add_annotation(this->AsFFI(),
        {body.data(), body.size()});
    return result;
}

inline bool HonzoBuilderHandle::add_sync_cue(diplomat::span<const uint8_t> body) {
    auto result = diplomat::capi::HonzoBuilderHandle_add_sync_cue(this->AsFFI(),
        {body.data(), body.size()});
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
