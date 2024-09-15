#include <math.h>

#include "geometry.h"

cartesian_pos_2d_t convert_to_cartesian_2d(const polar_pos_2d_t *polar_pos) {
  cartesian_pos_2d_t result;
  result.x = polar_pos->length * cos_deg(polar_pos->theta);
  result.y = polar_pos->length * sin_deg(polar_pos->theta);
  return result;
}

polar_pos_2d_t convert_to_polar_2d(const cartesian_pos_2d_t *cartesian_pos) {
  polar_pos_2d_t result;
  result.length = sqrt(cartesian_pos->x * cartesian_pos->x +
                       cartesian_pos->y * cartesian_pos->y);
  result.theta = to_deg(atan(cartesian_pos->y / cartesian_pos->x));
  return result;
}
