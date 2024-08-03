fn main() {
    // Rc/Arc are Rust's reference-counted pointer types
    //
    // Rc stands for reference count (not thread-safe)
    // Arc stands for atomic reference count (thread-safe)

    // Rc<T> values are pointers to heap-allocated memory
    // together with a (strong) reference count. Cloning
    // an Rc<T> doesn't create a copy of the heap-allocated
    // memory, instead it creates another pointer to it
    // and increments the reference count

    // Types can actually be inferred, written out for clarity
    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();

    // NOTE: Values owned by an Rc<T> are *always immutable*,
    // values can't be shared and mutable at the same time

    // NOTE: A problem with reference-counted pointers can be
    // closed loops which can lead to memory leaks, even in Rust.
    // Achieving such memory leaks is difficult to do, however
    // is possible with *interior mutability* (see RefCell<T>).
    // Rust provides *weak pointers* for preventing such cycles.
}
