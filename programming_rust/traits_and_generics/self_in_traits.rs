pub trait Spliceable {
    // `Self` in a trait refers to the type of the
    // *receiver* that calls this method -> x.splice.
    // If `x` is `String` then `Self` is `String`
    // (not `dyn Spliceable`).
    fn splice(&self, other: &Self) -> Self;

    // NOTE: When using `Self` this way, the trait
    // can't be used as a trait object. This is because
    // Rust has no way to type-check code otherwise.
}

fn main() {}
