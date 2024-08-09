// Array patterns
fn hsl_to_rgb(hsl: [u8; 3]) -> [u8; 3] {
    match hsl {
        [_, _, 0] => [0, 0, 0],
        [_, _, 255] => [255, 255, 255],
        _ => hsl,
    }
}

// Slice patterns
fn greet_people(names: &[&str]) {
    match names {
        [] => println!("Hello, nobody."),
        [a] => println!("Hello, {}.", a),
        [a, b] => println!("Hello, {} and {}.", a, b),
        // Since slices can have varying lengths, you can use `..` to match
        // an arbitrary number of elements
        [a, .., b] => println!("Hello, everyone from {} to {}", a, b),
    }
}

fn main() {}
