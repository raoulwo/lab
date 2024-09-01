// The type `std::borrow::Cow` (Cow -> "Clone on write")
// can be used in scenarios where you don't know whether
// a certain type should be a borrowed ref or an owned
// value. It's defined like this:
use std::borrow::ToOwned;

enum Cow<'a, B: ?Sized>
where
    B: ToOwned,
{
    // Here, `Cow` holds a shared ref to `B` which itself
    // implements `ToOwned` (meaning it can be converted into
    // an owned copy if needed)
    Borrowed(&'a B),
    // Here, `Cow` holds an owned copy of the type `Owned`
    // which is the associated type of the `ToOwned` trait.
    Owned(<B as ToOwned>::Owned),
}

// NOTE: `Cow` implements `Deref`, so no matter what the current
// state of `Cow` is, you can access the underlying value transparently:
//
// * If `Cow` is a shared ref, then it just exposes this ref.
// * IF `Cow` is owned, then it shared a ref to the owned value.

// NOTE: You can borrow a mutable ref from `Cow` using its `to_mut` method:
//
// * If `Cow` is a shared ref, `to_mut` returns the result of `to_owned`
// * If `Cow` is owned, then it borrows mutably from the already owned value.

// A common use case for `Cow` is to hold either statically allocated string
// constants (which are `&'static str`) or strings holding computed values
// (which are `String`). Using `Cow` makes it possible for consumers of its
// API to put off memory allocation until it becomes necessary.

fn main() {}
