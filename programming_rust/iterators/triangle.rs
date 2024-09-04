fn triangle(n: i32) -> i32 {
    let mut sum = 0;
    // `1..=n` is of type `RangeInclusive<i32>` which is an iterator,
    // that means it can be used inside a for loop.
    for num in 1..=n {
        sum += num;
    }
    sum
}

fn triangle_fold(n: i32) -> i32 {
    // Here, a more functional approach using `fold` that is also
    // possible because `1..=n` is an iterator.
    (1..=n).fold(0, |sum, num| sum + num)
}

// NOTE: Rust's iterators perform really well and can often be optimized
// by the compiler (0-cost abstractions).

fn main() {
    let n1 = triangle(10);
    let n2 = triangle_fold(10);
    assert_eq!(n1, n2);
}
