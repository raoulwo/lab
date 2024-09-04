// The adapter `inspect` is like `IO.inspect` in Elixir.
// You give it a closure to a shared ref of the item with
// which you can debug by either printing or making assertions
// about them. `inspect` then passes the item through to the
// next method in the pipeline.

fn main() {
    // NOTE: When executing this program you can actually see
    // how iterators are lazily evaluated because of the log
    // output order.

    let nums: Vec<i32> = (0..10)
        .into_iter()
        .inspect(|n| println!("{}", n))
        .map(|n| n * n)
        .inspect(|n| println!("{}", n))
        .collect();
}
