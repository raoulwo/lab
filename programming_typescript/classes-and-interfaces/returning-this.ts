// Using `this` as a return type you can define
// *fluent* interfaces that leverage method chaining
// while still being type correct when subtyping a class.

// Here without using `this`:
class Set0 {
  has(_value: number): boolean {
    return false;
  }

  add(_value: number): Set0 {
    return this;
  }
}

class SubSet0 extends Set0 {
  // Here, we would need to override the method.
  add(_value: number): SubSet0 {
    return this;
  }
}

class Set1 {
  // Because of return type `this` we don't need
  // to override the method.
  add(_value: number): this {
    return this;
  }
}

class SubSet1 extends Set1 { }
