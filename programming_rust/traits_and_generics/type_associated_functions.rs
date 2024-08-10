// Traits can include type-associated functions. When
// they do, you can't use them with trait objects. If
// you still want to, you must change the trait to add
// the bound `where Self: Sized` to all associated
// functions using `Self`.

// NOTE: This "trick" works for all trait object
// incompatible methods.
trait StringSet {
    fn new() -> Self
    where
        Self: Sized;
    fn from_slice(strings: &[&str]) -> Self
    where
        Self: Sized;
    fn contains(&self, string: &str) -> bool;
    fn add(&mut self, string: &str);
}

fn main() {}
