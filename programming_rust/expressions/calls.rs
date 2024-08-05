fn main() {
    // The `.` operator can implicitly convert refs/values to their
    // counterpart when calling a method
    let mut v = vec![6, 2, 3, 8, 4, 1];

    // `sort` expects a `&mut`, the conversion is done implicitly
    v.sort();

    // The same applies for smart pointer types
    let mut bv = Box::new(vec![2, 0, 1]);
    bv.sort();

    // similar to static methods, rust supports *type-associated functions*
    let foo = String::from("foo");

    // calling generic type associated functions has a weird syntax,
    // the following is not allowed since Rust interprets the `<`
    // as the less than operator
    // let v = Vec<i32>::with_capacity(50);

    // the *turbofish* syntax ::<T> is used here instead
    let v = Vec::<i32>::with_capacity(30);
}
