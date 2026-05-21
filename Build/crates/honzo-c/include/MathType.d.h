#ifndef MathType_D_H
#define MathType_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum MathType {
  MathType_MathML = 0,
  MathType_LaTeX = 1,
} MathType;

typedef struct MathType_option {union { MathType ok; }; bool is_ok; } MathType_option;



#endif // MathType_D_H
