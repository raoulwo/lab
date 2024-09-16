#include <stdio.h>
#include <stdlib.h>

#define QUEUE_MAX_SIZE 100

typedef struct {
  int front;
  int rear;
  double *arr;
} queue_t;

void init(queue_t *q) {
  q->front = q->rear = 0;
  // The heap allocated memory is *owned* by the queue.
  q->arr = (double *)malloc(QUEUE_MAX_SIZE * sizeof(double));
}

void destroy(queue_t *q) { free(q->arr); }

int size(queue_t *q) { return q->rear - q->front; }

void enqueue(queue_t *q, double item) {
  q->arr[q->rear] = item;
  q->rear++;
}

double dequeue(queue_t *q) {
  double item = q->arr[q->front];
  q->front++;
  return item;
}

int main(int argc, char **argv) {
  queue_t *q = (queue_t *)malloc(sizeof(queue_t));

  init(q);

  enqueue(q, 6.5);
  enqueue(q, 1.3);
  enqueue(q, 2.4);

  printf("%f\n", dequeue(q));
  printf("%f\n", dequeue(q));
  printf("%f\n", dequeue(q));

  // Order is important, first deallocate the *owned* value
  // and only then the *owner*.
  destroy(q);
  free(q);

  // WARNING: Besides memory leaks, you also need to avoid
  // *double frees* since they can lead to memory corruption.

  return 0;
}

// Alternatives to ownership strategy:
//
// * Garbage collection
// * Smart pointers (reference counting)
// * RAII

// NOTE: Heap allocations/deallocations come at a significant
// cost. In memory/time constrained programs, what's often done
// is that a large chunk of memory is allocated once upfront
// and that memory block is then managed by the programmers.
// It can be subdivided into smaller memory blocks for example.
