// Traits must be brought into scope in order to use
// their methods, associated functions...
use std::io::Write;

// NOTE: Some methods from traits like `Clone` or `Iterator`
// work without an explicit `use` since they're in scope
// by default being part of the standard prelude.

fn main() {
    // `Vec<u8>` implements the `Write` trait.
    let mut buf: Vec<u8> = vec![];
    let _ = buf.write_all(b"hello");
}
