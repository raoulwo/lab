use std::io::Write;
// Trait objects are used together with generics in Rust for
// writing polymorphic code. They incur the runtime overhead
// of dynamic dispatch (also known as virtual method call).

fn main() {
    // Rust doesn't know the type of trait objects at compile time.
    // That's why Rust has to add additional data for trait objects
    // in order for the dynamic dispatch to the corresponding
    // methods to work.

    // In memory, trait objects are fat pointers (2 machine words)
    // consisting of:
    //
    // 1. A pointer to the value
    // 2. A virtual pointer (vptr) to a table (vtable) representing
    //    the value's type
    //
    // In Rust (and C++) the vtable is created once at compile time
    // and shared by all objects of the same type. The vtable is a
    // private implementation detail of Rust and can't be accessed
    // directly.

    // Rust needs to determine the size of a variable at compile
    // time. That's why you can't define trait objects directly.
    // However, you can explicitly define a reference to one.
    // Alternatively, you can store a trait object in a smart
    // pointer like a `Box`.
    let mut buf: Vec<u8> = vec![];
    let writer: &mut dyn Write = &mut buf;
}
