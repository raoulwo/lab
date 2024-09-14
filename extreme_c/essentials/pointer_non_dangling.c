#include <stdio.h>
#include <stdlib.h>

int *create_int(int val) {
  int *var_ptr = (int *)malloc(sizeof(int));
  *var_ptr = val;
  return var_ptr;
}

int main(int argc, char **argv) {
  int *ptr = NULL;
  ptr = create_int(0);
  printf("%d\n", *ptr);
  free(ptr);

  return 0;
}
