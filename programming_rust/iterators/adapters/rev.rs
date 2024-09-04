// Iterators that implement the `DoubleEndedIterator` trait
// can produce items from "both ends". Either from the front
// as normal using `next` or from the end backwards using
// `next_back`. The `DoubleEndedIterator` trait extends the
// `Iterator` trait (each double ended iterator *is an* iterator)
// and looks like this:
trait MyDoubleEndedIterator: Iterator {
    fn next_back(&mut self) -> Option<Self::Item>;
}

// You can think of double ended iterators having two *cursors*
// marking the position of the current position of the front and
// back sequence as opposed to normal iterators only having one
// *cursor*.
fn main() {
    let animals = vec!["monkey", "lion", "eagle"];

    // Consuming items from either end moves the cursors towards each
    // other. Once they meet the iteration is done:
    let mut iter = animals.iter();
    assert_eq!(iter.next(), Some(&"monkey"));
    assert_eq!(iter.next_back(), Some(&"eagle"));
    assert_eq!(iter.next(), Some(&"lion"));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);

    // Each double ended iterator can be reversed using the `rev`
    // adapter. The resulting iterator is also double ended, `next`
    // and `next_back` are just swapped.
    let mut rev_iter = animals.iter().rev();
    assert_eq!(rev_iter.next(), Some(&"eagle"));
    assert_eq!(rev_iter.next_back(), Some(&"monkey"));
    assert_eq!(rev_iter.next(), Some(&"lion"));
    assert_eq!(rev_iter.next_back(), None);
    assert_eq!(rev_iter.next(), None);
}
