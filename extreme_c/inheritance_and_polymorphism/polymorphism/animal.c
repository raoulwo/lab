#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "animal_private.h"

// Default definition of `animal_sound`
void __animal_sound(void *this) {
  animal_t *animal = (animal_t *)this;
  printf("%s: Beeeep\n", animal->name);
}

animal_t *animal_new() { return (animal_t *)malloc(sizeof(animal_t)); }

void animal_ctor(animal_t *animal) {
  animal->name = (char *)malloc(10 * sizeof(char));
  strcpy(animal->name, "Animal");
  // Set the function pointer to point to the default implementation
  animal->sound_func = __animal_sound;
}

void animal_dtor(animal_t *animal) { free(animal->name); }

void animal_get_name(animal_t *animal, char *buffer) {
  strcpy(buffer, animal->name);
}

void animal_sound(animal_t *animal) {
  // Call the function referenced by the function pointer
  animal->sound_func(animal);
}
