fn f<'a>(p: &'a i32) {
    // Stuff here ...
}

fn g(p: &'static i32) {
    // Stuff here ...
}

fn main() {
    {
        // `x` is a local var
        let x = 10;

        // The signature of `f` guarantees that Rust won't store the value of `x`
        // in any ref that will outlive `x` itself (which would be illegal)
        f(&x);

        // This doesn't work, since the signature of `g` assumes the lifetime of
        // the ref passed as an argument to be `static`. The ref might outlive `x`
        // itself which isn't legal
        g(&x); // Error
    }
}
