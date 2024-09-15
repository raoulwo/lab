#include <stdio.h>

#include "geometry.h"

int main(int argc, char **argv) {
  cartesian_pos_2d_t cartesian_pos;
  cartesian_pos.x = 3;
  cartesian_pos.y = 4;

  polar_pos_2d_t polar_pos = convert_to_polar_2d(&cartesian_pos);

  printf("Polar Coordinates:\n");
  printf("Length: %f\n", polar_pos.length);
  printf("Theta: %f\n", polar_pos.theta);

  return 0;
}
