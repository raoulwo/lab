// Just like you can define variables which act
// as aliases for values you can define aliases for types.
type Age = number;
type Person = {
  name: string,
  age: Age,
};

// Aliases are never inferred, you must always explicitly
// specify them.
let age: Age = 24;

// Since they're only aliases you can still assign them
// to the underlying types.
let num: number = age;

// INFO: Aliases have block scope and can't be declared
// twice in such a scope.

// INFO: Aliases are useful for simplifying up complex types
// and making them more descriptive.
