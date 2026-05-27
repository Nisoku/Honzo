#ifndef HonzoBuilderHandle_H
#define HonzoBuilderHandle_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "HonzoBuilderHandle.d.h"






HonzoBuilderHandle* HonzoBuilderHandle_new(void);

bool HonzoBuilderHandle_add_chunk(HonzoBuilderHandle* self, DiplomatU8View tag, DiplomatU8View data, uint8_t compression, uint8_t content_type_kind, uint8_t content_type_value);

bool HonzoBuilderHandle_set_language(HonzoBuilderHandle* self, DiplomatStringView lang);

bool HonzoBuilderHandle_set_auto_sidx(HonzoBuilderHandle* self, bool enable);

bool HonzoBuilderHandle_add_math_chunk(HonzoBuilderHandle* self, DiplomatU8View data, uint8_t math_type, uint8_t compression);

bool HonzoBuilderHandle_set_meta(HonzoBuilderHandle* self, DiplomatU8View msgpack);

bool HonzoBuilderHandle_add_extra_entry(HonzoBuilderHandle* self, DiplomatU8View tag, DiplomatStringView namespace, DiplomatU8View body);

bool HonzoBuilderHandle_add_annotation(HonzoBuilderHandle* self, DiplomatU8View body);

bool HonzoBuilderHandle_add_sync_cue(HonzoBuilderHandle* self, DiplomatU8View body);

bool HonzoBuilderHandle_finalize(HonzoBuilderHandle* self);

DiplomatU8View HonzoBuilderHandle_get_result(const HonzoBuilderHandle* self);

void HonzoBuilderHandle_destroy(HonzoBuilderHandle* self);





#endif // HonzoBuilderHandle_H
