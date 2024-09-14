// This macro acts like a function and accepts arguments.
// During the preprocessing phase, each occurence is of
// `ADD` is substituted (inlined). It is **not** a function,
// this means it doesn't interact with the call stack at all.
#define ADD(a, b) a + b

// NOTE: After the preprocessing phase (compilation, linking...)
// macros **don't** exist anymore. Technically, the compiler
// doesn't know anything about macros.

int main(int argc, char **argv) {
  int x = 2;
  int y = 3;
  int z = ADD(x, y);

  return 0;
}
