#include <stdlib.h>

#include "engine.h"

typedef struct {
  // `car_t` is *composed* of `engine_t`. The `engine` pointer is *private*
  // and doesn't leak out of the API. The type `struct engine_t` is incomplete
  // in this source file, however the pointer can still reference a complete
  // `engine_t` struct at runtime. The `car_t` only interacts with `engine_t`s
  // public API, it doesn't know anything about its implementation details.
  struct engine_t *engine;
} car_t;

car_t *car_new() { return (car_t *)malloc(sizeof(car_t)); }

void car_ctor(car_t *car) {
  car->engine = engine_new();
  engine_ctor(car->engine);
}

void car_dtor(car_t *car) {
  engine_dtor(car->engine);
  free(car->engine);
}

void car_start(car_t *car) { engine_turn_on(car->engine); }

void car_stop(car_t *car) { engine_turn_off(car->engine); }

double car_get_engine_temperature(car_t *car) {
  return engine_get_temperature(car->engine);
}
