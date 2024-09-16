#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int main(int argc, char **argv) {
  void *ptr = malloc(1024);
  printf("address: %p\n", ptr);
  fflush(stdout);

  while (1) {
    sleep(1);
  }

  return 0;
}
