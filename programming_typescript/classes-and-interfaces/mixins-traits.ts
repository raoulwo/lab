// TS doesn't support mixins/traits natively, however
// it is possible to implement that behavior ourselves.

class User {
  constructor(
    public id: string,
    public name: string,
  ) { }
  getDebugValue() {
    return {
      id: this.id,
      name: this.name,
    };
  }
}

// Type alias that represents any constructor function
type ClassConstructor<T> = new (...args: any[]) => T

function withDebug<C extends ClassConstructor<{
  getDebugValue(): object
}>>(Class: C) {
  return class extends Class {
    debug() {
      let Name = Class.constructor.name;
      let value = this.getDebugValue();
      return Name + '(' + JSON.stringify(value) + ')';
    }
  };
}

let DebuggableUser = withDebug(User);
let user = new DebuggableUser("1", "raoulwo");
console.log(user.debug());
