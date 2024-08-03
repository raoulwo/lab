// Often times, you want to define a function taking in a ref
// of a certain data structure that returns a ref to a part
// of it. Here an example
fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}

// The lifetimes of this function are implicit, the lifetime
// of the returning ref must be the same as the passed ref.
// It would be possible to write out the lifetimes explicitly
fn smallest_with_lifetimes<'a>(v: &'a [i32]) -> &'a i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}

fn main() {
    let s;
    {
        let parabola = [9, 4, 1, 0, 4, 9];
        // The lifetime of `s` must be the same as `&parabola`
        s = smallest(&parabola);
    } // <-- parabola dropped here

    // Here, we use `s` outside of its guaranteed lifetime -> Error
    assert_eq!(*s, 0);
}
