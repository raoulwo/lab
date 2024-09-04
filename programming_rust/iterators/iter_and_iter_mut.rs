// Many collection types provide the convenience methods
// `iter` and `iter_mut` which return an iterator over
// shared refs (`iter`) or mutable refs (`iter_mut`)
// whereas `into_iter` be default creates an iterator
// over the owned values of the `IntoIterator`.

// NOTE: `iter` and `iter_mut` only exist for ergonomics,
// they aren't traits therefore can't be used in bounds
// and only `into_iter` is what "drives" for loops.

fn main() {
    // You can either explicitly borrow a shared/mutable
    // ref from an `IntoIterator` before calling `into_iter`
    // or call the convenience methods.
    let mut v = vec!["foo", "bar", "baz"];

    // The following two ways of getting an iterator over shared
    // refs are equivalent:
    let shared_iter1 = (&v).into_iter();
    let shared_iter2 = v.iter();

    // Same thing goes for exclusive refs:
    let exclusive_iter1 = (&mut v).into_iter();
    let exclusive_iter2 = v.iter_mut();

    // NOTE: The behavior above of `into_iter` returning a shared ref
    // or mutable ref depending on whether you call it using `(&v).`
    // or `(&mut v).` exists because collections provide several
    // implementations of `IntoIterator`: For `(&T)`, for `(&mut T)`
    // and for `(T)`. This is the reason for the following for loop
    // idioms:

    // (1) Loop over shared refs => (&v).into_iter, iter:
    for item in &v {}

    // (2) Loop over mutable refs => (&mut v).into_iter, iter_mut:
    for item in &mut v {}

    // (3) Loop over owned values => v.into_iter:
    for item in v {}

    // NOTE: Not all collection types provide all three implementations
    // of `IntoIterator`:
    //
    // * `HashSet`, `BTreeSet` and `BinaryHeap` don't support `IntoIterator`
    // for mutable refs since mutation of their elements could destroy constraints
    // provided by the data structure.
    //
    // * `HashMap`, `BTreeMap` only support mutable iteration over values not the
    // keys of the key-value pairs since mutating the keys would alter their hashes.
}

// NOTE: `str&` doesn't have an `iter` method since it's
// ambiguous what the items it produces would be. Instead,
// it supports the method `chars` to get an iterator over
// the unicode chars and the method `bytes` to get an
// iterator over each byte.
