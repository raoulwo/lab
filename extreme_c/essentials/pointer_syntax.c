int main(int argc, char **argv) {
  // Pointers are just variables that hold a *memory address*.
  // They are declared using the `*` character.
  int num = 100;
  // NOTE: Assigning `0` to a pointer is equivalent to setting
  // it to the value `NULL` since `NULL` is a macro defined as 0.
  // After setting it to `NULL` a pointer is considered to be a
  // *null pointer* (C++ uses the keyword `nullptr` instead).
  int *ptr = 0;
  // The *reference* (or *address*) operator `&` is used to
  // access the memory address of a variable.
  ptr = &num;
  // The *dereference* operator `*` is used to access the
  // underlying value of a pointer (or memory address).
  *ptr = 200;

  return 0;
}

// WARNING: Null pointers aren't pointing to valid memory addresses.
// Dereferencing null pointers would lead to undefined behaviour and
// should therefore be avoided.
