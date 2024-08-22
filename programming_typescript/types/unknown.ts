// `unknown` types are like `any`, they can't be used until
// they are *refined* by checking what they are.

// `unknown` values can be compared using `==`, `===`, `&&`,
// `||`, `?`, negated with `!` and refined using `typeof`, `instanceof`

// TypeScript will never infer `unknown`, you have to use it explicitly
let a: unknown = 5;

// WARNING: Can't use `a` of type `unknown` until refined
// let b = a + 10;

if (typeof a === 'number') {
  let c = a + 10;
}

// NOTE: `unknown` is the supertype of every other type. This
// means you can assign values of every other type to values of type
// `unknown`. To do it the other way around you would first have to
// narrow the types however.
