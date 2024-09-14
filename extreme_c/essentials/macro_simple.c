// The `#define` directive is used to define a macro
// with given name and value. Each occurence of `NUM`
// will be substituted by the value `3` through a step
// of the preprocessing phase called *macro expansion*.
#define NUM 3

// NOTE: You can use `#undef` to *undefine* a previously
// defined macro.

int main(int argc, char **argv) {
  int x = 2;
  int y = NUM;
  int z = x + y;

  return 0;
}
