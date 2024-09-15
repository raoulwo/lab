#include <dlfcn.h>
#include <stdio.h>
#include <stdlib.h>

#include "geometry.h"

typedef polar_pos_2d_t (*func_ptr)(cartesian_pos_2d_t *);

int main(int argc, char **argv) {
  void *handle = dlopen("/home/raoul/repos/lab/extreme_c/object_files/"
                        "shared_object_lazy/libgeometry.so",
                        RTLD_LAZY);
  if (!handle) {
    fprintf(stderr, "%s\n", dlerror());
    exit(1);
  }

  func_ptr convert_ptr = dlsym(handle, "convert_to_polar_2d");
  if (!convert_ptr) {
    fprintf(stderr, "%s\n", dlerror());
    exit(1);
  }

  cartesian_pos_2d_t cartesian_pos;
  cartesian_pos.x = 3;
  cartesian_pos.y = 4;

  polar_pos_2d_t polar_pos = convert_ptr(&cartesian_pos);
  printf("Polar Coordinates:\n");
  printf("Length: %f\n", polar_pos.length);
  printf("Theta: %f\n", polar_pos.theta);

  return 0;
}
