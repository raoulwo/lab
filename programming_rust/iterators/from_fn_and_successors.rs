use std::iter::{from_fn, successors};

fn main() {
    // The function `std::iter::from_fn` takes in a closure
    // returning a `Option<T>` and creates an iterator which
    // itself uses the closure to yield/produce its items:
    let only_zeroes: Vec<i32> = from_fn(|| Some(0))
        // Infinite sequence but we only take 1000 elements.
        .take(1_000)
        // Collect actually builds a collection from the iterator.
        // Since the type can't be inferred we specify it explicitly.
        .collect();

    // If an item's value depends on the one before you can use
    // `std::iter::successors` instead to create a *generator*
    // (iterator generating a sequence without underlying data structure).
    let integers: Vec<i32> = successors(Some(1), |&prev| Some(prev + 1))
        .take(1_000)
        .collect();
}

// `from_fn` and `successors` actually accept `FnMut` closures, this means
// they can capture and modify values from surrounding scopes. This allows
// us to implement an iterator generating the fibonacci sequence:
fn fibonacci() -> impl Iterator<Item = usize> {
    let mut state = (0, 1);
    from_fn(move || {
        state = (state.1, state.0 + state.1);
        Some(state.0)
    })
}

// WARNING: These two functions are very flexible/powerful making it possible
// to implement behavior various other iterator methods. This probably creates
// unnecessary complexity, so be sure to look at other methods first before
// attempting to use `from_fn` and `successors`.
