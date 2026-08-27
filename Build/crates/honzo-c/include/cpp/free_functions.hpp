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

    typedef struct diplomat_external_guess_font_format_result {union { diplomat::capi::HonzoErrorCode err;}; bool is_ok;} diplomat_external_guess_font_format_result;
    diplomat_external_guess_font_format_result diplomat_external_guess_font_format(diplomat::capi::DiplomatU8View bytes, diplomat::capi::DiplomatWrite* write);

    typedef struct diplomat_external_guess_image_mime_result {union { diplomat::capi::HonzoErrorCode err;}; bool is_ok;} diplomat_external_guess_image_mime_result;
    diplomat_external_guess_image_mime_result diplomat_external_guess_image_mime(diplomat::capi::DiplomatU8View bytes, diplomat::capi::DiplomatWrite* write);

    typedef struct diplomat_external_hzo_extract_meta_from_file_result {union { diplomat::capi::HonzoErrorCode err;}; bool is_ok;} diplomat_external_hzo_extract_meta_from_file_result;
    diplomat_external_hzo_extract_meta_from_file_result diplomat_external_hzo_extract_meta_from_file(diplomat::capi::DiplomatStringView path, uint16_t reader_version, diplomat::capi::DiplomatWrite* write);

    typedef struct diplomat_external_latex_to_mathml_result {union { diplomat::capi::HonzoErrorCode err;}; bool is_ok;} diplomat_external_latex_to_mathml_result;
    diplomat_external_latex_to_mathml_result diplomat_external_latex_to_mathml(diplomat::capi::DiplomatU8View bytes, diplomat::capi::DiplomatWrite* write);

    typedef struct diplomat_external_normalize_search_term_result {union { diplomat::capi::HonzoErrorCode err;}; bool is_ok;} diplomat_external_normalize_search_term_result;
    diplomat_external_normalize_search_term_result diplomat_external_normalize_search_term(diplomat::capi::DiplomatStringView term, diplomat::capi::DiplomatStringView lang, diplomat::capi::DiplomatWrite* write);

    typedef struct diplomat_external_render_math_result {union { diplomat::capi::HonzoErrorCode err;}; bool is_ok;} diplomat_external_render_math_result;
    diplomat_external_render_math_result diplomat_external_render_math(diplomat::capi::DiplomatU8View bytes, uint8_t math_type, diplomat::capi::DiplomatWrite* write);

    bool diplomat_external_validate_css(diplomat::capi::DiplomatU8View bytes);

    bool diplomat_external_validate_mathml(diplomat::capi::DiplomatU8View bytes);

    } // extern "C"
} // namespace capi
} // namespace


inline diplomat::result<std::string, HonzoErrorCode> guess_font_format(diplomat::span<const uint8_t> bytes) {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = diplomat::capi::diplomat_external_guess_font_format({bytes.data(), bytes.size()},
        &write);
    return result.is_ok ? diplomat::result<std::string, HonzoErrorCode>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, HonzoErrorCode>(diplomat::Err<HonzoErrorCode>(HonzoErrorCode::FromFFI(result.err)));
}
template<typename W>
inline diplomat::result<std::monostate, HonzoErrorCode> guess_font_format_write(diplomat::span<const uint8_t> bytes, W& writeable) {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = diplomat::capi::diplomat_external_guess_font_format({bytes.data(), bytes.size()},
        &write);
    return result.is_ok ? diplomat::result<std::monostate, HonzoErrorCode>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, HonzoErrorCode>(diplomat::Err<HonzoErrorCode>(HonzoErrorCode::FromFFI(result.err)));
}
inline diplomat::result<std::string, HonzoErrorCode> guess_image_mime(diplomat::span<const uint8_t> bytes) {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = diplomat::capi::diplomat_external_guess_image_mime({bytes.data(), bytes.size()},
        &write);
    return result.is_ok ? diplomat::result<std::string, HonzoErrorCode>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, HonzoErrorCode>(diplomat::Err<HonzoErrorCode>(HonzoErrorCode::FromFFI(result.err)));
}
template<typename W>
inline diplomat::result<std::monostate, HonzoErrorCode> guess_image_mime_write(diplomat::span<const uint8_t> bytes, W& writeable) {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = diplomat::capi::diplomat_external_guess_image_mime({bytes.data(), bytes.size()},
        &write);
    return result.is_ok ? diplomat::result<std::monostate, HonzoErrorCode>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, HonzoErrorCode>(diplomat::Err<HonzoErrorCode>(HonzoErrorCode::FromFFI(result.err)));
}
inline diplomat::result<diplomat::result<std::string, HonzoErrorCode>, diplomat::Utf8Error> hzo_extract_meta_from_file(std::string_view path, uint16_t reader_version) {
    if (!diplomat::capi::diplomat_is_str(path.data(), path.size())) {
    return diplomat::Err<diplomat::Utf8Error>();
  }
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = diplomat::capi::diplomat_external_hzo_extract_meta_from_file({path.data(), path.size()},
        reader_version,
        &write);
    return diplomat::Ok<diplomat::result<std::string, HonzoErrorCode>>(result.is_ok ? diplomat::result<std::string, HonzoErrorCode>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, HonzoErrorCode>(diplomat::Err<HonzoErrorCode>(HonzoErrorCode::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<diplomat::result<std::monostate, HonzoErrorCode>, diplomat::Utf8Error> hzo_extract_meta_from_file_write(std::string_view path, uint16_t reader_version, W& writeable) {
    if (!diplomat::capi::diplomat_is_str(path.data(), path.size())) {
    return diplomat::Err<diplomat::Utf8Error>();
  }
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = diplomat::capi::diplomat_external_hzo_extract_meta_from_file({path.data(), path.size()},
        reader_version,
        &write);
    return diplomat::Ok<diplomat::result<std::monostate, HonzoErrorCode>>(result.is_ok ? diplomat::result<std::monostate, HonzoErrorCode>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, HonzoErrorCode>(diplomat::Err<HonzoErrorCode>(HonzoErrorCode::FromFFI(result.err))));
}
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
inline diplomat::result<diplomat::result<std::string, HonzoErrorCode>, diplomat::Utf8Error> normalize_search_term(std::string_view term, std::string_view lang) {
    if (!diplomat::capi::diplomat_is_str(term.data(), term.size())) {
    return diplomat::Err<diplomat::Utf8Error>();
  }
    if (!diplomat::capi::diplomat_is_str(lang.data(), lang.size())) {
    return diplomat::Err<diplomat::Utf8Error>();
  }
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = diplomat::capi::diplomat_external_normalize_search_term({term.data(), term.size()},
        {lang.data(), lang.size()},
        &write);
    return diplomat::Ok<diplomat::result<std::string, HonzoErrorCode>>(result.is_ok ? diplomat::result<std::string, HonzoErrorCode>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, HonzoErrorCode>(diplomat::Err<HonzoErrorCode>(HonzoErrorCode::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<diplomat::result<std::monostate, HonzoErrorCode>, diplomat::Utf8Error> normalize_search_term_write(std::string_view term, std::string_view lang, W& writeable) {
    if (!diplomat::capi::diplomat_is_str(term.data(), term.size())) {
    return diplomat::Err<diplomat::Utf8Error>();
  }
    if (!diplomat::capi::diplomat_is_str(lang.data(), lang.size())) {
    return diplomat::Err<diplomat::Utf8Error>();
  }
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = diplomat::capi::diplomat_external_normalize_search_term({term.data(), term.size()},
        {lang.data(), lang.size()},
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
inline bool validate_css(diplomat::span<const uint8_t> bytes) {
    auto result = diplomat::capi::diplomat_external_validate_css({bytes.data(), bytes.size()});
    return result;
}
inline bool validate_mathml(diplomat::span<const uint8_t> bytes) {
    auto result = diplomat::capi::diplomat_external_validate_mathml({bytes.data(), bytes.size()});
    return result;
}


#endif // free_functions_HPP
