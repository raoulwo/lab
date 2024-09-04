// `for_each` simply applies a closure to an iterator,
// similar to a for loop. For long adapter chains, it might
// look cleaner however. `try_for_each` is similar but can
// exit early (which is great for operations that might fail).
fn main() {
    (0..10).for_each(|n| println!("{}", n));
}
