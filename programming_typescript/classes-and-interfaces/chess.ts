type Color = 'Black' | 'White';
type File = 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H';
type Rank = 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8;

// Possible access modifiers: private, protected, public

class Game {
  // Call of static method.
  private pieces = Game.makePieces();

  // Static method definition
  private static makePieces() {
    return [
      new King('White', 'E', 1),
      new King('Black', 'E', 8),
      // Add other pieces here ...
    ];
  }
}

// The class is *abstract* -> it can't be instantiated.
abstract class Piece {
  // No assignment during declaration, we have to provide
  // a value for `position` inside the constructor else
  // it isn't *definitely assigned*. That means we specified
  // it to be of type `T`, however it actually is `T | undefined`
  protected position: Position;

  constructor(
    // Instance properties can be specified as `readonly`
    private readonly color: Color,
    file: File,
    rank: Rank,
  ) {
    this.position = new Position(file, rank);
  }

  // Instance method implementation
  moveTo(position: Position) { // (default access -> public)
    this.position = position;
  }

  // Abstract method declaration, subclasses need to implement
  // this.
  abstract canMoveTo(position: Position): boolean;
}

class Position {
  constructor(
    private file: File,
    private rank: Rank,
  ) { }

  distanceFrom(position: Position) {
    return {
      rank: Math.abs(position.rank - this.rank),
      file: Math.abs(position.file.charCodeAt(0) - position.file.charCodeAt(0))
    };
  }
}

// Inherit from a super class using `extends`
class King extends Piece {
  canMoveTo(position: Position) {
    let distance = this.position.distanceFrom(position);
    return distance.rank < 2 && distance.file < 2;
  }
}

class Queen extends Piece {
  canMoveTo(position: Position): boolean {
    return false;
  }
}
class Bishop extends Piece {
  canMoveTo(position: Position): boolean {
    return false;
  }
}
class Knight extends Piece {
  canMoveTo(position: Position): boolean {
    return false;
  }
}
class Rook extends Piece {
  canMoveTo(position: Position): boolean {
    return false;
  }
}
class Pawn extends Piece {
  canMoveTo(position: Position): boolean {
    return false;
  }
}

