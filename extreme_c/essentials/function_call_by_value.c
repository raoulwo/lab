#include <stdio.h>

// Higher level languages usually differentiate when
// calling functions between call-by-value and
// call-by-reference. Since C doesn't have any notion
// of references, there only exists call-by-value.

void func(int a) { ++a; }

int main(int argc, char **argv) {
  int a = 0;

  printf("before function call: %d\n", a);
  func(a);
  printf("after function call: %d\n", a);

  return 0;
}
