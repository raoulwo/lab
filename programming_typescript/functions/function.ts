// Function params should be annotated explicitly,
// the return type is usually inferred.
function foo(a: number, b: number) {
  return a + b;
}

function bar(a: number, b: number): number {
  return a + b;
}
