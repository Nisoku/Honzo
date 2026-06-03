#ifndef TocEntryOwned_D_HPP
#define TocEntryOwned_D_HPP

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
    struct TocEntryOwned {
      uint32_t chunk_id;
      uint64_t offset;
      uint32_t size_compressed;
      uint32_t size_raw;
      uint8_t compression;
      uint8_t ctype_kind;
      uint8_t ctype_value;
      uint8_t cover_type;
      uint8_t flags;
      uint32_t crc32;
    };

    typedef struct TocEntryOwned_option {union { TocEntryOwned ok; }; bool is_ok; } TocEntryOwned_option;
} // namespace capi
} // namespace


struct TocEntryOwned {
    uint32_t chunk_id;
    uint64_t offset;
    uint32_t size_compressed;
    uint32_t size_raw;
    uint8_t compression;
    uint8_t ctype_kind;
    uint8_t ctype_value;
    uint8_t cover_type;
    uint8_t flags;
    uint32_t crc32;

    inline diplomat::capi::TocEntryOwned AsFFI() const;
    inline static TocEntryOwned FromFFI(diplomat::capi::TocEntryOwned c_struct);
};


#endif // TocEntryOwned_D_HPP
