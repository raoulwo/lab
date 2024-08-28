// Bounded polymorphism allows us to define certain bounds
// for generics which establish further constraints for
// the possible types that satisfy the generics.

type TreeNode = {
  value: string
};

type LeafNode = TreeNode & {
  isLeaf: true
};

type InnerNode = TreeNode & {
  children: [TreeNode] | [TreeNode, TreeNode]
};

// This function has a generic type param `T` with an *upper bound* of `TreeNode`.
// This means `T` can either be of type `TreeNode` or a *subtype* of `TreeNode`.
function mapNode<T extends TreeNode>(node: T, fn: (value: string) => string): T {
  return { ...node, value: fn(node.value) }
}

// NOTE: Had we omitted the upper bound `extends TreeNode`, TS would've complained
// about the member access `node.value`.

// (Raoul) I'm guessing *subtype* in terms of bounded polymorphism relates to the
// set-theoretic relationship between subtype B and supertype A. B is subtype of A
// if all members of B are also members of A. B is subset of A.

// Here an example of how bounded polymorphism with multiple constraints comes
// into play.
type HasSides = { numberOfSides: number };
type SidesHaveLength = { sideLength: number };

function calcPerimeter<Shape extends HasSides & SidesHaveLength>(s: Shape): number {
  return s.numberOfSides * s.sideLength;
}

type Square = HasSides & SidesHaveLength;
let square: Square = { numberOfSides: 4, sideLength: 3 };
let perimeter = calcPerimeter(square);

// Bounded polymorphism also comes into play when modelling variadic functions.
function call<T extends unknown[], R>(fn: (...args: T) => R, ...args: T): R {
  return fn(...args);
}

function fill(length: number, value: string): string[] {
  return Array.from({ length }, () => value);
}

call(fill, 10, 'foo');
