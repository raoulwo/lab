#include <stdio.h>

// Conditional compilation allows you to have different
// preprocessed source code depending on different
// conditions. These conditions are evaluated by the
// preprocessor, not the compiler.

// NOTE: You can define macros when invoking a compiler
// by using the `-D` option, passing the name of the
// macro as an argument:
//
// * `gcc -E cond_comp_simple.c`
// * `gcc -E -DCONDITION cond_comp_simple.c`
//

// NOTE: Conditional compilation is very useful when
// compiling a single source for different architectures
// or operating systems.

int main(int argc, char **argv) {
#ifdef CONDITION
  printf("Hello, world!\n");
#endif

  return 0;
}
