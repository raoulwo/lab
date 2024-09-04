// The `Iterator` trait provides a lot of methods
// that have default implementations when you implement
// the `next` method manually. Some of those methods
// are *adapter methods*. These are methods that consume
// an iterator and return a transformed one (this makes
// iterators behave lazily) until *collected/consumed*.

// Here's an example of the well known adapter methods
// `map` and `filter`:
fn main() {
    let text = "  ponies  \n   giraffes\nmonkeys  \nsquids".to_string();
    let v: Vec<&str> = text.
        // Returns an iterator over the string's lines. It **doesn't**
        // actually parse the lines of a string, just creates an iterator
        // that **would** do so **if** consumed.
        lines().
        // `map` takes in a `FnMut` (which all functions fulfill).
        map(str::trim)
        // `filter` takes in a `FnMut` predicate. The `FnMut` takes
        // in its arg by shared ref, that's why we need to deref `s`.
        // (The type of `s` is `&&str` -> after deref: `&str`)
        .filter(|s| *s != "squids")
        // Iterators are "lazy", computation is only performed when needed.
        // And when computation is needed, iterators do the minimum amount
        // necessary.
        .collect();
    assert_eq!(vec!["ponies", "giraffes", "monkeys"], v);
}

// NOTE: Creating a pipeline of adapters on an iterator doesn't consume
// any items. The result is simply a modified iterator that's result of
// the composed adapter methods. Only when explicitly consumed (by the
// consumer) for example by calling `next` items are produced. This
// characteristic of iterators in Rust is called **laziness*, Rust's
// iterators are lazy and only do as much as needed to satisfy the
// requests of their clients and only when explicitly asked.

// NOTE: Iterator adapters are zero-overhead abstractions!
