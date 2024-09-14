#include <stdio.h>

#define SIZE 5

// A classic use case for pointer arithmetic is
// when iterating over regions of memory.

int main(int argc, char **argv) {
  int arr[SIZE];
  arr[0] = 9;
  arr[1] = 22;
  arr[2] = 30;
  arr[3] = 23;
  arr[4] = 18;

  // NOTE: `&arr[0]` is equivalent to `&arr`.
  int *ptr = &arr[0];

  for (;;) {
    printf("%d\n", *ptr);
    if (ptr == &arr[SIZE - 1]) {
      break;
    }
    ++ptr;
  }

  return 0;
}
