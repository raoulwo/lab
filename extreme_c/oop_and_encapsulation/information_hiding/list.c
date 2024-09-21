#include <stdio.h>
#include <stdlib.h>

#define MAX_SIZE 10

typedef int bool_t;

typedef struct {
  size_t size;
  int *items;
} list_t;

// Here, we used the conventional prefix `__` to denote *private functions*.
// You can use another convention if you want to. Note that the private
// functions aren't declared in the header file.
bool_t __list_is_full(list_t *list) { return list->size == MAX_SIZE; }
bool_t __check_index(list_t *list, const int index) {
  return index >= 0 && index <= list->size;
};

// We provide the `list_malloc` function since the clients of the list API
// don't know about the size of `list_t` since we declare the struct as an
// incomplete type `struct size_t;`. Therefore, the clients can't determine
// the list size with `sizeof`.
list_t *list_malloc() { return (list_t *)malloc(sizeof(list_t)); }

void list_init(list_t *list) {
  list->size = 0;
  list->items = (int *)malloc(MAX_SIZE * sizeof(int));
}

void list_destroy(list_t *list) { free(list->items); }

int list_add(list_t *list, const int item) {
  if (__list_is_full(list)) {
    return -1;
  }
  list->items[list->size++] = item;
  return 0;
}

int list_get(list_t *list, const int index, int *result) {
  if (__check_index(list, index)) {
    *result = list->items[index];
    return 0;
  }
  return -1;
}

void list_clear(list_t *list) { list->size = 0; }

size_t list_size(list_t *list) { return list->size; }

void list_print(list_t *list) {
  printf("[");
  for (size_t i = 0; i < list->size; ++i) {
    printf("%d%s", list->items[i], i == list->size - 1 ? "" : ", ");
  }
  printf("]\n");
}
