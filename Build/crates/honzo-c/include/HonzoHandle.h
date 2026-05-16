#ifndef HonzoHandle_H
#define HonzoHandle_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "HonzoHandle.d.h"






HonzoHandle* HonzoHandle_parse(DiplomatU8View data, uint16_t _reader_version);

uint32_t HonzoHandle_chunk_count(const HonzoHandle* self);

uint8_t HonzoHandle_layout_mode(const HonzoHandle* self);

bool HonzoHandle_has_drm(const HonzoHandle* self);

bool HonzoHandle_has_sidx(const HonzoHandle* self);

typedef struct HonzoHandle_get_chunk_result {union {DiplomatU8View ok; }; bool is_ok;} HonzoHandle_get_chunk_result;
HonzoHandle_get_chunk_result HonzoHandle_get_chunk(const HonzoHandle* self, uint32_t index);

DiplomatU8View HonzoHandle_get_meta(const HonzoHandle* self);

void HonzoHandle_destroy(HonzoHandle* self);





#endif // HonzoHandle_H
