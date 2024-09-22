#include <stdlib.h>
#include <string.h>

// We don't have a private header for the implementation of `person_t`
// anymore. Now we can't access its internals from within other compilation
// units.
typedef struct {
  char first_name[32];
  char last_name[32];
  unsigned int birth_year;
} person_t;

person_t *person_new() { return (person_t *)malloc(sizeof(person_t)); }

void person_ctor(person_t *person, const char *first_name,
                 const char *last_name, unsigned int birth_year) {
  strcpy(person->first_name, first_name);
  strcpy(person->last_name, last_name);
  person->birth_year = birth_year;
}
void person_dtor(person_t *person) {}

void person_get_first_name(person_t *person, char *buffer) {
  strcpy(buffer, person->first_name);
}
void person_get_last_name(person_t *person, char *buffer) {
  strcpy(buffer, person->last_name);
}
unsigned int person_get_birth_year(person_t *person) {
  return person->birth_year;
}
