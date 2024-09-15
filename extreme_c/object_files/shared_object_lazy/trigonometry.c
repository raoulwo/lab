#include <math.h>

#include "geometry.h"

double to_rad(double deg) { return PI * deg / 180; }
double to_deg(double rad) { return 180 * rad / PI; }

double cos_deg(double deg) { return cos(to_rad(deg)); }
double acos_deg(double deg) { return acos(to_rad(deg)); }

double sin_deg(double deg) { return sin(to_rad(deg)); }
double asin_deg(double deg) { return asin(to_rad(deg)); }
