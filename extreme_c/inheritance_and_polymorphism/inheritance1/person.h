#ifndef PERSON_H
#define PERSON_H

// Forward declaration of an incomplete type since we don't want
// to expose the internals of our `person_t` struct. Only the
// *subclasses* of `person_t` should have access to its internals.
// That's where *private header files* come in.
struct person_t;

struct person_t *person_new();

void person_ctor(struct person_t *, const char *, const char *, unsigned int);
void person_dtor(struct person_t *);

void person_get_first_name(struct person_t *, char *);
void person_get_last_name(struct person_t *, char *);
unsigned int person_get_birth_year(struct person_t *);

#endif
