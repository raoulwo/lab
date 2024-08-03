// When defining structs holding refs, these refs must have a lifetime
struct A<'a> {
    val: &'a i32,
}

// The constraints for the struct A are:
//
// * the ref `val` must not outlive its underlying value with lifetime `'a`
// * the ref `val` must live at least as long as the struct

// The same applies for structs holding other structs with lifetimes
struct B<'a> {
    a: A<'a>,
}

fn main() {}
