// TS will bind generics to the actual types at different
// times, depending on where the generics are declared.

// When declaring a generic as part of a call signature,
// TS will bind a concrete type to `T` at call time.
type Filter0 = {
  // Here, `T` is scoped to this single call signature
  <T>(array: T[], fn: (item: T) => boolean): T[]
};

let filter0: Filter0 = (array, fn) => {
  let result = [];

  for (let i = 0; i < array.length; ++i) {
    if (fn(array[i])) {
      result.push(array[i]);
    }
  }

  return result;
};

// If the generic type is instead scoped to the type alias,
// TS requires us to bind a concrete type at the time of
// implementation.
type Filter1<T> = { // Here, `T` is scoped to all call signatures
  (array: T[], fn: (item: T) => boolean): T[]
};

let numberFilter: Filter1<number> = (array, fn) => {
  let result = []

  for (let i = 0; i < array.length; ++i) {
    if (fn(array[i])) {
      result.push(array[i]);
    }
  }

  return result;
};
type StringFilter = Filter1<string>;

// Generally, TS will bind a concrete type to a generic when
// you use the generic:
//
// * (RUNTIME) Functions/Classes => When they are called/instantiated
// * (COMPTIME) Type Aliases, Interfaces => When used/implemented

function map<TInput, TOutput>(array: TInput[], fn: (item: TInput) => TOutput): TOutput[] {
  let result = [];

  for (let i = 0; i < array.length; ++i) {
    result.push(fn(array[i]));
  }

  return result;
}

// TS usually does a good job at inferring generic types, you can optionally explicitly
// annotate the concrete types. Sometimes you have to explictly annotate the types
// in certain scenarios where TS can't infer them alone.

let even = map([1, 2, 3], (num) => num % 2 === 0);
let odd = map<number, boolean>([1, 2, 3], (num) => num % 2 === 1);
