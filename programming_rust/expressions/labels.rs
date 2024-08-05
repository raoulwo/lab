fn main() {
    // loops can be labeled with lifetimes for breaking out of them
    let foo = 'outer: loop {
        loop {
            break 'outer "foo";
        }
    };
    println!("{}", foo);
}
