#include <stdint.h>

int64_t double_loop(int64_t n) {
  int64_t sum = 0;
  for (int64_t i = 0; i < n; i++) {
    for (int64_t j = 0; j < n; j++) {
      sum += i * j;
    }
  }
  return sum;
}
