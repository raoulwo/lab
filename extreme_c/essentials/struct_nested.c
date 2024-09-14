#include <stdio.h>

// Structs are used for defining user-defined
// types UDTs. They can hold primitive types
// or other structs (UDTs).

// Calculating the sizes of nested structs
// is just like with simple structs, again
// memory layout and alignment needs to be
// taken into consideration.

typedef struct {
  float x;
  float y;
} point_t;

typedef struct {
  point_t center;
  float radius;
} circle_t;

typedef struct {
  point_t start;
  point_t end;
} line_t;

int main() {
  printf("sizeof(point_t) = %lu\n", sizeof(point_t));
  printf("sizeof(circle_t) = %lu\n", sizeof(circle_t));
  printf("sizeof(line_t) = %lu\n", sizeof(line_t));

  return 0;
}
