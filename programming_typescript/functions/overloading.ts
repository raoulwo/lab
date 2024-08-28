type Reservation = { from: Date, to?: Date, destination: string };

// When declaring a set of overload signatures for a function, the
// caller of the function will perceive the function's type to be
// the *union* of the call signatures.
//
// From the implementation standpoint you have to define a single
// combined type that needs to be implemented. This combined call
// signature needs to be specified manually.
type Reserve = {
  // The client of the function will perceive the first two signatures.
  (from: Date, to: Date, destination: string): Reservation
  (from: Date, destination: string): Reservation
  // The implementation's signature is a combination of the signatures above.
  // It isn't visible to clients calling this function.
  (from: Date, toOrDestination: Date | string, destination?: string): Reservation
};

// Each overload signature needs to be assignable to the actual implementation,
// don't be too general when declaring the implementation's signature. Try to
// keep the types as strict/narrow as possible. You have to *narrow* the types
// further either way when you want to use them safely, so you'll make your
// life easier by typing them strictly in the first place.
let reserve: Reserve = (from: Date, toOrDestination: Date | string, destination?: string) => {
  if (toOrDestination instanceof Date && destination !== undefined) {
    return { from, to: toOrDestination, destination };
  } else if (typeof toOrDestination === 'string') {
    return { from, destination: toOrDestination };
  }
  throw new Error('Invalid reservation data');
};

// You can also overload function declarations.
function greet(name: string, greeting: string): string;
function greet(name: string): string;
function greet(nameOrGreeting: string, greeting?: string): string {
  if (greeting !== undefined) {
    return `${greeting}, ${nameOrGreeting}!`;
  }
  return `Hello, ${nameOrGreeting}!`;
}

// Type signatures can also be used to specify any properties functions can have.
// (Remember, functions are also just objects)
type Foo = {
  (bar: string): string
  // `baz` is a property of type `string`
  baz: string
};
