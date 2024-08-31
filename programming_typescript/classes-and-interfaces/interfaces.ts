// Interfaces like type aliases exist for naming types
// so that they don't need to be defined inline every time.
// They only exist in TS and aren't generated in the transpiled
// JS code.

// Interfaces and type aliases are both used for defining shapes,
// there are some subtle differences however.

// Here, the type alias and interface for sushi are equivalent.
type SushiType0 = {
  calories: number,
  salty: boolean,
  tasty: boolean,
};

interface SushiInterface0 {
  calories: number;
  salty: boolean;
  tasty: boolean;
}

// It gets more interesting when composing types/interfaces
type CakeType0 = {
  calories: number,
  sweet: boolean,
  tasty: boolean,
};

type FoodType = {
  calories: number,
  tasty: boolean,
};

type SushiType1 = FoodType & {
  salty: boolean,
};

type CakeType1 = FoodType & {
  sweet: boolean,
};

interface FoodInterface {
  calories: number;
  tasty: boolean;
}

// NOTE: Interfaces can also extend other shapes:
//
// * Type aliases,
// * Classes,
// * Other interfaces
//
interface SushiInterface1 extends FoodInterface {
  salty: boolean;
}

interface CakeInterface extends FoodInterface {
  sweet: boolean;
}

// Differences between types and interfaces:

// 1. Type aliases are more general, their RHS can be any type,
// the RHS of interfaces **must** be a shape.
type A = number;
type B = A | string;
// ^ not possible with interfaces.

// 2. When extending interfaces TSa assures that the interface
// you're extending doesn't conflict with the extension.
interface SuperInterface {
  good(x: number): string;
  bad(x: number): string;
};

interface SubInterface extends SuperInterface {
  good(x: string | number): string;
  // Here, `(string) => string` conflicts with `(number) => string`
  bad(x: string): string;
};

type FooType = {
  foo(x: number): string;
};

// Here, we don't get a compile-time error, instead TS combines the
// types resulting in an overload function call signature
type BarType = FooType & {
  foo(x: string): string;
};

// 3. Multiple interfaces with the same name in the same scope are merged,
// multiple type aliases in the same scope throw compile-time errors.
