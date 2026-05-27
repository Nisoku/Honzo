#ifndef HonzoHandle_D_HPP
#define HonzoHandle_D_HPP

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
    struct HonzoHandle;
} // namespace capi
} // namespace

class HonzoHandle {
public:

  inline static std::unique_ptr<HonzoHandle> parse(diplomat::span<const uint8_t> data, uint16_t reader_version);

  inline uint32_t chunk_count() const;

  inline uint8_t layout_mode() const;

  inline bool has_drm() const;

  inline bool has_sidx() const;

  inline std::optional<diplomat::span<const uint8_t>> get_chunk(uint32_t index) const;

  inline diplomat::span<const uint8_t> get_meta() const;

  inline diplomat::result<std::string, HonzoErrorCode> get_meta_parsed() const;
  template<typename W>
  inline diplomat::result<std::monostate, HonzoErrorCode> get_meta_parsed_write(W& writeable_output) const;

  inline diplomat::result<std::string, HonzoErrorCode> get_annotations() const;
  template<typename W>
  inline diplomat::result<std::monostate, HonzoErrorCode> get_annotations_write(W& writeable_output) const;

  inline diplomat::result<std::string, HonzoErrorCode> get_sync_cues() const;
  template<typename W>
  inline diplomat::result<std::monostate, HonzoErrorCode> get_sync_cues_write(W& writeable_output) const;

  inline diplomat::result<std::string, HonzoErrorCode> get_toc() const;
  template<typename W>
  inline diplomat::result<std::monostate, HonzoErrorCode> get_toc_write(W& writeable_output) const;

    inline const diplomat::capi::HonzoHandle* AsFFI() const;
    inline diplomat::capi::HonzoHandle* AsFFI();
    inline static const HonzoHandle* FromFFI(const diplomat::capi::HonzoHandle* ptr);
    inline static HonzoHandle* FromFFI(diplomat::capi::HonzoHandle* ptr);
    inline static void operator delete(void* ptr);
private:
    HonzoHandle() = delete;
    HonzoHandle(const HonzoHandle&) = delete;
    HonzoHandle(HonzoHandle&&) noexcept = delete;
    HonzoHandle operator=(const HonzoHandle&) = delete;
    HonzoHandle operator=(HonzoHandle&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};


#endif // HonzoHandle_D_HPP
