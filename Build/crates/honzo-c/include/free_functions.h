#ifndef free_functions_H
#define free_functions_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "HonzoErrorCode.d.h"






typedef struct diplomat_external_latex_to_mathml_result {union { HonzoErrorCode err;}; bool is_ok;} diplomat_external_latex_to_mathml_result;
diplomat_external_latex_to_mathml_result diplomat_external_latex_to_mathml(DiplomatU8View bytes, DiplomatWrite* write);

typedef struct diplomat_external_render_math_result {union { HonzoErrorCode err;}; bool is_ok;} diplomat_external_render_math_result;
diplomat_external_render_math_result diplomat_external_render_math(DiplomatU8View bytes, uint8_t math_type, DiplomatWrite* write);

bool diplomat_external_validate_mathml(DiplomatU8View bytes);





#endif // free_functions_H
