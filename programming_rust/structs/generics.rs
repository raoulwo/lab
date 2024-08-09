// Struct of generic type T
pub struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>,
}

// Impl-block for generic type T
impl Queue<T> {
    // NOTE: `Self` is a special type available inside impl blocks
    // representing the associated type. The function signature
    // can also be written as: pub fn new() -> Self { ... }
    pub fn new() -> Queue<T> {
        // Could also do `Self { ... }` as return value
        Queue {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }

    // ...
}

// Impl-block *only* for f64
impl Queue<f64> {
    pub fn sum(&self) -> f64 {
        0.0
    }
}

fn main() {
    // Generic associated functions can be called using the *turbofish* syntax: `::<T>`
    // when type inference doesn't apply.
    let mut q = Queue::<char>::new();
}
