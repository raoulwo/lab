#ifndef STUDENT_H
#define STUDENT_H

struct student_t;

struct student_t *student_new();

void student_ctor(struct student_t *, const char *, const char *, unsigned int,
                  const char *, unsigned int);
void student_dtor(struct student_t *);

void student_get_student_number(struct student_t *, char *);
unsigned int student_get_passed_credits(struct student_t *);

#endif
