#ifndef GEOMETRY_H
#define GEOMETRY_H

#define PI 3.14159265359

typedef struct {
  double x;
  double y;
} cartesian_pos_2d_t;

typedef struct {
  double length;
  double theta;
} polar_pos_2d_t;

typedef struct {
  double x;
  double y;
  double z;
} cartesian_pos_3d_t;

typedef struct {
  double length;
  double theta;
  double phi;
} polar_pos_3d_t;

double to_rad(double deg);
double to_deg(double rad);

double cos_deg(double deg);
double acos_deg(double deg);

double sin_deg(double deg);
double asin_deg(double deg);

cartesian_pos_2d_t convert_to_cartesian_2d(const polar_pos_2d_t *polar_pos);
polar_pos_2d_t convert_to_polar_2d(const cartesian_pos_2d_t *cartesian_pos);

cartesian_pos_3d_t convert_to_cartesian_3d(const polar_pos_3d_t *polar_pos);
polar_pos_3d_t convert_to_polar_3d(const cartesian_pos_3d_t *cartesian_pos);

#endif
