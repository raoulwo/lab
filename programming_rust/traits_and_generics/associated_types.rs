trait Enumerator {
    // This is an associated type of the trait. Each
    // type implementing this trait must explicitly
    // specify the associated type.
    type Item;

    // This method produces the next value of the
    // enumerator (iterator) of the associated type.
    // The associated type can't stand alone (Item),
    // it must be qualified.
    fn next(&mut self) -> Option<Self::Item>;
}

use std::env::Args;

// Here's how you could implement the trait.
impl Enumerator for Args {
    // You have to specify the associated type.
    type Item = String;

    fn next(&mut self) -> Option<String> {
        None
    }
}

// Generic code can also use the associated types.
fn collect_into_vector<E: Enumerator>(enumerator: E) -> Vec<E::Item> {
    vec![]
}

use std::fmt::Debug;

// Here's another generic example with a bound on the associated type.
fn dump<E>(enumerator: &mut E)
where
    E: Enumerator,
    E::Item: Debug,
{
    while let Some(value) = enumerator.next() {
        println!("{:?}", value);
    }
}

// You can also specify subsets of generics where the associated type
// is specified explicitly.
fn foo<E>(enumerator: E)
where
    E: Enumerator<Item = String>, // Here, we specify `item` to be `String`
{
}

fn main() {}
