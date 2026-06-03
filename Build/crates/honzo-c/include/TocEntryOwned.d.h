#ifndef TocEntryOwned_D_H
#define TocEntryOwned_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct TocEntryOwned {
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
} TocEntryOwned;

typedef struct TocEntryOwned_option {union { TocEntryOwned ok; }; bool is_ok; } TocEntryOwned_option;



#endif // TocEntryOwned_D_H
