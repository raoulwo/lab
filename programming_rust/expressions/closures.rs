fn main() {
    // Rust supports closures (arg and return types are inferred)
    let is_even = |x| x % 2 == 0;

    // You can explicitly specify types, if you do so with the 
    // return type, you must enclose the function body in a block
    let is_odd |x: u32| -> bool { x % 2 == 1 };
}
