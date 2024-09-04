// An iterator in Rust is a value that implements the
// `std::iter::Iterator` trait which look like this:
trait MyIterator {
    // `Item` is the type of values the iterators will produce.
    type Item;
    // `next` either returns an option of the associated item
    // type it yields. It returns `None` to indicate the end
    // of its sequence.
    fn next(&mut self) -> Option<Self::Item>;
    // ... a lot of methods with default implementations.
}

// Types can implement the `std::iter::IntoIterator` trait so
// that they can be transformed into iterators. Any type that
// is `IntoIterator` is *iterable* since it can be transformed
// into an iterator at any time.
trait MyIntoIterator
where
    // The iterator we can transform into must yield items of
    // the same type we specify in `Self::Item`.
    Self::IntoIter: MyIterator<Item = Self::Item>,
{
    // Again, `Item` is the type of values the iterator we can
    // transform into will produce.
    type Item;
    // `IntoIter` is the type of the iterator the implementor will
    // be able to transform into.
    type IntoIter: MyIterator;
    // NOTE: `into_iter` takes in `self`, which means that the
    // implementor is consumed when transformed into the corresponding
    // iterator.
    fn into_iter(self) -> Self::IntoIter;
}

fn main() {
    let v = vec!["foo", "bar", "baz"];
    // Iterators allow us to use a for loop to iterate over vectors:
    for item in &v {
        println!("{}", item);
    }
    println!("");

    // Under the hood, for loops are shorthands for the following
    // iterator calls to `IntoIterator` and `Iterator` methods:
    let mut iter = (&v).into_iter();
    while let Some(item) = iter.next() {
        println!("{}", item);
    }
    println!("");

    // NOTE: We borrow `v` immutably so we don't consume the vec
    // by calling `into_iter`.

    // NOTE: For loops call `into_iter` on their operands if they
    // are *iterables* (= implement `IntoIterator`). Should they
    // already be iterators (like `RangeInclusive<T>`) they just
    // use the iterator directly because **all** iterators already
    // implement `IntoIterator` of which the `into_iter` just returns
    // themselves.

    // NOTE: (Read above) This means **all** iterators are also iterable.
}

// The basic terminology of iterators:
//
// * *Iterators* are any types that implement `Iterator`.
// * *Iterables* are any types that implement `IntoIterator`.
// * Iterators *produce/yield* values. These values are called *items*.
// * The code receiving the items is called the *consumer* of the iterator.
