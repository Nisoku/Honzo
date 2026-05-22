#ifndef free_functions_HPP
#define free_functions_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "HonzoErrorCode.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {

    typedef struct diplomat_external_latex_to_mathml_result {union { diplomat::capi::HonzoErrorCode err;}; bool is_ok;} diplomat_external_latex_to_mathml_result;
    diplomat_external_latex_to_mathml_result diplomat_external_latex_to_mathml(diplomat::capi::DiplomatU8View bytes, diplomat::capi::DiplomatWrite* write);

    typedef struct diplomat_external_normalize_search_term_result {union { diplomat::capi::HonzoErrorCode err;}; bool is_ok;} diplomat_external_normalize_search_term_result;
    diplomat_external_normalize_search_term_result diplomat_external_normalize_search_term(diplomat::capi::DiplomatStringView term, diplomat::capi::DiplomatWrite* write);

    typedef struct diplomat_external_render_math_result {union { diplomat::capi::HonzoErrorCode err;}; bool is_ok;} diplomat_external_render_math_result;
    diplomat_external_render_math_result diplomat_external_render_math(diplomat::capi::DiplomatU8View bytes, uint8_t math_type, diplomat::capi::DiplomatWrite* write);

    bool diplomat_external_validate_mathml(diplomat::capi::DiplomatU8View bytes);

    } // extern "C"
} // namespace capi
} // namespace


inline diplomat::result<std::string, HonzoErrorCode> latex_to_mathml(diplomat::span<const uint8_t> bytes) {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = diplomat::capi::diplomat_external_latex_to_mathml({bytes.data(), bytes.size()},
        &write);
    return result.is_ok ? diplomat::result<std::string, HonzoErrorCode>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, HonzoErrorCode>(diplomat::Err<HonzoErrorCode>(HonzoErrorCode::FromFFI(result.err)));
}
template<typename W>
inline diplomat::result<std::monostate, HonzoErrorCode> latex_to_mathml_write(diplomat::span<const uint8_t> bytes, W& writeable) {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = diplomat::capi::diplomat_external_latex_to_mathml({bytes.data(), bytes.size()},
        &write);
    return result.is_ok ? diplomat::result<std::monostate, HonzoErrorCode>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, HonzoErrorCode>(diplomat::Err<HonzoErrorCode>(HonzoErrorCode::FromFFI(result.err)));
}
inline diplomat::result<diplomat::result<std::string, HonzoErrorCode>, diplomat::Utf8Error> normalize_search_term(std::string_view term) {
    if (!diplomat::capi::diplomat_is_str(term.data(), term.size())) {
    return diplomat::Err<diplomat::Utf8Error>();
  }
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = diplomat::capi::diplomat_external_normalize_search_term({term.data(), term.size()},
        &write);
    return diplomat::Ok<diplomat::result<std::string, HonzoErrorCode>>(result.is_ok ? diplomat::result<std::string, HonzoErrorCode>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, HonzoErrorCode>(diplomat::Err<HonzoErrorCode>(HonzoErrorCode::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<diplomat::result<std::monostate, HonzoErrorCode>, diplomat::Utf8Error> normalize_search_term_write(std::string_view term, W& writeable) {
    if (!diplomat::capi::diplomat_is_str(term.data(), term.size())) {
    return diplomat::Err<diplomat::Utf8Error>();
  }
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = diplomat::capi::diplomat_external_normalize_search_term({term.data(), term.size()},
        &write);
    return diplomat::Ok<diplomat::result<std::monostate, HonzoErrorCode>>(result.is_ok ? diplomat::result<std::monostate, HonzoErrorCode>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, HonzoErrorCode>(diplomat::Err<HonzoErrorCode>(HonzoErrorCode::FromFFI(result.err))));
}
inline diplomat::result<std::string, HonzoErrorCode> render_math(diplomat::span<const uint8_t> bytes, uint8_t math_type) {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = diplomat::capi::diplomat_external_render_math({bytes.data(), bytes.size()},
        math_type,
        &write);
    return result.is_ok ? diplomat::result<std::string, HonzoErrorCode>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, HonzoErrorCode>(diplomat::Err<HonzoErrorCode>(HonzoErrorCode::FromFFI(result.err)));
}
template<typename W>
inline diplomat::result<std::monostate, HonzoErrorCode> render_math_write(diplomat::span<const uint8_t> bytes, uint8_t math_type, W& writeable) {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = diplomat::capi::diplomat_external_render_math({bytes.data(), bytes.size()},
        math_type,
        &write);
    return result.is_ok ? diplomat::result<std::monostate, HonzoErrorCode>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, HonzoErrorCode>(diplomat::Err<HonzoErrorCode>(HonzoErrorCode::FromFFI(result.err)));
}
inline bool validate_mathml(diplomat::span<const uint8_t> bytes) {
    auto result = diplomat::capi::diplomat_external_validate_mathml({bytes.data(), bytes.size()});
    return result;
}


#endif // free_functions_HPP
