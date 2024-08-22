// Arrays can be specified using T[] or Array<T>.

let a: number[] = [1, 2, 3];
let b: Array<number> = [1, 2, 3];

// Array of union type (try to avoid doing this).
let c: (number | string)[] = [1, 'foo', 3];

// An empty array is a special case, it is inferred to
// be of type `any[]` until you add a first element.
let arr = [];
arr.push(true); // Now `boolean[]`

// You can define arrays to be read-only making them immutable.
// You can use one of the following types:
let foo: readonly number[] = [1, 2, 3];
let bar: ReadonlyArray<number> = [1, 2, 3];
let baz: Readonly<number[]> = [1, 2, 3];

