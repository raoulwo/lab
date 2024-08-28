// NOTE: This article explains generators in TS:
// https://dev.to/gsarciotto/generators-in-typescript-4b37

// Generator functions can be explicitly annotated to return a generic IterableIterator<T>.
function* createFibonacciGenerator(): IterableIterator<number> {
  let a = 0;
  let b = 1;
  while (true) {
    yield a;
    [a, b] = [b, a + b];
  }
}

// The inferred type is `Generator<T, TReturn, TNext>`, this StackOverflow post explains
// the difference between Generator and IterableIterator: https://stackoverflow.com/questions/58568399/why-are-typescripts-iterableiterator-and-generator-generics-slightly-differ
function* createNumbers() {
  let n = 0;
  while (true) {
    yield n++;
  }
}
