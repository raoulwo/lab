fn main() {
    // break expressions can return values out of a loop
    let foo = loop {
        break "foo";
    };
}
