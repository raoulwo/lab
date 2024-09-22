#include <stdio.h>

// *Parent class*
typedef struct {
  char first_name[32];
  char last_name[32];
  unsigned int birth_year;
} person_t;

// *Child class*
typedef struct {
  // NOTE: You can only use a `person_t` value directly if it's **not**
  // an incomplete type. For incomplete types that are forward declared,
  // you have to use a pointer instead.
  person_t person;
  char student_number[16];
  unsigned int passed_credits;
} student_t;

int main(int argc, char **argv) {
  student_t student;

  student_t *student_ptr = &student;
  // *Upcasting* `student_t *` to `person_t *`
  person_t *person_ptr = (person_t *)&student;

  printf("Student pointer points to: %p\n", (void *)student_ptr);
  printf("Person pointer points to: %p\n", (void *)person_ptr);

  return 0;
}
