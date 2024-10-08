#include <stdlib.h>

#include "animal.h"
#include "cat.h"
#include "duck.h"

int main(int argc, char **argv) {
  struct animal_t *animal = animal_new();
  struct cat_t *cat = cat_new();
  struct duck_t *duck = duck_new();

  animal_ctor(animal);
  cat_ctor(cat);
  duck_ctor(duck);

  animal_sound(animal);
  animal_sound((struct animal_t *)cat);
  animal_sound((struct animal_t *)duck);

  animal_dtor(animal);
  cat_dtor(cat);
  duck_dtor(duck);

  free(animal);
  free(cat);
  free(duck);

  return 0;
}
