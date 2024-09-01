// The trait `Fn` can be used to describe closures,
// this can't be said for all closures however. Some
// closures *move* values they're capturing out of
// their environment. This means that the captured
// value is not available after a call to the closure.
// The closures that *move* values can be described
// by the more restrictive trait `FnOnce` which only
// allows a closure to be called a single time.

fn main() {
    let name = String::from("Raoul Christian Wograndl");
    let kill = || drop(name);

    drop(name);
    drop(name);
}
