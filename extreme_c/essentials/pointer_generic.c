#include <stdio.h>

// Pointers of type `void *` are generic pointers.
// They can point to any address like all other pointers,
// the underlying data type, and therefore the arithmetic
// step size, is unknown however.

int main(int argc, char **argv) {
  int num = 1;
  int *int_ptr = &num;
  void *generic_ptr = &num;
  // WARNING: Generic pointers can't be dereferenced before
  // you cast them into a different type (The assignment to
  // a void pointer **doesn't** need an explicit cast).
  printf("%d\n", *generic_ptr);

  return 0;
}
