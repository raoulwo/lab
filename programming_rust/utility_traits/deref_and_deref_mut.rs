// You can specify how dereferencing operators like
// `*` and `.` behave on your types by implementing
// the following traits:
//
// * `std::ops::Deref`
// * `std::ops::DerefMut`
//
// If the context requires a mutable reference then
// Rust uses the `DerefMut` trait, when readonly
// access is enough Rust uses `Deref`.
//
// Rust's smart pointer types for example implement
// these traits so that they behave like non-pointers.

// Here are the definitions:
trait MyDeref {
    type Target: ?Sized; // Something that the implementor owns, contains...
    fn deref(&self) -> &Self::Target;
}

trait MyDerefMut: Deref {
    fn deref_mut(&mut self) -> &mut Self::Target;
}

// Rust uses these traits for *deref coercions*.
// Whenever Rust encounters a type-mismatch between
// `&Self` and `&Self::Target` it calls the corresponding
// method implicitly (same for `DerefMut`). Rust will
// apply deref coercion multiple times in succession
// if necessary, e.g.:
//
// &Rc<String> -> &String -> &str

// NOTE: This is how you most operations on strings
// are actually implemented for string slices `&str`
// because the type `String` implements `Deref<Target = str>`.
// The strings are simply coerced implicitly into string slices.

struct Cursor<T> {
    /// Elements available in this `Cursor`.
    elements: Vec<T>,

    /// The index of the "current" element in `elements`. A `Cursor`
    /// behaves like a pointer to the current element.
    current: usize,
}

use std::ops::{Deref, DerefMut};

impl<T> Deref for Cursor<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Cursor<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements[self.current]
    }
}

fn main() {
    let mut cursor = Cursor {
        elements: vec!['a', 'b', 'c'],
        current: 0,
    };

    // `Cursor` is `Deref` so we can dereference it using `*`.
    assert_eq!('a', *cursor);

    // `is_alphabetic` is a method on type `Char` because of
    // deref coercion, we can call the method directly.
    assert!(cursor.is_alphabetic());

    // `Cursor` is `DerefMut` so we can also mutate its
    // underlying elements.
    *cursor = 'A';
    assert_eq!('A', *cursor);
}

// NOTE: Rust doesn't deref coerce args in function calls to
// check whether the associated target type satisfies certain
// trait bounds for generic type params.
