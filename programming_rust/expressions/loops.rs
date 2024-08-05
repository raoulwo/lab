fn main() {
    // Infinite loop
    loop {
        break;
    }

    // While loop
    let mut i = 0;
    while i < 5 {
        i += 1;
    }

    // While let loop (analogous to if let)
    while let None = Some(0) {
        println!("Shouldn't be reached");
    }

    // For loop
    for _num in [1, 2, 3] { // For loops over values consume the value!
         // Stuff here...
    }
}
