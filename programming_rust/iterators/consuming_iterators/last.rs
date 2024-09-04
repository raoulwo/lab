// `last` returns the last item an iterator produces,
// this consumes **all** items. To get the last item
// without consuming all others, use `next_back` of
// a `DoubleEndedIterator` instead.

fn main() {
    let nums = (0..10).map(|n| n * n);
    assert_eq!(Some(81), nums.last());
}
