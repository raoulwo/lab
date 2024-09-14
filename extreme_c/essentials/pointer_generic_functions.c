#include <stdio.h>

// Generic pointers are especially useful for defining
// *generic functions* (in order to achieve polymorphism)
// which can accept a wide variety of different pointers
// as args.

// NOTE: The data type `size_t` is part of the C99 spec
// and is an unsigned data type specifically used for
// storing sizes. It represents the size of pointers
// and is used wherever you'd use pointers (for example
// indexing into arrays).

// Pointer sizes differ between different architectures,
// however you can find out the sizes of data types using
// the `sizeof` operator (it's an operator not a function).
// As a rule of thumb, pointer sizes are:
//
// * 4 bytes in 32 bit architectures
// * 8 bytes in 64 bit architectures

// WARNING: Since pointer sizes depend on the underlying
// architecture, you shouldn't write any code that relies
// on pointer sizes!

void print_bytes(void *data, size_t length) {
  unsigned char *ptr = data;
  // Step size of `char` is 1, so we're iterating over
  // each byte one by one.
  for (size_t i = 0; i < length; ++i) {
    printf("0x%02x ", *ptr);
    ++ptr;
  }
  printf("\n");
}

int main(int argc, char **argv) {
  int a = 9;
  double b = 18.9;

  print_bytes(&a, sizeof a);
  print_bytes(&b, sizeof b);
}
