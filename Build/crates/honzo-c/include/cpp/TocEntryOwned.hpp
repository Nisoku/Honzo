#ifndef TocEntryOwned_HPP
#define TocEntryOwned_HPP

#include "TocEntryOwned.d.hpp"

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

} // namespace capi
} // namespace


inline diplomat::capi::TocEntryOwned TocEntryOwned::AsFFI() const {
    return diplomat::capi::TocEntryOwned {
        /* .chunk_id = */ chunk_id,
        /* .offset = */ offset,
        /* .size_compressed = */ size_compressed,
        /* .size_raw = */ size_raw,
        /* .compression = */ compression,
        /* .ctype_kind = */ ctype_kind,
        /* .ctype_value = */ ctype_value,
        /* .cover_type = */ cover_type,
        /* .flags = */ flags,
        /* .crc32 = */ crc32,
    };
}

inline TocEntryOwned TocEntryOwned::FromFFI(diplomat::capi::TocEntryOwned c_struct) {
    return TocEntryOwned {
        /* .chunk_id = */ c_struct.chunk_id,
        /* .offset = */ c_struct.offset,
        /* .size_compressed = */ c_struct.size_compressed,
        /* .size_raw = */ c_struct.size_raw,
        /* .compression = */ c_struct.compression,
        /* .ctype_kind = */ c_struct.ctype_kind,
        /* .ctype_value = */ c_struct.ctype_value,
        /* .cover_type = */ c_struct.cover_type,
        /* .flags = */ c_struct.flags,
        /* .crc32 = */ c_struct.crc32,
    };
}


#endif // TocEntryOwned_HPP
