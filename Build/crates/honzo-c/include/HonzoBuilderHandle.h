#ifndef HonzoBuilderHandle_H
#define HonzoBuilderHandle_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "HonzoBuilderHandle.d.h"






HonzoBuilderHandle* HonzoBuilderHandle_new(void);

bool HonzoBuilderHandle_add_chunk(HonzoBuilderHandle* self, DiplomatU8View tag, DiplomatU8View data, uint8_t compression, uint8_t content_type_kind, uint8_t content_type_value, uint8_t cover_type, DiplomatStringView alt_text, int32_t font_embedding, DiplomatStringView font_license_url);

bool HonzoBuilderHandle_set_language(HonzoBuilderHandle* self, DiplomatStringView lang);

bool HonzoBuilderHandle_set_auto_sidx(HonzoBuilderHandle* self, bool enable);

bool HonzoBuilderHandle_set_auto_covt(HonzoBuilderHandle* self, bool enable);

bool HonzoBuilderHandle_set_layout(HonzoBuilderHandle* self, uint8_t layout);

bool HonzoBuilderHandle_set_flags(HonzoBuilderHandle* self, uint32_t flags);

bool HonzoBuilderHandle_set_min_reader_version(HonzoBuilderHandle* self, uint16_t version);

bool HonzoBuilderHandle_add_pmap_entry(HonzoBuilderHandle* self, uint32_t print_page, uint32_t chunk_id, uint32_t byte_offset);

bool HonzoBuilderHandle_add_math_chunk(HonzoBuilderHandle* self, DiplomatU8View data, uint8_t math_type, uint8_t compression);

bool HonzoBuilderHandle_set_meta(HonzoBuilderHandle* self, DiplomatU8View msgpack);

bool HonzoBuilderHandle_set_extra(HonzoBuilderHandle* self, DiplomatU8View extra);

bool HonzoBuilderHandle_add_extra_entry(HonzoBuilderHandle* self, DiplomatU8View tag, DiplomatStringView namespace, DiplomatU8View body);

bool HonzoBuilderHandle_add_annotation(HonzoBuilderHandle* self, DiplomatU8View body);

bool HonzoBuilderHandle_set_drm_config(HonzoBuilderHandle* self, DiplomatU32View encrypt_chunk_ids, DiplomatU8View recipient_public_key, DiplomatStringView license_url, uint64_t expires_at);

bool HonzoBuilderHandle_add_sync_cue(HonzoBuilderHandle* self, DiplomatU8View body);

bool HonzoBuilderHandle_finalize(HonzoBuilderHandle* self);

DiplomatU8View HonzoBuilderHandle_get_result(const HonzoBuilderHandle* self);

void HonzoBuilderHandle_destroy(HonzoBuilderHandle* self);





#endif // HonzoBuilderHandle_H
