fn main() {
    // NOTE: Rust tries to assign a *lifetime* to each reference type
    // which defines a portion of the program for which the ref can
    // be used safely (the underlying value definitely exists).
    // The concept of lifetimes only applies during **compile-time**,
    // they have no relevance for the runtime of a program.

    // You can't borrow references to local variables
    // and use that reference out of scope (lifetime constraint)

    {
        let r;
        {
            // <-- START `r` should only be used in here
            let x = 1;
            r = &x;
            // <-- END
        }
        assert_eq!(*r, 1); // Can't do this
    }
    // A ref can't outlive (have a greater lifetime) than the underlying value.
    // The variable's lifetime must *enclose* (or contain) the refs lifetime.
}
