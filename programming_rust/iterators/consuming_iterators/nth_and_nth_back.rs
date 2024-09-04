// `nth` and `nth_back` both take in an index `n`.
// They then proceed to skip `n` items that the iterator
// produces and then return the next item:
//
// * `nth(0)` is equivalent to `next()`
// * `nth_back(0)` is equivalent to `next_back()`

fn main() {
    let mut squares = (0..10).map(|n| n * n);

    assert_eq!(squares.nth(0), Some(0));
    assert_eq!(squares.nth(2), Some(9));
    assert_eq!(squares.nth(4), Some(64));
    assert_eq!(squares.nth(8), None);
}
