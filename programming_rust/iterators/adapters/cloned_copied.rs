// The `cloned` adapter takes an iterator that produces
// refs and transforms it into one that instead produces
// cloned values (like `iter.map(|val| val.clone())`).
// The corresponding item type must be `Clone` to work.

// The `copied` adapter is like `cloned` just that the items
// are copied (like `iter.map(|val| *val)`). The items must
// be `Copy` (and therefore also `Clone`) for that to work.
// This means `copied` is more restrictive and less general
// than `cloned`.
fn main() {
    let a = [1, 2, 3];

    // `iter` -> `&i32`
    assert_eq!(Some(&1), a.iter().next());
    // `cloned` -> `i32` (must be `Clone`)
    assert_eq!(Some(1), a.iter().cloned().next());
    // `copied` -> `i32` (must be `Copy`)
    assert_eq!(Some(1), a.iter().copied().next());
}
