// Object types specify the shape of objects. This characteristic
// is called *structural typing* or *duck typing*.

// The `object` type is a little narrower than `any`, however only
// specifies that the value is an object and does provide any
// information about what properties are part of the object.
let a: object = {
  prop: 'x'
};

// If inferred, the object is specified using *object literal syntax*
// (which is not the same as *type literals*).
let b = {
  prop: 'x'
};

// INFO: When defining a constant object, the property will still be
// inferred as the corresponding type and not the *type literal* since
// JS objects are still mutable.
const c = {
  prop: 'x'
};

// The object literal syntax describes the shape/structure of an object
// or a class instance.
class Person {
  constructor(
    public firstName: string,
    public lastName: string,
  ) { }
}

let p: { firstName: string, lastName: string } = {
  firstName: 'john',
  lastName: 'doe'
};
p = new Person('jane', 'doe');

// Objects can have optional properties. If a property is optional
// it can either be omitted or have the value `undefined`.
let d: { x: number, y?: number } = {
  x: 0,
  y: 0,
};
d = { x: 0, y: undefined };
d = { x: 0 };

// Object properties can be defined as read-only.
let e: { readonly prop: string } = {
  prop: 'read'
};

// A special case for object literal types is the empty object `{}`,
// every type except for `null` and `undefined` is assignable to values
// of the "empty object type". Avoid this as much as possible.
let f: {} = 5;
f = 'foo';
f = true;

// Lastly, there is the `Object` type which is really similar to the
// empty object type and should also be avoided.
let g: Object = 5;
g = 'bar';
g = false;

