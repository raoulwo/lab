fn main() {
    // You can define references of references
    struct Point {
        x: i32,
        y: i32,
    }
    let point = Point { x: 1000, y: 750 };

    // Types are specified for clarity, can be inferred
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;

    // The `.` operator can implicitly dereference as many refs as needed
    assert_eq!(rrr.x, 1000);
    assert_eq!(rrr.y, 750);

    // NOTE: In Rust, references are guaranteed to always point towards
    // valid values. This means they can never be null. To indicate that
    // something may be a missing value, use Option<&T> instead.

    // NOTE: References are usually a single machine-word pointer,
    // references to slices and trait objects are *fat pointers* however.
    //
    // * slices: starting address + length (2 machine-words)
    // * trait objects: value's address + pointer to implementation (vtable?)
}
