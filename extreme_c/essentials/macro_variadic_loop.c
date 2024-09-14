#include <stdio.h>

// The only way of using macros for creating loops is
// to just put the instructions for each iteration one
// after the other. This technique is called *loop
// unrolling* and can lead to better performance, with
// the trade-off of larger binary sizes.

// NOTE: A rule of thumb when working with macros
// (regardless of language):
//
// If a macro can be written as a C function, then you
// should write a C function instead!

#define LOOP_3(arg, ...) printf("%s\n", #arg);

#define LOOP_2(arg, ...)                                                       \
  printf("%s\n", #arg);                                                        \
  LOOP_3(__VA_ARGS__)

#define LOOP_1(arg, ...)                                                       \
  printf("%s\n", #arg);                                                        \
  LOOP_2(__VA_ARGS__)

#define LOOP(...) LOOP_1(__VA_ARGS__)

int main(int argc, char **argv) {
  LOOP(copy paste cut)
  LOOP(copy, paste, cut)
  LOOP(copy, paste, cut, delete)

  return 0;
}
