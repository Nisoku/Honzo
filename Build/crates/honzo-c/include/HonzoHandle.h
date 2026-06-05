#ifndef HonzoHandle_H
#define HonzoHandle_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "HonzoErrorCode.d.h"

#include "HonzoHandle.d.h"






HonzoHandle* HonzoHandle_parse(DiplomatU8View data, uint16_t reader_version);

HonzoHandle* HonzoHandle_parse_with_private_key(DiplomatU8View data, uint16_t reader_version, DiplomatU8View private_key_der);

uint32_t HonzoHandle_chunk_count(const HonzoHandle* self);

uint8_t HonzoHandle_version_major(const HonzoHandle* self);

uint8_t HonzoHandle_version_minor(const HonzoHandle* self);

uint16_t HonzoHandle_min_reader_version(const HonzoHandle* self);

uint32_t HonzoHandle_flags(const HonzoHandle* self);

uint64_t HonzoHandle_toc_size(const HonzoHandle* self);

uint64_t HonzoHandle_data_size(const HonzoHandle* self);

uint64_t HonzoHandle_extra_size(const HonzoHandle* self);

uint64_t HonzoHandle_meta_size(const HonzoHandle* self);

uint8_t HonzoHandle_layout_mode(const HonzoHandle* self);

bool HonzoHandle_has_drm(const HonzoHandle* self);

bool HonzoHandle_has_sidx(const HonzoHandle* self);

bool HonzoHandle_has_annotations(const HonzoHandle* self);

bool HonzoHandle_has_sync(const HonzoHandle* self);

DiplomatU8View HonzoHandle_get_extra(const HonzoHandle* self);

typedef struct HonzoHandle_get_chunk_result {union {DiplomatU8View ok; }; bool is_ok;} HonzoHandle_get_chunk_result;
HonzoHandle_get_chunk_result HonzoHandle_get_chunk(HonzoHandle* self, uint32_t index);

DiplomatU8View HonzoHandle_get_meta(const HonzoHandle* self);

typedef struct HonzoHandle_get_meta_parsed_result {union { HonzoErrorCode err;}; bool is_ok;} HonzoHandle_get_meta_parsed_result;
HonzoHandle_get_meta_parsed_result HonzoHandle_get_meta_parsed(const HonzoHandle* self, DiplomatWrite* write);

typedef struct HonzoHandle_get_annotations_result {union { HonzoErrorCode err;}; bool is_ok;} HonzoHandle_get_annotations_result;
HonzoHandle_get_annotations_result HonzoHandle_get_annotations(const HonzoHandle* self, DiplomatWrite* write);

typedef struct HonzoHandle_get_sync_cues_result {union { HonzoErrorCode err;}; bool is_ok;} HonzoHandle_get_sync_cues_result;
HonzoHandle_get_sync_cues_result HonzoHandle_get_sync_cues(const HonzoHandle* self, DiplomatWrite* write);

typedef struct HonzoHandle_get_toc_result {union { HonzoErrorCode err;}; bool is_ok;} HonzoHandle_get_toc_result;
HonzoHandle_get_toc_result HonzoHandle_get_toc(const HonzoHandle* self, DiplomatWrite* write);

void HonzoHandle_destroy(HonzoHandle* self);





#endif // HonzoHandle_H
