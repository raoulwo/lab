// Use rest params instead of the `arguments` array-like
// for variadic functions, that way we're type-safe.
function sum(...numbers: number[]) {
  return numbers.reduce((sum, num) => sum + num, 0);
}
