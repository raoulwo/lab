#include <stdio.h>

// Just like with primitive data types, it's
// possible to have pointers to structs as well.
// The arithmetic step size of pointers to structs
// is (like with everything else) determined by the
// struct size.

// NOTE: A struct variable starting memory address
// is the same as its first elements' memory address.

typedef struct {
  float x;
  float y;
} point_t;

typedef struct {
  point_t center;
  float radius;
} circle_t;

int main(int argc, char **argv) {
  circle_t c;

  // The following three pointers all access the same
  // memory location (the first field of `c`), however
  // the pointer types are *different*. This can be used
  // to extend structs from other libraries or to implement
  // something akin to *inheritance* in C.
  circle_t *p1 = &c;
  point_t *p2 = (point_t *)&c;
  float *p3 = (float *)&c;

  printf("p1: %p\n", (void *)p1);
  printf("p2: %p\n", (void *)p2);
  printf("p3: %p\n", (void *)p3);

  return 0;
}
