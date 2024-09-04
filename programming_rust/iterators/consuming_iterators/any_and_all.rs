// `any` checks if a predicate is fulfilled for at
// least one item an iterator produces. `all` checks
// if the given predicate is satisfied for all items.
fn main() {
    let name = "Raoul Christian Wograndl";

    println!("{}", name);
    println!("any uppercase? {}", name.chars().any(char::is_uppercase));
    println!("all uppercase? {}", name.chars().all(char::is_uppercase));
}
