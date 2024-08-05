fn main() {
    // Functions that never return are called *divergent functions*
    // They have the special `!` return type.
    std::process::exit(1);
}

// You can also write your own divergent functions
fn divergent() -> ! {
    loop {}
}
