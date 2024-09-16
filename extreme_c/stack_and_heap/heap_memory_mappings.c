#include <stdio.h>
#include <stdlib.h>

void print_memory_maps() {
#ifdef __linux__
  FILE *fd = fopen("/proc/self/maps", "r");
  if (!fd) {
    printf("couldn't open maps file\n");
    exit(1);
  }
  char line[1024];
  while (!feof(fd)) {
    fgets(line, 1024, fd);
    printf("> %s", line);
  }
#endif
}

int main(int argc, char **argv) {
  // `malloc` => memory allocate
  char *ptr1 = (char *)malloc(10 * sizeof(char));
  printf("address of ptr1: %p\n", (void *)&ptr1);
  printf("memory allocated by malloc at %p: ", (void *)ptr1);
  for (int i = 0; i < 10; ++i) {
    printf("0x%02x ", (unsigned char)ptr1[i]);
  }
  printf("\n");

  // `malloc` => clear and allocate
  char *ptr2 = (char *)calloc(10, sizeof(char));
  printf("address of ptr2: %p\n", (void *)&ptr2);
  printf("memory allocated by calloc at %p: ", (void *)ptr2);
  for (int i = 0; i < 10; ++i) {
    printf("0x%02x ", (unsigned char)ptr2[i]);
  }
  printf("\n");

  // NOTE: `malloc` is usually faster than `calloc` since
  // it doesn't initialize the allocated memory. You can
  // use `memset` to initialize the data after allocating
  // it using `malloc`.

  // NOTE: Another memory allocation function is `realloc`
  // which can be used to reallocate memory by resizing an
  // already allocated memory block. This is an expensive
  // operation.

  print_memory_maps();

  free(ptr1);
  free(ptr2);

  return 0;
}
