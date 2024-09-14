#include <stdio.h>

// NOTE: Macros, as with other programming languages, can
// be used to create DSLs.

#define PRINT(val) printf("%d\n", val);
#define LOOP(index, start, end) for (int index = start; index <= end; ++index) {
#define ENDLOOP }

int main(int argc, char **argv) {
  LOOP(i, 1, 10)
  PRINT(i)
  ENDLOOP
}
