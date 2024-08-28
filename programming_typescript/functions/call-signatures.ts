// Call signatures (or type signatures) specify a function's type.
// They only contain *type-level* code, code that only consists
// of types and type operators. Rule of thumb:
//
// * JS code -> value-level
// * TS but not valid JS -> type-level

// You can (but don't have to) pull out call signatures to their own
// type aliases.
type Greet = (name: string) => string;
type Log = (message: string, userId?: number) => void
type SumVariadic = (...numbers: number[]) => number

// NOTE: The call signatures above are **shorthand** call signatures.
// The call signatures can be written out more explicitly:
type Greet1 = {
  (name: string): string
};
type Log1 = {
  (message: string, userId?: number): void
};
type SumVariadic1 = {
  (...numbers: number[]): number
};

// These full type signatures are important when *overloading functions*.

// When defining an implementation for a type signature, you don't
// have to annotate parameter or return types again since they are
// already part of the call signature:
let greet: Greet = (name) => {
  return `Hello, ${name}!`;
};

// The characteristic that types don't have to be annotated again
// is called *contextual typing*, since TS can infer them from the
// context they're used in.

// Contextual typing most often comes up when working with callback
// functions:
function times(fn: (index: number) => void, n: number) {
  for (let i = 0; i < n; i++) {
    fn(i);
  }
}

// Here, the type of `i` is inferred from the context it's used within.
// This only works because we defined the callback inline and not as
// a standalone function (else we would have to specify types)
times((i) => console.log(i), 100);

