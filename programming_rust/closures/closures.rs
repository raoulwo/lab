// Rust supports *closures* (functions that "capture" their environment).
// Because Rust isn't garbage collected, there are consequences for using
// closures that you wouldn't have to take into consideration in other
// languages. Rust uses lifetimes instead of garbage collection, whenever
// closures capture a reference to a variable in their environments Rust
// requires that the references (closures) can't outlive the original.

#[derive(Debug)]
struct City {
    name: String,
    country: String,
    population: i64,
}

impl City {
    // Functions can be stored in structs or vectors, in memory
    // they are a single memory address of the function's machine
    // code (like function pointers in C++).
    // fn(&City, &Statistic) -> i64
    fn get_statistic(&self, stat: &Statistic) -> i64 {
        match stat {
            Statistic::Population => self.population,
        }
    }
}

enum Statistic {
    Population,
}

// Functions can be used as values, this means they have types.
// The type of this function is: fn(&mut Vec<City) -> ()
fn sort_cities(cities: &mut Vec<City>) {
    // Here, we pass a closure to `sort_by_key`.
    cities.sort_by_key(|city| -city.population);
}

fn sort_by_statistic(cities: &mut Vec<City>, stat: Statistic) {
    // This closure borrows a ref to `stat`, this means it can't outlive
    // `stat` itself.
    cities.sort_by_key(|city| -city.get_statistic(&stat));
}

use std::thread;

fn start_sorting_thread(mut cities: Vec<City>, stat: Statistic) -> thread::JoinHandle<Vec<City>> {
    let key_fn = move |city: &City| -> i64 { -city.get_statistic(&stat) };

    // In this case we want to use `&cities` and `&stat` (captured in the closure `key_fn`)
    // in another thread. Rust can't make sure that the original values are still available
    // across different threads, that's why we need to **move** those values to the closures.
    // That way, the closures actually **steal** the values instead of holding references.

    // This means Rust's closures follow the same rules that apply elsewhere: They can
    // *move* or they can *borrow* data. Whenever `Copy` types would be moved, they are copied
    // instead. By abiding to these rules Rust can *guarantee thread safety* that way.

    thread::spawn(move || {
        cities.sort_by_key(key_fn);
        cities
    })
}

// It is possible to define higher order functions that take in other functions.
// Rust's function values are basically the same thing as function pointers in C/C++.
fn count_selected_cities(cities: &Vec<City>, test_fn: fn(&City) -> bool) -> usize {
    let mut count = 0;
    for city in cities {
        if test_fn(city) {
            count += 1;
        }
    }
    count
}

// Closures **do not** have the same type as functions. To support closures as function
// arguents you must introduce a generic type param for the closure that satisfies certain
// function trait bounds (`Fn`, `FnMut`, `FnOnce`).
fn count_selected_cities_closure<F>(cities: &Vec<City>, test_fn: F) -> usize
where
    // This trait bound constraints `F` to implement the trait `Fn(&City) -> bool`.
    // This trait is automatically implemented by all functions of same signature,
    // making this more general than the signature above which only supports functions
    // but not closures.
    F: Fn(&City) -> bool,
{
    let mut count = 0;
    for city in cities {
        if test_fn(city) {
            count += 1;
        }
    }
    count
}

fn main() {
    let vienna = City {
        name: "Vienna".into(),
        country: "Austria".into(),
        population: 1_900_000,
    };
    let paris = City {
        name: "Paris".into(),
        country: "France".into(),
        population: 2_100_000,
    };
    let mut cities = vec![vienna, paris];
    println!("{:?}", cities);

    sort_cities(&mut cities);
    println!("{:?}", cities);
}

// NOTE: Rust's closures are performant, they aren't garbage collected nor are they
// heap allocated (unless explicitly wrapped by a `Box<T>` or other pointers). Each
// closure has a distinct type, whenever the Rust compiler knows which closure is
// called it can inline it. That's why you can even use them in tight loops.

// NOTE: There are multiple ways that closures can be represented in memory:
//
// * move closures: structs that contain their own values
// * non-move closures: structs that contain refs to the original values
// * closures which don't capture: are basically function pointers -> single memory
//   word pointing to the function's machine code.
//
// Just like with structs the same `Copy` and `Clone` rules apply for closures as well:
//
// * non-move closures that only hold shared refs are both `Clone` and `Copy`
// * non-move closures that mutate therefore hold mutable refs and are neither
//   `Clone` nor `Copy`
// * move closures are `Copy` if all values they move are `Copy` and they're
//   `Clone` if all values they move are `Clone`
