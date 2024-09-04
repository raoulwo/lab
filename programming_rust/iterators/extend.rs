// If a type implements `std::iter::Extend`, then its `extend`
// method can be used to add the items of an iterable (`IntoIterator`)
// to its collection.
fn main() {
    let mut v: Vec<i32> = (0..3).collect();
    v.extend(3..6);
    assert_eq!(vec![0, 1, 2, 3, 4, 5], v);
}

// NOTE: All standard collections implement `Extend`, it's also pretty
// similar to `std::iter::FromIterator`, in fact many implementations
// of `from_iter` just use `extend` internally.
