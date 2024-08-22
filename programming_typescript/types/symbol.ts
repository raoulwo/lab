// When creating a new symbol with a given name, this symbol
// will then be unique and doesn't equal any other symbol
// using either == or === even if they have the same name.

// Variables are inferred to be `symbol`, constants are of
// type `unique symbol`. Like with other primitive types,
// you can explicitly define constants to be `unique symbol`,
// variables can't be of type `unique symbol` however.

let a = Symbol('a');
const b = Symbol('b');
const c: unique symbol = Symbol('c');
