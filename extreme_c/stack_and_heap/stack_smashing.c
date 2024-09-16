#include <string.h>

int main(int argc, char **argv) {
  char str[5];
  // Here, `strcpy` is overwriting parts of the stack.
  // This type of bug is also referred to as *stack
  // smashing*.
  strcpy(str, "012345");

  return 0;
}
