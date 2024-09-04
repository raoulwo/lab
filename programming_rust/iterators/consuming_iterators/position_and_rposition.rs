// `position` applies a predicate to each item of an
// iterator and returns the first index for which it
// returns true.

// `rposition` does the same from the end of the
// iterator, for this the iterator must be a
// `DoubleEndedIterator`. It also must be an
// `ExactSizeIterator`. The trait for that looks like
// this:
trait MyExactSizeIterator: Iterator {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

// `rposition` only works for iterators which know their
// length (that means the number of items they produce)
// in advance. Not all iterators can fulfill that requirement,
// for example `str::chars` doesn't know about the number
// of characters it'll produce since UTF-8 chars can have
// varying widths (`str::bytes` is an `ExactSizeIterator`
// however).

// NOTE: Both methods return `Option<usize>`, returning
// `None` should the predicate never return `true`.
fn main() {
    let name = "zeus";

    println!("{}", name);
    println!(
        "char position of `e`: {}",
        name.chars().position(|c| c == 'e').unwrap()
    );
    println!(
        "byte position of `u`: {}",
        name.bytes().rposition(|c| c == b'u').unwrap()
    );
}
