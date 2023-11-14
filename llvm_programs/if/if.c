#include <stdint.h>

int64_t if_expr(int64_t a, int64_t b, int64_t c) {
  if (c) {
    return a;
  } else {
    return b;
  }
}
