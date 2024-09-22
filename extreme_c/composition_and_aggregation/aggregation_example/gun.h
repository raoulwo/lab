#ifndef GUN_H
#define GUN_H

typedef int bool_t;

// Forward declaration of incomplete type that can't
// be instantiated.
struct gun_t;

struct gun_t *gun_new();

void gun_ctor(struct gun_t *, int);
void gun_dtor(struct gun_t *);

bool_t gun_has_bullets(struct gun_t *);
void gun_trigger(struct gun_t *);
void gun_reload(struct gun_t *);

#endif
