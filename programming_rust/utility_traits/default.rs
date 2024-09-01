// The `std::default::Default` trait can be
// used to provide types with a *default value*.
// The trait looks like this:
trait MyDefault {
    fn default() -> Self;
}

// All of Rust's collection types implement default
// returning an empty collection.
impl MyDefault for String {
    fn default() -> String {
        String::new()
    }
}

// NOTE: If `T` is `Default`, then Rust's standard
// library automatically implemens `Default` for
// smart pointers like `Rc<T>`, `Arc<T>`, `Box<T>`...

// NOTE: If all elements of a tuple are `Default`
// then the tuple does as well. For structs you have
// to specify so explicitly however.

fn main() {}
