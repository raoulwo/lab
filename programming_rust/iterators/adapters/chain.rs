// The `chain` adapter can append an iterator or iterable
// to an iterator. This will lead to the first iterator
// producing all values until it's exhausted and then
// proceeding to the next iterator producing its values.
// The appended iterator/iterable must produce the same
// item type for chain to work, if both iterators/iterables
// are reversible then the combined chain of both is reversible
// as well!
fn main() {
    let v: Vec<i32> = (1..4).chain([4, 5, 6]).collect();
    assert_eq!(vec![1, 2, 3, 4, 5, 6], v);

    let v: Vec<i32> = (1..4).chain([4, 5, 6]).rev().collect();
    assert_eq!(vec![6, 5, 4, 3, 2, 1], v);
}
