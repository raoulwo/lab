#ifndef ANIMAL_PRIVATE_H
#define ANIMAL_PRIVATE_H

// This function pointer type needs to point to
// different morphs of `animal_sound`.
typedef void (*sound_func_t)(void *);

typedef struct {
  char *name;
  sound_func_t sound_func;
} animal_t;

#endif
