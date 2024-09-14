
// Structs are used for defining UDTs (user-defined
// types). These new types encapsulate other types,
// producing higher level abstractions that are
// easier to reason about when it comes to business
// logic.

// NOTE: Historically speaking, the introduction of
// structs was a first step towards *encapsulation*
// known in todays programming languges.

struct color_t {
  unsigned char red;
  unsigned char green;
  unsigned char blue;
};

// NOTE: Again, we use the convention of the suffix
// `_t` to denote UDTs.

int main(int argc, char **argv) {
  struct color_t white = {255, 255, 255};

  return 0;
};
