#ifndef HonzoFileReader_H
#define HonzoFileReader_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "HonzoErrorCode.d.h"

#include "HonzoFileReader.d.h"






typedef struct HonzoFileReader_open_result {union {HonzoFileReader* ok; HonzoErrorCode err;}; bool is_ok;} HonzoFileReader_open_result;
HonzoFileReader_open_result HonzoFileReader_open(DiplomatStringView path, uint16_t reader_version);

typedef struct HonzoFileReader_open_with_private_key_result {union {HonzoFileReader* ok; HonzoErrorCode err;}; bool is_ok;} HonzoFileReader_open_with_private_key_result;
HonzoFileReader_open_with_private_key_result HonzoFileReader_open_with_private_key(DiplomatStringView path, uint16_t reader_version, DiplomatU8View private_key);

uint32_t HonzoFileReader_chunk_count(const HonzoFileReader* self);

typedef struct HonzoFileReader_get_chunk_result {union {DiplomatU8View ok; }; bool is_ok;} HonzoFileReader_get_chunk_result;
HonzoFileReader_get_chunk_result HonzoFileReader_get_chunk(HonzoFileReader* self, uint32_t index);

typedef struct HonzoFileReader_get_meta_result {union { HonzoErrorCode err;}; bool is_ok;} HonzoFileReader_get_meta_result;
HonzoFileReader_get_meta_result HonzoFileReader_get_meta(HonzoFileReader* self, DiplomatWrite* write);

void HonzoFileReader_destroy(HonzoFileReader* self);





#endif // HonzoFileReader_H
