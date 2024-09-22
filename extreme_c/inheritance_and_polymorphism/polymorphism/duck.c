#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "animal.h"
#include "animal_private.h"

typedef struct {
  animal_t animal;
} duck_t;

// Define a new behaviour for the animal sound
void __duck_sound(void *ptr) {
  animal_t *animal = (animal_t *)ptr;
  printf("%s: Quack\n", animal->name);
}

duck_t *duck_new() { return (duck_t *)malloc(sizeof(duck_t)); }

void duck_ctor(duck_t *duck) {
  animal_ctor((struct animal_t *)duck);
  strcpy(duck->animal.name, "Duck");
  // Override the default behaviour
  duck->animal.sound_func = __duck_sound;
}
void duck_dtor(duck_t *duck) { animal_dtor((struct animal_t *)duck); }
