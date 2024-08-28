// Terminology:
//
// * Iterable: Object that implements the [Symbol.iterator] property.
//     This property is a function that returns an `Iterator`
// * Iterator: An object implementing the method `next` which returns
//     an object with the properties `value` and `done`
//
// When an object implements both `[Symbol.iterator]` and a `next`
// method, then it is a *iterable iterator*.

// The type is `Generator<number, void, unknown>` which is similar
// to but not the same as `IterableIterator<number>`
let numbers = {
  *[Symbol.iterator]() {
    for (let n = 1; n <= 10; n++) {
      yield n;
    }
  }
};
