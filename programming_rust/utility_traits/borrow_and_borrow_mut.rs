// `std::borrow::Borrow` is similar to `AsRef`, it's
// more restrictive however. A type `T` **should only**
// implement this `Borrow<U>` when a `&U` hashes and
// compares exactly like the original `T`. (Rust doesn't
// enforce this, it's the intent of the trait however)

// The hashing and comparing guarantees make `Borrow`
// valuable for keys in hash tables and comparing hashes
// for other reasons.

// The definition of `Borrow` looks like this (identical to `AsRef`):
trait Borrow<Borrowed: ?Sized> {
    fn borrow(&self) -> &Borrowed;
}

fn main() {}
