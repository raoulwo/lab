#include <math.h>

#include "geometry.h"

cartesian_pos_3d_t convert_to_cartesian_3d(const polar_pos_3d_t *polar_pos) {
  cartesian_pos_3d_t result;
  result.x =
      polar_pos->length * sin_deg(polar_pos->theta) * cos_deg(polar_pos->phi);
  result.y =
      polar_pos->length * sin_deg(polar_pos->theta) * sin_deg(polar_pos->phi);
  result.z = polar_pos->length * cos_deg(polar_pos->theta);
  return result;
}

polar_pos_3d_t convert_to_polar_3d(const cartesian_pos_3d_t *cartesian_pos) {
  polar_pos_3d_t result;
  result.length = sqrt(cartesian_pos->x * cartesian_pos->x +
                       cartesian_pos->y * cartesian_pos->y +
                       cartesian_pos->z * cartesian_pos->z);
  result.theta = to_deg(acos(cartesian_pos->x / result.length));
  result.phi = to_deg(atan(cartesian_pos->y / cartesian_pos->x));
  return result;
}
