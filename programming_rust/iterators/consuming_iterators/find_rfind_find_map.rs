// `find` and `rfind` take a predicate and return
// the first (option of an) item of an iterator for
// which the predicate is true, else `None` if never
// true.

// `find_map` is similar, its closure instead returns
// an option. The method then returns the first option
// that is `Some`.

fn main() {
    let nums = [1, 3, 5, 6];
    assert_eq!(Some(6), nums.iter().copied().find(|n| n % 2 == 0));
}
