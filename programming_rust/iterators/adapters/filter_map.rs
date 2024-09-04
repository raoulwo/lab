// `filter_map` is similar to `map` in that they
// both transform an item into another one, `filter_map`
// can additionally drop items from the iteration.
// It's like a combination of `filter` and `map`. Instead
// of returning a new item value, it returns an option.
// Is it `None` then it's dropped else it's transformed.
use std::str::FromStr;

fn main() {
    // Here, we scan a string for whitespace-separated words.
    // We only care about words that can be parsed into numbers,
    // dropping all other values.
    let text = "1\nfoo .5 10\n7.125 bar\n";
    for num in text
        .split_whitespace()
        .filter_map(|word| f64::from_str(word).ok())
    {
        println!("{:4.2}", num);
    }
}
