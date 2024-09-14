#include <stdio.h>

// Function pointers are, like the name suggests,
// pointers to functions. Their most important
// application is for splitting large binaries
// into smaller ones that can then be loaded again.
// They're also the underlying building blocks for
// implementing polymorphism in C++.

// Function pointers are the only way of supporting
// polymorphism and virtual functions in C.

int sum(int a, int b) { return a + b; }
int subtract(int a, int b) { return a - b; }

int main(int argc, char **argv) {
  // `func_ptr` is a pointer to a function of the
  // following signature:
  //
  //   int (*)(int, int)
  //
  int (*func_ptr)(int, int) = NULL;

  func_ptr = &sum;
  int res = func_ptr(3, 2);
  printf("sum: %d\n", res);

  func_ptr = &subtract;
  res = func_ptr(3, 2);
  printf("subtract: %d\n", res);

  return 0;
}
