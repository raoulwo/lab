// Generally speaking, main memory access of the CPU
// is *orders of magnitudes* slower than accessing the
// same data from CPU caches. That is why for better
// better performance, we want to design programs in a
// way that *minimizes cache misses*.

// When reading data from main memory, the CPU fetches
// a *bucket* of items to be cached for the upcoming
// memory access attempts. Besides the piece of data
// itself, the other items in the bucket are one's
// that are located in the *proximity* of the datum.
// That's because of the *principle of locality*
// which describes that in computer systems, it's
// usually observed that data located in the same
// proximity is more likely to be accessed next
// (Think array/struct access since they are both
// represented as contiguous regions in memory).

// By designing our algorithms and data structures
// around this *principle of locality*, we can try
// to minimize cache misses, thusly improving our
// performance!

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// In this example, we create a matrix in a one-
// dimensional buffer. We store it in *row-major
// order* this means we store it *row by row* as
// opposed to *column-major order* (col by col).

// To leverage the principle of locality and
// increase our chance at cache hits, we can
// iterate over our matrix in row-major order,
// the same order as it's stored in memory in.

// When testing both the friendly and unfriendly
// sums using the `test` command you can see
// the difference cache-friendliness makes!

void fill(int *matrix, int rows, int cols) {
  int counter = 1;
  for (int i = 0; i < rows; ++i) {
    for (int j = 0; j < cols; ++j) {
      *(matrix + i * cols + j) = counter;
    }
    counter++;
  }
}

void print_matrix(int *matrix, int rows, int cols) {
  printf("matrix:\n");
  for (int i = 0; i < rows; ++i) {
    for (int j = 0; j < cols; ++j) {
      printf("%d ", *(matrix + i * cols + j));
    }
    printf("\n");
  }
}

void print_flat(int *matrix, int rows, int cols) {
  printf("flat matrix:\n");
  for (int i = 0; i < (rows * cols); i++) {
    printf("%d ", *(matrix + i));
  }
  printf("\n");
}

int friendly_sum(int *matrix, int rows, int cols) {
  int sum = 0;
  for (int i = 0; i < rows; ++i) {
    for (int j = 0; j < cols; ++j) {
      sum += *(matrix + i * cols + j);
    }
  }

  return sum;
}

int unfriendly_sum(int *matrix, int rows, int cols) {
  int sum = 0;
  for (int j = 0; j < cols; ++j) {
    for (int i = 0; i < rows; ++i) {
      sum += *(matrix + i * cols + j);
    }
  }

  return sum;
}

int main(int argc, char **argv) {
  if (argc < 4) {
    printf("usage: %s [print|friendly-sum|unfriendly-sum] ", argv[0]);
    printf("[number-of-rows] [number-of-cols]\n");
    exit(1);
  }

  char *operation = argv[1];
  int rows = atol(argv[2]);
  int cols = atol(argv[3]);

  int *matrix = (int *)malloc(rows * cols * sizeof(int));
  fill(matrix, rows, cols);

  if (strcmp(operation, "print") == 0) {
    print_matrix(matrix, rows, cols);
    print_flat(matrix, rows, cols);
  } else if (strcmp(operation, "friendly-sum") == 0) {
    int sum = friendly_sum(matrix, rows, cols);
    printf("friendly sum: %d\n", sum);
  } else if (strcmp(operation, "unfriendly-sum") == 0) {
    int sum = unfriendly_sum(matrix, rows, cols);
    printf("unfriendly sum: %d\n", sum);
  } else {
    printf("FATAL: unsupported operation!\n");
    exit(1);
  }

  free(matrix);

  return 0;
}
