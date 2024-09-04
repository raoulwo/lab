// `zip` combines two iterators into a single iterator
// that produces pairs holding one value from each iterator.
// The name stems from the image of a zipper joining its two
// sides into a single seam. The zipped iterator ends when
// the first of its two underlying iterators ends.
fn main() {
    // Here, we emulate the adapter `enumerate` by zipping the
    // unbounded range `0..` with a finite iterator:
    let v: Vec<(i32, &str)> = (0..).zip(["foo", "bar", "baz"]).collect();
    assert_eq!(vec![(0, "foo"), (1, "bar"), (2, "baz")], v);

    // NOTE: You can think of `zip` as a more flexible generalization
    // of `enumerate`. Also, like `chain` its argument doesn't have to
    // be an iterator. It can also be an iterable.
}
