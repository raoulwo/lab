// NOTE: When implementing traits, either the trait or
// the type must be new in the current crate -> *Orphan Rule*.
// If Rust would allow this, different crates could implement
// any trait on any type resulting in different conflicting
// implementations.

struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
}

struct Canvas {}

// Give the trait a name and define its type signatures
trait Visible {
    fn draw(&self, canvas: &mut Canvas);
    fn hit_test(&self, x: i32, y: i32) -> bool;

    // Traits can also define default implementations
    // for any types implementing the trait but not
    // defining the corresponding method.
}

// Implement the trait for a given type
impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        // ...
    }

    fn hit_test(&self, x: i32, y: i32) -> bool {
        // ...

        false
    }
}

fn main() {}
