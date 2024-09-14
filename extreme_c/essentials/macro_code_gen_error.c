#include <stdio.h>

// Macros can be used for *code generation* before
// compilation. That way, macros can be used as a
// metaprogramming mechanism.

// NOTE: Compilers like `gcc` and `clang` support
// the `-E` flag used to dump the *translation unit*
// after preprocessing.

// NOTE: A **translation unit** (or **compilation unit**)
// is the preprocessed C code ready to be passed to the
// compiler. In translation units, all preprocessor directives
// are already substituted with expanded macros, or file
// inclusions. The result is a flat (possibly long) piece
// of C code.

#define PRINT printf("%d\n", i);

int main(int argc, char **argv) {
  PRINT

  return 0;
}
