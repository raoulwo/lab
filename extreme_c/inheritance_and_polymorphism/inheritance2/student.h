#ifndef STUDENT_H
#define STUDENT_H

// NOTE: The main difference to the first approach to inheritance is that
// now we'll keep a pointer to `person_t` instead of keeping a value directly.
// This time, we'll only rely on the public API of `person_t`.

struct student_t;

struct student_t *student_new();

void student_ctor(struct student_t *, const char *, const char *, unsigned int,
                  const char *, unsigned int);
void student_dtor(struct student_t *);

// The functions originally part of the `person_t` API become part of the
// `student_t` API since we can't *upcast* the `student_t` anymore. That's
// because the first element of `student_t` won't be a `person_t` anymore.
void student_get_first_name(struct student_t *, char *);
void student_get_last_name(struct student_t *, char *);
unsigned int student_get_birth_year(struct student_t *);

void student_get_student_number(struct student_t *, char *);
unsigned int student_get_passed_credits(struct student_t *);

#endif
