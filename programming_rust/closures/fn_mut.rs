// `FnMut` is a closure that contains mutable values or refs.
// Rust considers non-mut values to be safe to share across
// threads, this is not the case for non-mut closures though
// since race conditions could occur should multiple threads
// try to call this closure at the same time causing them
// to read/write the same data concurrently. Any closure
// that requires mutable access to a value *but* doesn't drop
// it is an `FnMut`:
fn main() {
    let mut i = 0;
    let mut incr = || {
        i += 1;
        println!("i = {}", i);
    };

    incr();
    incr();
    incr();
}
