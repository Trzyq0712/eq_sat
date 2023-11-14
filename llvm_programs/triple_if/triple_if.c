#include <stdint.h>

int64_t triple_if(int64_t a, int64_t b, int64_t c, int64_t cond) {
  if (cond == 0) {
    return a;
  } else if (cond == 1) {
    return b;
  } else {
    return c;
  }
}
