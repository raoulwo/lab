#include <stdio.h>

// Call-by-reference doesn't exist, in order to
// mutate values in place you can pass pointers
// to functions however. The pointers are passed
// by value (they're copied), the underlying value
// (the memory address) stays the same however.
// This way of passing pointers to functions is
// also referred to as call-by-pointer.
void func(int *a) {
  int b = -1;
  // The underlying value is mutated.
  ++*a;
  // The pointer is passed by value, the original
  // pointer remains unchanged.
  a = &b;
}

int main(int argc, char **argv) {
  int a = 0;
  int *a_ptr = &a;

  printf("value before function call: %d\n", a);
  printf("pointer before function call: %p\n", (void *)a_ptr);
  func(a_ptr);
  printf("value after function call: %d\n", a);
  printf("pointer after function call: %p\n", (void *)a_ptr);

  return 0;
}
