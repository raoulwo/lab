// TS doesn't have a `final` keyword to prevent
// you from subclassing a given class, you can
// however make the constructor of a class private
// which achieves the same thing.

class NonFinal {

}

class Final extends NonFinal {
  private constructor() {
    super();
  }
}

// Doesn't work!
let final = new Final;
