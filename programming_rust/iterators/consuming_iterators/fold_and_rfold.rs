// `fold` is like `reduce` from languages like TS, Ruby,
// Elixir... It is used to accumulate a single value from
// an initial value and an *accumulator* function which
// takes in the next item produced by the iterator and the
// accumulated value. `rfold` accumulates over an iterator
// from the back, the iterator must be `DoubleEndedIterator`.

fn main() {
    let nums = [1, 2, 3, 4, 5];
    println!("{:?}", nums);
    println!("sum: {}", nums.iter().fold(0, |sum, num| sum + num));

    let strs = ["192", "168", "0", "1"];
    println!("{:?}", strs);
    println!(
        "join: {}",
        strs[1..]
            .iter()
            .fold(String::from(strs[0]), |s, octet| s + "." + octet)
    );
}
