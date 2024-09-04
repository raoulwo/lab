// The method `collect` can be used to transform an
// iterator into a collection. Any type that implements
// `FromIterator` can be constructed this way:
trait MyFromIterator<A>: Sized {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self;
}

// NOTE: There exists an iterator method `size_hint` which returns
// a lower and optional upper bound for how many items it will produce.
// This information can lead to us implementing a more efficient
// `from_iter`.

fn main() {}
