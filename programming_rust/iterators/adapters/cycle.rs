// `cycle` transforms an iterator into an iterator
// that repeats the sequence of the underlying
// iterator ad infinitum. The underlying iterator
// must be `Clone` so that `cycle` can save its
// initial state and reuse it each time it starts
// again.
fn main() {
    let directions = ["north", "east", "south", "west"];
    let mut spin = directions.iter().cycle();
    assert_eq!(spin.next(), Some(&"north"));
    assert_eq!(spin.next(), Some(&"east"));
    assert_eq!(spin.next(), Some(&"south"));
    assert_eq!(spin.next(), Some(&"west"));
    assert_eq!(spin.next(), Some(&"north"));
    assert_eq!(spin.next(), Some(&"east"));
    assert_eq!(spin.next(), Some(&"south"));
    assert_eq!(spin.next(), Some(&"west"));
}
