// C driver program to demonstrate calling printf() from assembly
// language.

#include <stdio.h>

extern void asmFunc(void);

int main(void) {
  // Need at least one call to printf() in the C program to allow
  // calling it from assembly.
  
  printf("calling `asmFunc`\n");
  asmFunc();
  printf("returned from `asmFunc`\n");
}
