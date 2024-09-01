// The trait `std::clone::Clone` is implemented
// for types that can create copies of themselves.
// It's defined as:
trait Clone: Sized {
    fn clone(&self) -> Self;
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

// `Clone` extends `Sized`, this constraints types
// that can implement `Clone` to only be `Sized`.
// For many scenarios it's good enough to simply
// derive `Clone`.
#[derive(Clone)]
struct User {
    id: String,
    name: String,
}

fn main() {
    let u1 = User {
        id: "1".to_string(),
        name: "raoulwo".to_string(),
    };
    let u2 = u1.clone();
}
