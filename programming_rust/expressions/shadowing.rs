fn main() {
    let foo = 0;
    // Here, `foo` is initialized a second time *shadowing* the first one
    let foo = "foo";
}
