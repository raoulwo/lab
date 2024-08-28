// Here, we define a generic type for a custom events.
type MyEvent<T> = {
  target: T,
  type: string,
};

// When using generic type aliases, the concrete types must
// be bound explicitly, they can't be inferred.
let myEvent: MyEvent<HTMLButtonElement | null> = {
  target: document.querySelector('#myButton'),
  type: 'click',
};

// You can reuse generic types in other type aliases or
// function call signatures:
type TimedEvent<T> = {
  event: MyEvent<T>,
  from: Date,
  to: Date,
};

function triggerEvent<T>(event: MyEvent<T>): void { }

