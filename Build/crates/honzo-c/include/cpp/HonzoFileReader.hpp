#ifndef HonzoFileReader_HPP
#define HonzoFileReader_HPP

#include "HonzoFileReader.d.hpp"

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

    typedef struct HonzoFileReader_open_result {union {diplomat::capi::HonzoFileReader* ok; diplomat::capi::HonzoErrorCode err;}; bool is_ok;} HonzoFileReader_open_result;
    HonzoFileReader_open_result HonzoFileReader_open(diplomat::capi::DiplomatStringView path, uint16_t reader_version);

    typedef struct HonzoFileReader_open_with_private_key_result {union {diplomat::capi::HonzoFileReader* ok; diplomat::capi::HonzoErrorCode err;}; bool is_ok;} HonzoFileReader_open_with_private_key_result;
    HonzoFileReader_open_with_private_key_result HonzoFileReader_open_with_private_key(diplomat::capi::DiplomatStringView path, uint16_t reader_version, diplomat::capi::DiplomatU8View private_key);

    uint32_t HonzoFileReader_chunk_count(const diplomat::capi::HonzoFileReader* self);

    uint32_t HonzoFileReader_get_chunk_type(const diplomat::capi::HonzoFileReader* self, uint32_t index);

    uint8_t HonzoFileReader_get_chunk_content_type_kind(const diplomat::capi::HonzoFileReader* self, uint32_t index);

    uint8_t HonzoFileReader_get_chunk_content_type_value(const diplomat::capi::HonzoFileReader* self, uint32_t index);

    typedef struct HonzoFileReader_get_chunk_result {union {diplomat::capi::DiplomatU8View ok; }; bool is_ok;} HonzoFileReader_get_chunk_result;
    HonzoFileReader_get_chunk_result HonzoFileReader_get_chunk(diplomat::capi::HonzoFileReader* self, uint32_t index);

    typedef struct HonzoFileReader_get_meta_result {union { diplomat::capi::HonzoErrorCode err;}; bool is_ok;} HonzoFileReader_get_meta_result;
    HonzoFileReader_get_meta_result HonzoFileReader_get_meta(diplomat::capi::HonzoFileReader* self, diplomat::capi::DiplomatWrite* write);

    void HonzoFileReader_destroy(HonzoFileReader* self);

    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<diplomat::result<std::unique_ptr<HonzoFileReader>, HonzoErrorCode>, diplomat::Utf8Error> HonzoFileReader::open(std::string_view path, uint16_t reader_version) {
    if (!diplomat::capi::diplomat_is_str(path.data(), path.size())) {
    return diplomat::Err<diplomat::Utf8Error>();
  }
    auto result = diplomat::capi::HonzoFileReader_open({path.data(), path.size()},
        reader_version);
    return diplomat::Ok<diplomat::result<std::unique_ptr<HonzoFileReader>, HonzoErrorCode>>(result.is_ok ? diplomat::result<std::unique_ptr<HonzoFileReader>, HonzoErrorCode>(diplomat::Ok<std::unique_ptr<HonzoFileReader>>(std::unique_ptr<HonzoFileReader>(HonzoFileReader::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<HonzoFileReader>, HonzoErrorCode>(diplomat::Err<HonzoErrorCode>(HonzoErrorCode::FromFFI(result.err))));
}

inline diplomat::result<diplomat::result<std::unique_ptr<HonzoFileReader>, HonzoErrorCode>, diplomat::Utf8Error> HonzoFileReader::open_with_private_key(std::string_view path, uint16_t reader_version, diplomat::span<const uint8_t> private_key) {
    if (!diplomat::capi::diplomat_is_str(path.data(), path.size())) {
    return diplomat::Err<diplomat::Utf8Error>();
  }
    auto result = diplomat::capi::HonzoFileReader_open_with_private_key({path.data(), path.size()},
        reader_version,
        {private_key.data(), private_key.size()});
    return diplomat::Ok<diplomat::result<std::unique_ptr<HonzoFileReader>, HonzoErrorCode>>(result.is_ok ? diplomat::result<std::unique_ptr<HonzoFileReader>, HonzoErrorCode>(diplomat::Ok<std::unique_ptr<HonzoFileReader>>(std::unique_ptr<HonzoFileReader>(HonzoFileReader::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<HonzoFileReader>, HonzoErrorCode>(diplomat::Err<HonzoErrorCode>(HonzoErrorCode::FromFFI(result.err))));
}

inline uint32_t HonzoFileReader::chunk_count() const {
    auto result = diplomat::capi::HonzoFileReader_chunk_count(this->AsFFI());
    return result;
}

inline uint32_t HonzoFileReader::get_chunk_type(uint32_t index) const {
    auto result = diplomat::capi::HonzoFileReader_get_chunk_type(this->AsFFI(),
        index);
    return result;
}

inline uint8_t HonzoFileReader::get_chunk_content_type_kind(uint32_t index) const {
    auto result = diplomat::capi::HonzoFileReader_get_chunk_content_type_kind(this->AsFFI(),
        index);
    return result;
}

inline uint8_t HonzoFileReader::get_chunk_content_type_value(uint32_t index) const {
    auto result = diplomat::capi::HonzoFileReader_get_chunk_content_type_value(this->AsFFI(),
        index);
    return result;
}

inline std::optional<diplomat::span<const uint8_t>> HonzoFileReader::get_chunk(uint32_t index) {
    auto result = diplomat::capi::HonzoFileReader_get_chunk(this->AsFFI(),
        index);
    return result.is_ok ? std::optional<diplomat::span<const uint8_t>>(diplomat::span<const uint8_t>(result.ok.data, result.ok.len)) : std::nullopt;
}

inline diplomat::result<std::string, HonzoErrorCode> HonzoFileReader::get_meta() {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = diplomat::capi::HonzoFileReader_get_meta(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, HonzoErrorCode>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, HonzoErrorCode>(diplomat::Err<HonzoErrorCode>(HonzoErrorCode::FromFFI(result.err)));
}
template<typename W>
inline diplomat::result<std::monostate, HonzoErrorCode> HonzoFileReader::get_meta_write(W& writeable) {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = diplomat::capi::HonzoFileReader_get_meta(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, HonzoErrorCode>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, HonzoErrorCode>(diplomat::Err<HonzoErrorCode>(HonzoErrorCode::FromFFI(result.err)));
}

inline const diplomat::capi::HonzoFileReader* HonzoFileReader::AsFFI() const {
    return reinterpret_cast<const diplomat::capi::HonzoFileReader*>(this);
}

inline diplomat::capi::HonzoFileReader* HonzoFileReader::AsFFI() {
    return reinterpret_cast<diplomat::capi::HonzoFileReader*>(this);
}

inline const HonzoFileReader* HonzoFileReader::FromFFI(const diplomat::capi::HonzoFileReader* ptr) {
    return reinterpret_cast<const HonzoFileReader*>(ptr);
}

inline HonzoFileReader* HonzoFileReader::FromFFI(diplomat::capi::HonzoFileReader* ptr) {
    return reinterpret_cast<HonzoFileReader*>(ptr);
}

inline void HonzoFileReader::operator delete(void* ptr) {
    diplomat::capi::HonzoFileReader_destroy(reinterpret_cast<diplomat::capi::HonzoFileReader*>(ptr));
}


#endif // HonzoFileReader_HPP
