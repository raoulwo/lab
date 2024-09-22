#ifndef PERSON_PRIVATE_H
#define PERSON_PRIVATE_H

// NOTE: This is a *private header file*. Its purpose is to be
// only included in the child classes that want to access the
// private internals of `person_t`.

typedef struct {
  char first_name[32];
  char last_name[32];
  unsigned int birth_year;
} person_t;

#endif
