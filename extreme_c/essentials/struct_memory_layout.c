#include <stdio.h>

// When working with structs, you should always
// take its memory layout into consideration.
// Having structs with bad memory layouts can
// cause performance degredationsin certain
// computer architectures.

// NOTE: The code we write is transformed into
// machine instructions. By knowing the memory
// layout of our data, we can better understand
// what the CPU is doing and adjust our code
// to achieve better results (be that runtime
// performance, memory efficiency...).

// The memory layout of structs is generally
// very similar to that of arrays: In arrays,
// all elements are adjacent to each other in
// memory, the same applies to structs and its
// fields.
//
// There is a difference between structs and
// arrays however: Arrays hold elements of the
// same type, this means that the size of arrays
// can easily be calculated:
//
//   sizeof(element) * count(elements)
//
// The same formula **doesn't** necessarily apply
// to structs, their sizes **aren't** just the
// sums of the sizes of their individual fields.
// That's because **memory alignment** plays a
// big role in determining the size a struct has.
//
// The CPU performs computations and performs
// read/write operations on memory. The CPU
// performs these memory operations on a certain
// number of bytes at a time. This number of bytes
// is called a *word*, an atomic unit. The size
// of words in bytes is architecture-dependent
// (Usually 2 or 4 bytes in 64-bit architectures).
//
// A variable is said to be **aligned** in memory
// if its starting byte is at the beginning of a
// *word*. The CPU can load values in an optimized
// number of memory accesses (which are *very* slow
// compared to cache hits) when data is aligned
// correctly.

typedef struct {
  char first;
  char second;
  char third;
  short fourth;
} sample_t;

// NOTE: We simultaneously define a struct and
// give it a type alias of `sample_t` so we
// don't have to write `struct sample_t`.

// If we assume a word size of 4 bytes for `sample_t`
// and disregard alignment, the memory layout would
// look like the following:
//
//      WORD        WORD
//  ----------- -----------
//  fi se th fo fo xx xx xx
//
// To load the value of `fourth`, the CPU would need
// two memory accesses (because it's unaligned across
// two different words) and some bit-shifting.
//
// In reality, compilers use *padding bytes* to align
// values in memory for more performent access by the
// CPU. This is how it would look in memory:
//
//      WORD        WORD
//  ----------- -----------
//  fi se th 00 fo fo xx xx
//

// NOTE: It's possible to turn off padding for memory
// alignment. That way, structs become *packed*. Packed
// structs aren't aligned and may lead to binary
// incompatibilities and performance degradation.
// Packed structs are usually only used in memory-
// constrained environments, their use can come with
// a huge performance-hit however. Only modern CPUs
// are able to read unaligned values from multiple
// words without extra overhead.

typedef struct __attribute__((__packed__)) {
  char first;
  char second;
  char third;
  short fourth;
} sample_packed_t;

void print_size(sample_t *var) {
  printf("aligned size: %lu bytes\n", sizeof(*var));
}
void print_bytes(sample_t *var) {
  unsigned char *ptr = (unsigned char *)var;
  for (int i = 0; i < sizeof(*var); ++i, ++ptr) {
    printf("%d ", *ptr);
  }
  printf("\n");
}

void print_packed_size(sample_packed_t *var) {
  printf("packed size: %lu bytes\n", sizeof(*var));
}
void print_packed_bytes(sample_packed_t *var) {
  unsigned char *ptr = (unsigned char *)var;
  for (int i = 0; i < sizeof(*var); ++i, ++ptr) {
    printf("%d ", *ptr);
  }
  printf("\n");
}

int main(int argc, char **argv) {
  sample_t aligned = {'A', 'B', 'C', 765};
  print_size(&aligned);
  print_bytes(&aligned);

  sample_packed_t packed = {'A', 'B', 'C', 765};
  print_packed_size(&packed);
  print_packed_bytes(&packed);

  return 0;
}
