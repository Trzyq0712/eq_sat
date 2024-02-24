#include <stdint.h>

int64_t gauss_sum(int64_t n) {
  int64_t sum = 0;
  for (int64_t i = 1; i <= n; i++) {
    for (int64_t j = 1; j <= i; j++) {
      sum += 1;
    }
  }
  return sum;
}
