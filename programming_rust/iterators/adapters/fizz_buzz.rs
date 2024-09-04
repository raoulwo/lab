// Cool fizz buzz example using some iterator adapters!

use std::iter::{once, repeat};

fn main() {
    let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
    let buzzes = repeat("").take(4).chain(once("buzz")).cycle();
    let fizzes_buzzes = fizzes.zip(buzzes);

    let fizz_buzz = (1..100).zip(fizzes_buzzes).map(|pair| match pair {
        (i, ("", "")) => i.to_string(),
        (_, (fizz, buzz)) => format!("{}{}", fizz, buzz),
    });

    for line in fizz_buzz {
        println!("{}", line);
    }
}
