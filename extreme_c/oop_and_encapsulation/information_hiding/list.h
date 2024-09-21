#ifndef LIST_H
#define LIST_H

#include <unistd.h>

// Information hiding by forward declaring the struct. The actual
// attributes of the struct aren't part of the public API. This
// is called an *incomplete type*.
struct list_t;

struct list_t *list_malloc();

void list_init(struct list_t *);
void list_destroy(struct list_t *);

int list_add(struct list_t *, int);
int list_get(struct list_t *, int, int *);
void list_clear(struct list_t *);
size_t list_size(struct list_t *);
void list_print(struct list_t *);

#endif
