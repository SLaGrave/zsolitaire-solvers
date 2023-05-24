#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>

#include "./cribbage-rs/cribbage-rs.h"

int main(void) {
  uint32_t sum = addition(1, 2);
  printf("%" PRIu32 "\n", sum);
}