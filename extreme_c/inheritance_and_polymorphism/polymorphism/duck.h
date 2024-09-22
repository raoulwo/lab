#ifndef DUCK_H
#define DUCK_H

struct duck_t;

struct duck_t *duck_new();

void duck_ctor(struct duck_t *);
void duck_dtor(struct duck_t *);

#endif
