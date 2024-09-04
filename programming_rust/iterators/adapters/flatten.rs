// `flatten` works on iterators which items are iterable
// themselves (they implement `IntoIterator`). `flatten`
// returns an iterator of the concatenated items. The
// name stems from the image of "flattening" a two-level
// deep structure into a one-level structure.

use std::collections::BTreeMap;

fn main() {
    let mut parks = BTreeMap::new();
    parks.insert("Portland", vec!["Mt.Tabor Park", "Forest Park"]);
    parks.insert("Kyoto", vec!["Tadasu-no-Mori Forest", "Maruyama Koen"]);
    parks.insert("Nashville", vec!["Percy Warner Park", "Dragon Park"]);

    // Here' we build a vector of all concatenated parks. We "flatten" the
    // values of `parks` (iterator over vectors) into an iterator over the
    // concatenated vector elements.
    let all_parks: Vec<_> = parks.values().flatten().cloned().collect();
    for park in &all_parks {
        println!("{}", park);
    }
    println!("");

    // `flatten` can actually be used to filter out `None` in collections
    // of options because `Option`s are actually iterable! This works
    // because `None` represents zero values and `Some` represents one.
    let options = vec![Some("foo"), None, None, Some("bar"), Some("baz")];
    for opt in options.iter().flatten() {
        println!("{}", opt);
    }
}
