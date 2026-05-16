#ifndef HonzoBuilderHandle_H
#define HonzoBuilderHandle_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "HonzoBuilderHandle.d.h"






HonzoBuilderHandle* HonzoBuilderHandle_new(void);

bool HonzoBuilderHandle_add_chunk(HonzoBuilderHandle* self, DiplomatU8View tag, DiplomatU8View data, uint8_t compression, uint8_t markup_type);

bool HonzoBuilderHandle_set_meta(HonzoBuilderHandle* self, DiplomatU8View msgpack);

bool HonzoBuilderHandle_finalize(HonzoBuilderHandle* self);

DiplomatU8View HonzoBuilderHandle_get_result(const HonzoBuilderHandle* self);

void HonzoBuilderHandle_destroy(HonzoBuilderHandle* self);





#endif // HonzoBuilderHandle_H
