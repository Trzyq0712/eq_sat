#include <stdint.h>

int64_t while_loop(int64_t times, int64_t add) {
  int64_t s = 0;
  while (times > 0) {
    s += add;
    times--;
  }
  return s;
}
