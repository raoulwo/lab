// It is allowed to declare a variable in one place
// and assign it later. TypeScript will make sure it
// is *definitely assigned* by the time you use it.
let a: { b: number };
a = { b: 0 };
