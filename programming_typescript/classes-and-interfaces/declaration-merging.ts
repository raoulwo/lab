// When defining multiple interfaces with the same
// name in the same scope, they are combined into
// a single interface -> *declaration merging*.

interface User {
  name: string;
}

interface User {
  age: number;
}

let u: User = {
  name: 'raoulwo',
  age: 24,
};

// Doing the same with type aliases would lead to an error.
type UserType = { name: string };
type UserType = { age: number };

// When interfaces conflict, they can't be combined TS
// throws a compile-time error.
interface Confict {
  foo: string;
}

interface Confict {
  foo: number;
}

// When interfaces are generic, the generic type params
// need to be exactly the same (bounds and name).
interface Generic<T extends number> {
  foo: T;
}

interface Generic<T extends string> {
  foo: T;
}
