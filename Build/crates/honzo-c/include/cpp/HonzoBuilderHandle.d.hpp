#ifndef HonzoBuilderHandle_D_HPP
#define HonzoBuilderHandle_D_HPP

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
    struct HonzoBuilderHandle;
} // namespace capi
} // namespace

class HonzoBuilderHandle {
public:

  inline static std::unique_ptr<HonzoBuilderHandle> new_();

  inline diplomat::result<bool, diplomat::Utf8Error> add_chunk(diplomat::span<const uint8_t> tag, diplomat::span<const uint8_t> data, uint8_t compression, uint8_t content_type_kind, uint8_t content_type_value, uint8_t cover_type, std::string_view alt_text, int32_t font_embedding, std::string_view font_license_url);

  inline diplomat::result<bool, diplomat::Utf8Error> set_language(std::string_view lang);

  inline bool set_auto_sidx(bool enable);

  inline bool set_auto_covt(bool enable);

  inline bool set_layout(uint8_t layout);

  inline bool set_flags(uint32_t flags);

  inline bool set_min_reader_version(uint16_t version);

  inline bool add_pmap_entry(uint32_t print_page, uint32_t chunk_id, uint32_t byte_offset);

  inline bool add_math_chunk(diplomat::span<const uint8_t> data, uint8_t math_type, uint8_t compression);

  inline bool set_meta(diplomat::span<const uint8_t> msgpack);

  inline bool set_extra(diplomat::span<const uint8_t> extra);

  inline diplomat::result<bool, diplomat::Utf8Error> add_extra_entry(diplomat::span<const uint8_t> tag, std::string_view namespace_, diplomat::span<const uint8_t> body);

  inline bool add_annotation(diplomat::span<const uint8_t> body);

  inline bool add_sync_cue(diplomat::span<const uint8_t> body);

  inline bool finalize();

  inline diplomat::span<const uint8_t> get_result() const;

    inline const diplomat::capi::HonzoBuilderHandle* AsFFI() const;
    inline diplomat::capi::HonzoBuilderHandle* AsFFI();
    inline static const HonzoBuilderHandle* FromFFI(const diplomat::capi::HonzoBuilderHandle* ptr);
    inline static HonzoBuilderHandle* FromFFI(diplomat::capi::HonzoBuilderHandle* ptr);
    inline static void operator delete(void* ptr);
private:
    HonzoBuilderHandle() = delete;
    HonzoBuilderHandle(const HonzoBuilderHandle&) = delete;
    HonzoBuilderHandle(HonzoBuilderHandle&&) noexcept = delete;
    HonzoBuilderHandle operator=(const HonzoBuilderHandle&) = delete;
    HonzoBuilderHandle operator=(HonzoBuilderHandle&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};


#endif // HonzoBuilderHandle_D_HPP
