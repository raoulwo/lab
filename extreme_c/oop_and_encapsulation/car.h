#ifndef CAR_H
#define CAR_H

typedef struct {
  char name[32];
  double speed;
  double fuel;
} car_t;

void car_construct(car_t *car, const char *name);
void car_destruct(car_t *car);
void car_accelerate(car_t *car);
void car_brake(car_t *car);
void car_refuel(car_t *car, double fuel);

#endif
