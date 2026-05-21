#ifndef HonzoErrorCode_D_H
#define HonzoErrorCode_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum HonzoErrorCode {
  HonzoErrorCode_Ok = 0,
  HonzoErrorCode_InvalidMagic = 1,
  HonzoErrorCode_ReaderVersionTooOld = 2,
  HonzoErrorCode_BufferTooShort = 3,
  HonzoErrorCode_CrcMismatch = 4,
  HonzoErrorCode_EncryptedChunk = 5,
  HonzoErrorCode_InvalidMathML = 6,
  HonzoErrorCode_Truncated = 7,
  HonzoErrorCode_Unknown = 255,
} HonzoErrorCode;

typedef struct HonzoErrorCode_option {union { HonzoErrorCode ok; }; bool is_ok; } HonzoErrorCode_option;



#endif // HonzoErrorCode_D_H
