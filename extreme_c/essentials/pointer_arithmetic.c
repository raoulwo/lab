#include <stdio.h>

// You can think of memory as a very long one-dimensional
// array of bytes (that's a simplified model). Pointers
// are then just used to access specific elements in memory:
//
// * Incrementing pointers causes them to *go forward*.
// * Decrementing pointers causes them th *go backward*.
//
// These possible arithmetic operations on pointers are
// similar to the possible movements in arrays of bytes.

// Another important concept in regards to pointers is the
// **arithmetic step size**. This step size determines by how
// much a pointer moves forward/backward when it's incremented
// or decremented. This step size is determined by the
// underlying C data type of the pointer:
//
// * `char` 1 byte
// * `short` 2 bytes
// * `int` 4 bytes
// * `long` 4 bytes
// * `long long` 8 bytes
// * `float` 4 bytes
// * `double` 8 bytes
//
// (The numbers above depend on the system)

int main(int argc, char **argv) {
  int num = 1;

  char *char_ptr = (char *)&num;
  short *short_ptr = (short *)&num;
  int *int_ptr = &num;

  printf("Before incrementing the pointers:\n");
  printf("char_ptr: %u\n", (unsigned int)char_ptr);
  printf("short_ptr: %u\n", (unsigned int)short_ptr);
  printf("int_ptr: %u\n\n", (unsigned int)int_ptr);

  ++char_ptr;
  ++short_ptr;
  ++int_ptr;

  printf("After incrementing the pointers:\n");
  printf("char_ptr: %u\n", (unsigned int)char_ptr);
  printf("short_ptr: %u\n", (unsigned int)short_ptr);
  printf("int_ptr: %u\n\n", (unsigned int)int_ptr);
}
