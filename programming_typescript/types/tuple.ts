// Tuples are subtypes of arrays and are used to type
// arrays with fixed lengths where the values at given
// index have specific types.

// Since the JS syntax for both arrays and tuples is the
// same and TS always infers arrays when encountering []
// you have to explicitly type them.
let coord: [number, number] = [0, 0];
let user: [number, string] = [1, 'raoulwo'];

// You can define elements to be optional.
let res: ['ok' | 'error', number?] = ['ok', 7];
res = ['error'];
// The type above is equivalent to this union type.
type SameThing = ['ok' | 'error'] | ['ok' | 'error', number];

// Tuples also support rest elements which you can use to define
// tuples with minimum lengths.

// Tuple of at least 1 string.
type Friends = [string, ...string[]];

// Tuple of number, boolean and 0 or more strings.
type Idk = [number, boolean, ...string[]];

let idk: Idk = [0, true];
idk = [0, true, 'foo'];
idk = [0, true, 'foo', 'bar'];

// Similar to arrays, tuples can also be specified as readonly
let right: readonly [number, number] = [1, 0];
let up: Readonly<[number, number]> = [0, 1];

