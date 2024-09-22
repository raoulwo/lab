#include <string.h>

class Foo {
public:
  char c;
  char d;
};

class Bar : public Foo {
public:
  char str[5];
};

int main(int argc, char **argv) {
  Bar bar;
  bar.c = 'A';
  bar.d = 'B';
  strcpy(bar.str, "1234");
  // Breakpoint

  return 0;
}
