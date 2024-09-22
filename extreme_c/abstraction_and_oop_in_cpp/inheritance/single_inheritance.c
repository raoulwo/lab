#include <string.h>

typedef struct {
  char c;
  char d;
} foo_t;

typedef struct {
  foo_t parent;
  char str[5];
} bar_t;

int main(int argc, char **argv) {
  bar_t bar;
  bar.parent.c = 'A';
  bar.parent.d = 'B';
  strcpy(bar.str, "1234");
  // Breakpoint

  return 0;
}
