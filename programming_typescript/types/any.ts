// `any` is the set of *all* values, you can do *anything* with `any`.
// Only use it as a last resort, avoid it as much as possible.
let a: any = 666;
let b: any = ['danger'];
let c = a + b; // Doesn't error

// NOTE: The `noImplicitAny` flag leads to errors should the `any`
// type be inferred implicitly (part of 'strict' flags).
