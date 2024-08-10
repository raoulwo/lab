use std::io::Write;

// NOTE: Rust generates separate machine code for generic
// functions, inferring the types they are invoked with.
// This process is called *monomorphization*.

// NOTE: You can also define functions with generic
// lifetime params. Depending on the type of lifetimes
// they are called with, Rust can compile multiple versions
// of the generic function.

// NOTE: You can also define functions including generic
// constant parameters.

// NOTE: Not only functions can be generic in Rust, there
// also exists:
//
// * Generic structs (Vec, HashTable ...)
// * Generic enums (Result, Option ...)
// * Individual methods on a non-generic type
// * Type-aliases
//
// Generic params, lifetime params, constant params, bounds,
// where clauses... can be used on all generic items in Rust
// not just functions.

// Here, `W` is a generic type parameter with a *bound*.
// The bound specifies that `W` must implement the `Write`
// trait.
fn say_hello<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"Hello, world!\n")?;
    out.flush()
}

use std::fmt::Debug;
use std::hash::Hash;

// This is an example of a generic function where the generic
// type parameter must implement multiple traits to fulfill
// the bounds.
fn foo<T: Debug + Hash + Eq>(values: &Vec<T>) {}

// There exists an alternative syntax for long trait bounds.
fn bar<T>(values: &Vec<T>)
where
    T: Debug + Hash + Eq,
{
}

fn main() {}
