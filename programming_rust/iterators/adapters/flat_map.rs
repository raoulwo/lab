// `flat_map` is similar to `map` and `filter_map`,
// the difference is that the closure of `flat_map`
// can return one or more items. It then produces
// a concatenation of these sequences. The closure
// passed to `flat_map` must return an *iterable*
// so something that is `IntoIterator`.

use std::collections::HashMap;

fn main() {
    let mut cities = HashMap::new();
    cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    cities.insert("France", vec!["Paris"]);
    cities.insert("Austria", vec!["Vienna", "Mattersburg"]);

    let countries = ["Japan", "France", "Austria"];

    for &city in countries.iter().flat_map(|country| &cities[country]) {
        println!("{}", city);
    }
}
