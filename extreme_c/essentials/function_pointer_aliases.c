#include <stdio.h>

// Since the types of function pointers can
// become quite verbose, it's often a good
// idea to define *type aliases* for them:

typedef int bool_t;
typedef bool_t (*less_than_func_t)(int, int);

// NOTE: The suffix `_t` is used by convention
// to denote user defined type aliases. You can
// also find them in standard types:
//
// * `size_t`
// * `time_t`

bool_t less_than(int a, int b) { return a < b ? 1 : 0; }
bool_t less_than_modular(int a, int b) { return (a % 5) < (b % 5) ? 1 : 0; }

int main(int argc, char **argv) {
  less_than_func_t func_ptr = NULL;

  func_ptr = &less_than;
  bool_t res = func_ptr(3, 7);
  printf("%d\n", res);

  func_ptr = &less_than_modular;
  res = func_ptr(3, 7);
  printf("%d\n", res);

  return 0;
}
