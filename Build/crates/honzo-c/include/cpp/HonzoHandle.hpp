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
#include "HonzoErrorCode.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {

    diplomat::capi::HonzoHandle* HonzoHandle_parse(diplomat::capi::DiplomatU8View data, uint16_t reader_version);

    uint32_t HonzoHandle_chunk_count(const diplomat::capi::HonzoHandle* self);

    uint8_t HonzoHandle_version_major(const diplomat::capi::HonzoHandle* self);

    uint8_t HonzoHandle_version_minor(const diplomat::capi::HonzoHandle* self);

    uint16_t HonzoHandle_min_reader_version(const diplomat::capi::HonzoHandle* self);

    uint32_t HonzoHandle_flags(const diplomat::capi::HonzoHandle* self);

    uint64_t HonzoHandle_toc_size(const diplomat::capi::HonzoHandle* self);

    uint64_t HonzoHandle_data_size(const diplomat::capi::HonzoHandle* self);

    uint64_t HonzoHandle_extra_size(const diplomat::capi::HonzoHandle* self);

    uint64_t HonzoHandle_meta_size(const diplomat::capi::HonzoHandle* self);

    uint8_t HonzoHandle_layout_mode(const diplomat::capi::HonzoHandle* self);

    bool HonzoHandle_has_drm(const diplomat::capi::HonzoHandle* self);

    bool HonzoHandle_has_sidx(const diplomat::capi::HonzoHandle* self);

    bool HonzoHandle_has_annotations(const diplomat::capi::HonzoHandle* self);

    bool HonzoHandle_has_sync(const diplomat::capi::HonzoHandle* self);

    diplomat::capi::DiplomatU8View HonzoHandle_get_extra(const diplomat::capi::HonzoHandle* self);

    typedef struct HonzoHandle_get_chunk_result {union {diplomat::capi::DiplomatU8View ok; }; bool is_ok;} HonzoHandle_get_chunk_result;
    HonzoHandle_get_chunk_result HonzoHandle_get_chunk(diplomat::capi::HonzoHandle* self, uint32_t index);

    diplomat::capi::DiplomatU8View HonzoHandle_get_meta(const diplomat::capi::HonzoHandle* self);

    typedef struct HonzoHandle_get_meta_parsed_result {union { diplomat::capi::HonzoErrorCode err;}; bool is_ok;} HonzoHandle_get_meta_parsed_result;
    HonzoHandle_get_meta_parsed_result HonzoHandle_get_meta_parsed(const diplomat::capi::HonzoHandle* self, diplomat::capi::DiplomatWrite* write);

    typedef struct HonzoHandle_get_annotations_result {union { diplomat::capi::HonzoErrorCode err;}; bool is_ok;} HonzoHandle_get_annotations_result;
    HonzoHandle_get_annotations_result HonzoHandle_get_annotations(const diplomat::capi::HonzoHandle* self, diplomat::capi::DiplomatWrite* write);

    typedef struct HonzoHandle_get_sync_cues_result {union { diplomat::capi::HonzoErrorCode err;}; bool is_ok;} HonzoHandle_get_sync_cues_result;
    HonzoHandle_get_sync_cues_result HonzoHandle_get_sync_cues(const diplomat::capi::HonzoHandle* self, diplomat::capi::DiplomatWrite* write);

    typedef struct HonzoHandle_get_toc_result {union { diplomat::capi::HonzoErrorCode err;}; bool is_ok;} HonzoHandle_get_toc_result;
    HonzoHandle_get_toc_result HonzoHandle_get_toc(const diplomat::capi::HonzoHandle* self, diplomat::capi::DiplomatWrite* write);

    void HonzoHandle_destroy(HonzoHandle* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<HonzoHandle> HonzoHandle::parse(diplomat::span<const uint8_t> data, uint16_t reader_version) {
    auto result = diplomat::capi::HonzoHandle_parse({data.data(), data.size()},
        reader_version);
    return std::unique_ptr<HonzoHandle>(HonzoHandle::FromFFI(result));
}

inline uint32_t HonzoHandle::chunk_count() const {
    auto result = diplomat::capi::HonzoHandle_chunk_count(this->AsFFI());
    return result;
}

inline uint8_t HonzoHandle::version_major() const {
    auto result = diplomat::capi::HonzoHandle_version_major(this->AsFFI());
    return result;
}

inline uint8_t HonzoHandle::version_minor() const {
    auto result = diplomat::capi::HonzoHandle_version_minor(this->AsFFI());
    return result;
}

inline uint16_t HonzoHandle::min_reader_version() const {
    auto result = diplomat::capi::HonzoHandle_min_reader_version(this->AsFFI());
    return result;
}

inline uint32_t HonzoHandle::flags() const {
    auto result = diplomat::capi::HonzoHandle_flags(this->AsFFI());
    return result;
}

inline uint64_t HonzoHandle::toc_size() const {
    auto result = diplomat::capi::HonzoHandle_toc_size(this->AsFFI());
    return result;
}

inline uint64_t HonzoHandle::data_size() const {
    auto result = diplomat::capi::HonzoHandle_data_size(this->AsFFI());
    return result;
}

inline uint64_t HonzoHandle::extra_size() const {
    auto result = diplomat::capi::HonzoHandle_extra_size(this->AsFFI());
    return result;
}

inline uint64_t HonzoHandle::meta_size() const {
    auto result = diplomat::capi::HonzoHandle_meta_size(this->AsFFI());
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

inline bool HonzoHandle::has_annotations() const {
    auto result = diplomat::capi::HonzoHandle_has_annotations(this->AsFFI());
    return result;
}

inline bool HonzoHandle::has_sync() const {
    auto result = diplomat::capi::HonzoHandle_has_sync(this->AsFFI());
    return result;
}

inline diplomat::span<const uint8_t> HonzoHandle::get_extra() const {
    auto result = diplomat::capi::HonzoHandle_get_extra(this->AsFFI());
    return diplomat::span<const uint8_t>(result.data, result.len);
}

inline std::optional<diplomat::span<const uint8_t>> HonzoHandle::get_chunk(uint32_t index) {
    auto result = diplomat::capi::HonzoHandle_get_chunk(this->AsFFI(),
        index);
    return result.is_ok ? std::optional<diplomat::span<const uint8_t>>(diplomat::span<const uint8_t>(result.ok.data, result.ok.len)) : std::nullopt;
}

inline diplomat::span<const uint8_t> HonzoHandle::get_meta() const {
    auto result = diplomat::capi::HonzoHandle_get_meta(this->AsFFI());
    return diplomat::span<const uint8_t>(result.data, result.len);
}

inline diplomat::result<std::string, HonzoErrorCode> HonzoHandle::get_meta_parsed() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = diplomat::capi::HonzoHandle_get_meta_parsed(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, HonzoErrorCode>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, HonzoErrorCode>(diplomat::Err<HonzoErrorCode>(HonzoErrorCode::FromFFI(result.err)));
}
template<typename W>
inline diplomat::result<std::monostate, HonzoErrorCode> HonzoHandle::get_meta_parsed_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = diplomat::capi::HonzoHandle_get_meta_parsed(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, HonzoErrorCode>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, HonzoErrorCode>(diplomat::Err<HonzoErrorCode>(HonzoErrorCode::FromFFI(result.err)));
}

inline diplomat::result<std::string, HonzoErrorCode> HonzoHandle::get_annotations() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = diplomat::capi::HonzoHandle_get_annotations(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, HonzoErrorCode>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, HonzoErrorCode>(diplomat::Err<HonzoErrorCode>(HonzoErrorCode::FromFFI(result.err)));
}
template<typename W>
inline diplomat::result<std::monostate, HonzoErrorCode> HonzoHandle::get_annotations_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = diplomat::capi::HonzoHandle_get_annotations(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, HonzoErrorCode>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, HonzoErrorCode>(diplomat::Err<HonzoErrorCode>(HonzoErrorCode::FromFFI(result.err)));
}

inline diplomat::result<std::string, HonzoErrorCode> HonzoHandle::get_sync_cues() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = diplomat::capi::HonzoHandle_get_sync_cues(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, HonzoErrorCode>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, HonzoErrorCode>(diplomat::Err<HonzoErrorCode>(HonzoErrorCode::FromFFI(result.err)));
}
template<typename W>
inline diplomat::result<std::monostate, HonzoErrorCode> HonzoHandle::get_sync_cues_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = diplomat::capi::HonzoHandle_get_sync_cues(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, HonzoErrorCode>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, HonzoErrorCode>(diplomat::Err<HonzoErrorCode>(HonzoErrorCode::FromFFI(result.err)));
}

inline diplomat::result<std::string, HonzoErrorCode> HonzoHandle::get_toc() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = diplomat::capi::HonzoHandle_get_toc(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, HonzoErrorCode>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, HonzoErrorCode>(diplomat::Err<HonzoErrorCode>(HonzoErrorCode::FromFFI(result.err)));
}
template<typename W>
inline diplomat::result<std::monostate, HonzoErrorCode> HonzoHandle::get_toc_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = diplomat::capi::HonzoHandle_get_toc(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, HonzoErrorCode>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, HonzoErrorCode>(diplomat::Err<HonzoErrorCode>(HonzoErrorCode::FromFFI(result.err)));
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
