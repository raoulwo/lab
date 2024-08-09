// Tuple-like struct definition
struct Bounds(usize, usize);

fn main() {
    // Tuple-like struct definition
    let image_bounds = Bounds(1024, 768);
    // ^ `Bounds` is actually a function: fn Bounds(elem0: usize, elem1: usize) -> Bounds { ... }
}
