// `enumerate` transforms an iterator to one
// that attaches a running index to its sequence.
// An iterator that formerly produced values: A, B, C
// now would produce pairs of (0, A), (1, B), (2, C)
// and so on.
fn main() {
    for (index, character) in ('a'..'f').into_iter().enumerate() {
        println!("{}: {}", index, character);
    }
}
