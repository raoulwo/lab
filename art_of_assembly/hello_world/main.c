// A simple C++ program that calls an assembly language function.

#include <stdio.h>

// Here's the external function, written in assembly
// language, that this program will call:
extern void asmFunc(void);

int main(void) {
  printf("calling `asmFunc`\n");
  asmFunc();
  printf("returned from `asmFunc`\n");
}
