// Here's how to implement an interface in TS:
interface Animal {
  readonly name: string;
  eat(food: string): void;
  sleep(hours: number): void;
}

class Cat implements Animal {
  constructor(readonly name: string) { }

  eat(food: string): void {
    console.log(`currently eating ${food}`);
  }
  sleep(hours: number): void {
    console.log(`sleeping for ${hours} hours`);
  }
}

// NOTE: Interfaces can declare instance properties
// besides methods, they can't declare access mods
// or static props/methods. You can declare `readonly`
// props however.

// NOTE: Classes can implement multiple interfaces.
// (They are the mechanism for multiple inheritance)
