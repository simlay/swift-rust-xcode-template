#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * `FfiStr`, from the `ffi-support` crate, makes passing C-style (null-terminated) strings a little more convenient
 * and a little safer. The `to` and `name` variables here cannot outlive the call to `rust_hello`.
 */
char *rust_hello(FfiStr to) NS_SWIFT_NAME(rust_hello(to:));

void use_swift_closure(int32_t (*swift_fn)(int32_t)) NS_SWIFT_NAME(use_swift_closure(swift_fn:));
