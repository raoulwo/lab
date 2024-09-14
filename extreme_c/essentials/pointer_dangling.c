#include <stdio.h>

int *create_int(int val) {
  int var = val;
  return &var;
}

// Dangling pointers are pointers to memory addresses
// of already deallocated values. In this example, we
// store the address of the stack-allocated variable
// `var` (local to `create_int`) in `ptr`. Since `var`
// is local to a function, it's automatically dropped
// when the function scope ends. Accessing invalid
// memory can lead to crashes or *segmentation faults*.
int main(int argc, char **argv) {
  int *ptr = NULL;
  ptr = create_int(0);
  printf("%d\n", *ptr);

  return 0;
}

// NOTE: Segmentation faults occur when trying to access
// memory you're not allowed to.
