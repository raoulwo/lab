#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define VERSION "2.3.4"

// It's possible to define variadic macros using
// the `...` syntax as a macro parameter. The variadic
// arguments can then be accessed in the macro using
// the special identifier `__VA_ARGS__`.
#define LOG_ERROR(format, ...) fprintf(stderr, format, __VA_ARGS__)

int main(int argc, char **argv) {
  if (argc < 3) {
    // NOTE: Since the macro definition doesn't end with a semicolon,
    // we have to end it whenever we use the macro, else it's not a
    // syntactically correct C program.
    LOG_ERROR("Invalid number of arguments for version %s\n", VERSION);
    exit(1);
  }

  if (strcmp(argv[1], "-n") != 0) {
    LOG_ERROR("%s is a wrong param at index %d for version %s.", argv[1], 1,
              VERSION);
    exit(1);
  }
}
