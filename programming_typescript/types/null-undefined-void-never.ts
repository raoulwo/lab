// The type `undefined` is special since it has only one possible
// value - `undefined`. The same thing goes for `null`.

// Both types are sometimes used interchangeably, there is a subtle
// semantic difference between the two however:
//
// * `undefined` indicates that something hasn't been defined yet
// * `null` indicates the absence of a value
let a = undefined;
let b = null;

// The type `void` is used for functions without any explicit return
// values. (In JS these functions would return `undefined`).
function foo() { }

// The type `never` is the return type of functions that never return.
// This can be because of thrown errors/infinite loops/program exits...

const f0 = () => { throw TypeError(); }
// INFO: Because virtual functions that should be overridden often throw 
// errors in case someone forgot to override them, they still return
// `void` instead of `never` as opposed to function expressions.
// https://stackoverflow.com/questions/40251524/typescript-never-type-inference
function f1() { throw TypeError(); }


// INFO: The `never` type is subtype (bottom type) of all other types,
// this means it can be assigned to all other types. No values of other
// types can be assigned to `never` though.

// INFO: In older TS versions or with `strictNullChecks` set to false
// `null` is also subtype of all other types, meaning you have to check
// all values for being defined or not.

