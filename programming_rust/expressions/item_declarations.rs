fn main() {
    // Blocks in Rust can contain *item declarations*
    // This includes functions, structs, use...
    {
        // Functions defined this way are local to the
        // corresponding scope, they don't capture
        // their environment however, they aren't closures
        fn add(x: i32, y: i32) -> i32 {
            x + y
        }

        struct Point {
            x: f32,
            y: f32,
        }

        use std::collections::HashMap;
    }
}
