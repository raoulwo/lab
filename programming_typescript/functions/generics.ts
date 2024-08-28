// Generic type params are placeholder types that are used
// to enforce type-level constraints, they're also known
// as *polymorphic type params*.

// `T` is a generic type parameter of which we don't know
// the type in advance. TS will infer the type of `T` based
// on the types we pass in as arguments. Once TS can infer
// what type `T` is, it substitutes it for every `T`. It
// *parameterizes* the type.
type Filter = {
  // Generic type params are declared within <...>
  <T>(array: T[], f: (item: T) => boolean): T[]
};

let filter: Filter = (array, fn) => {
  let result = [];

  for (let i = 0; i < array.length; i++) {
    if (fn(array[i])) {
      result.push(array[i]);
    }
  }

  return result
};

// `T` => `number`
filter([1, 2, 3], n => n % 2 === 0);
// `T` => `string`
filter(['a', 'b'], s => s !== 'b');

// Generics are powerful since they allow functions to be more
// general than with concrete types. Think of them as *constraints*.
// The generic `T` constraints the types of all params with type `T`
// to the same type wherever `T` shows up.

// NOTE: You can not only use Generics in functions, they can also be used in:
//
// * Classes
// * Interfaces
// * Type Aliases
