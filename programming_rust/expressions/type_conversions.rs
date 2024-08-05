fn main() {
    // Types *usually* have to be converted explicitly
    let x = 0; // <- i32
    let y = x as usize; // <- usize

    // NOTE: There are some automatic conversions:
    //
    // &String -> &str
    // &Vec<T> -> &[T]
    // &Box<T> -> &T
    //
    // These conversions are called *deref coercions*
    // and apply to all types implementing the `Deref` trait.
    //
    // That way, smart pointer types can behave as much as
    // the underlying type as possible and you don't have
    // to explicitly dereference values every time you want
    // to access a field or call a method.
}
