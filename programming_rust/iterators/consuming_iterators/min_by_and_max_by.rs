// `max_by` and `min_by` return the greatest/least
// element of a collection, determined by a comparison
// function.
use std::cmp::Ordering;

fn cmp(lhs: &f64, rhs: &f64) -> Ordering {
    lhs.partial_cmp(rhs).unwrap()
}

fn main() {
    let nums = [-2.0, -1.0, 0.0, 1.0, 2.0];
    println!("{:?}", nums);

    // NOTE: Returns `Option<Self::Item>` since iterator
    // may produce no values.

    let min = nums.iter().copied().min_by(cmp).unwrap();
    println!("{}", min);

    let max = nums.iter().copied().max_by(cmp).unwrap();
    println!("{}", max);
}
