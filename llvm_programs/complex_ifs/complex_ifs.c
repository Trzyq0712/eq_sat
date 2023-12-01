#include <stdint.h>

int64_t complex_ifs(int64_t a, int64_t b, int64_t c) {
  int64_t x = 0;
  x += 2;
  if (a > b) {
    x -= 3;
  } else {
    x += 5;
    if (x > c) {
      x += 7;
    }
  }

  x = x * 10;

  return x;
}
