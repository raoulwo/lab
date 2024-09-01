// If `T` implements `AsRef<T>` you can borrow
// a `&T` from it. When implementing `AsMut<T>`
// you can borrow a `&mut T` instead. Here's
// how they're defined:
trait MyAsRef<T: ?Sized> {
    fn as_ref(&self) -> &T;
}

trait MyAsMut<T: ?Sized> {
    fn as_mut(&mut self) -> &mut T;
}

// NOTE: Think of these traits as implicit type-coercion
// traits when used as function arguments. `AsRef` and
// `AsMut` are **borrowing** type conversions, for
// **consuming** type conversions look at `From` and `Into`.

// For example:
//
// * `Vec<T>` implements `AsRef<[T]>`
// * `String` implements `AsRef<str>` and `AsRef<[u8]>`

// `AsRef` and `AsMut` are typically used to define
// functions that are more flexible in the types of
// arguments they can accept:

use std::path::Path;

fn inflexible(path: &Path) {}
fn flexible<P: AsRef<Path>>(path: P) {}

// Here, `inflexible` can only take in params of type
// `&Path`, `flexible` can on the other hand take in
// all types that you can borrow an immutable `&Path`
// from. This makes it possible to call this function
// with varying types without requiring the client to
// do manual conversion beforehand.
//
// All of Rust's filesystem access functions accept
// paths this way. This resembles function overloading
// in C++, Rust takes a different approach for this
// however.

fn main() {}
