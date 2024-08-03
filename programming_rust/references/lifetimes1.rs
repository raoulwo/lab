// A `static` is Rust's equivalent of a global variable.
// They're like statics in C, they have a global lifetime,
// their scope doesn't necessarily have to be global, depending
// on where they were defined.
static mut STASH: i32 = 0;

// If we want to write a function taking in an immutable (shared) ref
// to update the static (global) var `STASH`, we need to inform the
// Rust compiler that the reference we pass to the function lives
// long enough for it to be valid, so that `STASH` never contains
// invalid memory. Since `STASH` is static, the lifetime of the ref
// passed to the function also must be `static`.
fn assign(r: &'static i32) {
    // Mutable static (global) vars are unsafe, that's why we use
    // the `unsafe` block here
    unsafe {
        STASH = *r;
    }
}

fn main() {}
