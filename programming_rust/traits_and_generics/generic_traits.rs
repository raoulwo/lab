// This generic trait resembles the `std::ops::Mul` trait
// used to overload the `*` operator used for multiplication.
//
// Since it is generic, this trait can be implemented multiple
// times by the same type so that multiplication with various
// RHS types is possible.
trait Mul<RHS = Self> {
    //          ^ `Self` is default
    type Output;

    fn mul(self, rhs: RHS) -> Self::Output;
}

fn main() {}

// NOTE: The orphan rule specifies that you can't define a trait
// on a type if both trait and type are foreign to your crate.
// You must either:
//
// 1. Implement a foreign trait on one of your types
// 2. Implement your trait on a foreign type
//
// Generic traits are an exception to this rule. As long as the
// generic type you're implementing it for is one of yours, then
// both the trait and the type can be foreign.
//
// You could for example implement the `Mul` trait on the `f64`
// type where the `RHS` is a new type defined by you.
