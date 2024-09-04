// `min_by_key` and `max_by_key` select the min/max
// item an iterator yields by applying a closure to each
// item. The closure can select a field or perform a computation
// on the items making it very flexible and generally preferred
// over `max` and `min`. The closures need to return a value that
// is `std::cmp::Ord` (can be strictly compared).
use std::collections::HashMap;

fn main() {
    let mut populations = HashMap::new();
    populations.insert("Vienna", 1_900_000);
    populations.insert("Paris", 2_100_000);
    populations.insert("London", 8_000_000);

    let max_by_population = populations
        .iter()
        .max_by_key(|&(_city, population)| population)
        .unwrap();
    println!(
        "max population: {} => {}",
        max_by_population.0, max_by_population.1
    );

    let min_by_population = populations
        .iter()
        .min_by_key(|&(_city, population)| population)
        .unwrap();
    println!(
        "min population: {} => {}",
        min_by_population.0, min_by_population.1
    );
}
