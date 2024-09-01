// THere are three categories of closures:
//
// * `Fn`: You can call these closures multiple
//   times without any restrictions (all `fn` functions).
// * `FnMut`: You can call these closures multiple
//   times as long as the closure itself is mut.
// * `FnOnce`: You can only call these closures once
//   as long as you own the closure.

// Here's how the pseudocode for these Traits would look:
trait Fn() -> R {
    // Requires a shared ref
    fn call(&self) -> R;
}

trait FnMut() -> R {
    // Requires a mutable ref
    fn call_mut(&mut self) -> R;
}

trait FnOnce() -> R {
    // Consumes `self`
    fn call_one(self) -> R;
}
