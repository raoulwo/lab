#include <stdlib.h>

int main(int argc, char **argv) {
  // Memory leak that can be analyzed using valgrind.
  // To do so, we need to compile using debug symbols.
  char *ptr = (char *)malloc(16 * sizeof(char));

  return 0;
}
