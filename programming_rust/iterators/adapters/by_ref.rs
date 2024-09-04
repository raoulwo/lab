// Normally, adapters consume the iterator they're called on
// to produce a new iterator. In scenarios where you want to
// use a transformed iterator but afterwards still use the
// previous iterator you can first call its `by_ref` method
// which borrows the iterator mutably, **then** you can pass
// the borrowed ref to an adapter. This makes it possible for
// the adapater to still consume items from the iterator while
// advancing its cursor, however when the borrow of the iterator
// by the adapter ends you can access the original iterator.

// Here's a combined example of using `take_while` and `skip_while`
// to read the header lines and body lines of a given text using
// the same underlying iterator (achieved using `by_ref`):
fn main() {
    let message = "To: John Doe\nFrom: Raoul Wograndl\n\nHi!\n";

    let mut lines = message.lines();

    println!("Headers:");
    // Here, we use `by_ref` to mutably borrow the iterator over the
    // lines of the message to consume lines until we encounter an empty line.
    // This doesn't completely consume the underlying iterator since we only
    // borrowed an exclusive ref.
    for header in lines.by_ref().take_while(|line| !line.is_empty()) {
        println!("{}", header);
    }
    println!("");

    println!("Body:");
    // Here, we can use the original iterator over the lines the print out the
    // rest of the message.
    for body in lines {
        println!("{}", body);
    }
}
