#ifndef HonzoFileReader_D_HPP
#define HonzoFileReader_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"

class HonzoErrorCode;




namespace diplomat {
namespace capi {
    struct HonzoFileReader;
} // namespace capi
} // namespace

class HonzoFileReader {
public:

  inline static diplomat::result<diplomat::result<std::unique_ptr<HonzoFileReader>, HonzoErrorCode>, diplomat::Utf8Error> open(std::string_view path, uint16_t reader_version);

  inline static diplomat::result<diplomat::result<std::unique_ptr<HonzoFileReader>, HonzoErrorCode>, diplomat::Utf8Error> open_with_private_key(std::string_view path, uint16_t reader_version, diplomat::span<const uint8_t> private_key);

  inline uint32_t chunk_count() const;

  inline std::optional<diplomat::span<const uint8_t>> get_chunk(uint32_t index);

  inline diplomat::result<std::string, HonzoErrorCode> get_meta();
  template<typename W>
  inline diplomat::result<std::monostate, HonzoErrorCode> get_meta_write(W& writeable_output);

    inline const diplomat::capi::HonzoFileReader* AsFFI() const;
    inline diplomat::capi::HonzoFileReader* AsFFI();
    inline static const HonzoFileReader* FromFFI(const diplomat::capi::HonzoFileReader* ptr);
    inline static HonzoFileReader* FromFFI(diplomat::capi::HonzoFileReader* ptr);
    inline static void operator delete(void* ptr);
private:
    HonzoFileReader() = delete;
    HonzoFileReader(const HonzoFileReader&) = delete;
    HonzoFileReader(HonzoFileReader&&) noexcept = delete;
    HonzoFileReader operator=(const HonzoFileReader&) = delete;
    HonzoFileReader operator=(HonzoFileReader&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};


#endif // HonzoFileReader_D_HPP
