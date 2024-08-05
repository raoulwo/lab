fn main() {
    let u = Some("raoul");

    // if let expressions are shorthands for match expressions
    if let Some(username) = u {
        println!("Hello, {}!", username);
    }

    // same as this match expression
    match u {
        Some(username) => println!("Hello, {}!", username),
        _ => (),
    }
}
