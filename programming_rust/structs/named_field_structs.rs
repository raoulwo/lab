// Named-field struct definition
struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize),
}

fn main() {
    let width = 1024;
    let height = 576;
    // Named-field struct expression
    let image = GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height),
    };
    // Field access using `.` operator
    assert_eq!(image.size, (1024, 576));
}

fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap {
    assert_eq!(size.0 * size.1, pixels.len());
    // Shorthand expression
    GrayscaleMap { size, pixels }
}

mod foo {
    // Structs are private by default
    pub struct Bar {
        // Even if the struct is public, their fields are private by default
        pub val: String,
    }
}
