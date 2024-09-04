// You can't be sure that an iterator continues
// to produce the value `None` once it has produced
// it for the first time. It's actually up to the
// implementation of said iterator and is in no shape
// or form standardized behavior you should count on.
// The adapter `fuse` takes an iterator and transforms
// it into one that continues to produce `None` after
// it has done so the first time!
struct Flaky(bool);

impl Iterator for Flaky {
    type Item = &'static str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 {
            self.0 = false;
            Some("totally the last item")
        } else {
            self.0 = true;
            None
        }
    }
}

fn main() {
    let mut flaky = Flaky(true);
    assert_eq!(flaky.next(), Some("totally the last item"));
    assert_eq!(flaky.next(), None);
    // Here, the iterator produces `Some(...)` after already having
    // produced `None`!
    assert_eq!(flaky.next(), Some("totally the last item"));
    assert_eq!(flaky.next(), None);
    assert_eq!(flaky.next(), Some("totally the last item"));

    let mut fused = Flaky(true).fuse();
    assert_eq!(fused.next(), Some("totally the last item"));
    assert_eq!(fused.next(), None);
    // Here, the iterator continues to produce `None`!
    assert_eq!(fused.next(), None);
    assert_eq!(fused.next(), None);
    assert_eq!(fused.next(), None);
}
