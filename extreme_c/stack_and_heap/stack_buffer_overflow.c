#include <stdio.h>
#include <string.h>

int main(int argc, char **argv) {
  char str[10];
  // WARNING: Writing unchecked values into a *buffer*
  // (or byte/character array) allocated on the stackd
  // is considered a vulnerability. Attackers can design
  // an input buffer in order to take control of the
  // program. This is called a *buffer overflow* attack.
  // If you're lucky, this leads to a crash, in some
  // cases it leads to an exploit instead.
  strcpy(str, argv[1]);
  printf("Hello, %s!\n", str);

  return 0;
}
