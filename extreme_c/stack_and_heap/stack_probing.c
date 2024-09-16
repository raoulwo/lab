// The stack segment is a private region of memory
// that only the owning process can read and modify.
// To probe it, we can use the GNU debugger `gdb`.

// NOTE: To debug a source file, we need to compile
// it with debug symbols enabled. This is done by
// passing the option `-g` to the compiler.

int main(int argc, char **argv) {
  char arr[4];
  arr[0] = 'A';
  arr[1] = 'B';
  arr[2] = 'C';
  arr[3] = 'D';

  return 0;
}
