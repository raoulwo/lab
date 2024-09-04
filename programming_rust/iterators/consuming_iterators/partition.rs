// `partition` can divide an iterator's items among
// two collections of the same type. It uses a predicate
// to specify the target collection.
fn main() {
    let things = ["doorknob", "mushroom", "noodle", "giraffe", "grapefruit"];

    // Like with `collect`, you need to specify the collection types since
    // Rust can't infer them for you.
    let (living, nonliving): (Vec<&str>, Vec<&str>) =
        things.iter().partition(|name| name.as_bytes()[0] & 1 != 0);

    println!("living: {:?}", living);
    println!("nonliving: {:?}", nonliving);
}

// NOTE: `partition` requires its result collection type to implement the
// following traits: `std::default::Default` and `std::default::Extend`.
// All standard collections do so.
