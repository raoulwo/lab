// `std::marker::Copy` is a *marker trait* which
// extends `Clone`. It's used to mark certain types
// to be a *copy type* where assignment will cause
// a copy of the original to be created instead of it
// being moved.
//
// Rust only allows types to implement `Copy` if only
// a shallow byte-for-byte copy is all that's needed
// for a copy to be created. Types that allocate heap
// buffers/os handles can't be `Copy`.
//
// Types that are `Drop` can't be `Copy` as well since
// Rust assumes that types needing special cleanup
// when dropped also would require special copying code.
// Think carefully before making something `Copy` because
// implicit copies can be expensive.

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = p1;

    assert_eq!(p1, p2);
}
