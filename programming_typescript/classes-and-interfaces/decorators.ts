// Decorators can be used for annotating classes,
// methods, method params, props, getters/setters
// for metaprogramming. They are just special syntax
// for calling a function on the thing you're decorating.

type Payload = { toString: () => string };
type ClassCon<T> = new (...args: any[]) => T;

function serializable<T extends ClassCon<{
  getValue(): Payload
}>
>(Con: T) {
  return class extends Con {
    serialize() {
      return this.getValue().toString();
    }
  };
}

@serializable
class ApiPayload {
  getValue(): Payload {
    return {
      toString() {
        return 'payload';
      }
    };
  }
}

let payload = new ApiPayload;
// (I don't know if this info is still up-to-date):
// TS doesn't check if a decorator changes the shape of the
// thing it's decorating. It doesn't know if it extended the
// class by adding methods/properties in any way.
let serialized = (payload as any).serialize();
console.log(serialized);

// NOTE: The call works, however TS throws an error if not
// casting to any before the call.
