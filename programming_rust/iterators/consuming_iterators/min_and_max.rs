// `min` and `max` return the least or greatest item
// an iterator can produce. The iterator's item type
// must be `std::cmp::Ord` so that they can be compared.
// This means you can't use these methods with floats
// since they're `PartialOrd` but not `Ord`.
fn main() {
    let nums = [-2, -1, 0, 1, 2];
    println!("{:?}", nums);

    // NOTE: The methods return `Option<Self::Item>` so
    // that they can return `None` in case the iterator
    // doesn't produce any items.
    let min = nums.iter().min().unwrap();
    println!("min: {}", min);

    let max = nums.iter().max().unwrap();
    println!("max: {}", max);
}
