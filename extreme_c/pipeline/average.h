#ifndef AVERAGE_H
#define AVERAGE_H

typedef enum { NONE, NORMAL, SQUARED } average_type_t;

double avg(int *, int, average_type_t);

#endif
