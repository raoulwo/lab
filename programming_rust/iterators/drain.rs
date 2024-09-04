use std::iter::FromIterator;

fn main() {
    // NOTE: Many collections also provide a `drain` method that is similar to
    // `into_iter`. Unlike `into_iter` `drain` takes a mutable ref and returns
    // an iterator that passes ownership to the consumer. This doesn't move
    // the original collection, instead it removes the elements. When the
    // collection can be indexed, you can pass a range to drain indicating
    // which range of elements to "drain" out of the collection:
    let mut outer = "Earth".to_string();
    let inner = String::from_iter(outer.drain(1..4));

    assert_eq!("Eh", outer);
    assert_eq!("art", inner);
}
