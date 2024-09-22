#include <stdlib.h>
#include <string.h>

#include "person.h"

typedef struct {
  char *student_number;
  unsigned int passed_credits;
  // `person` doesn't need to be the first element anymore.
  struct person_t *person;
} student_t;

student_t *student_new() { return (student_t *)malloc(sizeof(student_t)); }

void student_ctor(student_t *student, const char *first_name,
                  const char *last_name, unsigned int birth_year,
                  const char *student_number, unsigned int passed_credits) {
  student->person = person_new();
  person_ctor(student->person, first_name, last_name, passed_credits);
  student->student_number = (char *)malloc(16 * sizeof(char));
  strcpy(student->student_number, student_number);
  student->passed_credits = passed_credits;
}
void student_dtor(student_t *student) {
  free(student->student_number);
  person_dtor(student->person);
  free(student->person);
}

void student_get_first_name(student_t *student, char *buffer) {
  person_get_first_name(student->person, buffer);
}
void student_get_last_name(student_t *student, char *buffer) {
  person_get_last_name(student->person, buffer);
}
unsigned int student_get_birth_year(student_t *student) {
  return person_get_birth_year(student->person);
}
void student_get_student_number(student_t *student, char *buffer) {
  strcpy(buffer, student->student_number);
}
unsigned int student_get_passed_credits(student_t *student) {
  return student->passed_credits;
}
