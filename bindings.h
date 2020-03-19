#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

char *rust_hello(const char *to);

void rust_hello_free(char *s);

void use_swift_closure(int32_t (*swift_fn)(int32_t));
