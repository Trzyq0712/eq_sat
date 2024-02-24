#include <stdint.h>

int64_t nested_ifs(int64_t a) {
  if (a < 0) {
    if (a == -1) {
      return -11;
    } else {
      return -2;
    }
  } else {
    if (a == 1) {
      return 11;
    } else {
      return 2;
    }
  }
}
