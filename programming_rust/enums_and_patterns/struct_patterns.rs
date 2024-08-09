struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 0, y: 5 };

    match point {
        Point { x: 0, y: height } => println!("straight up {} meters", height),
        // `..` is used to disregard other fields
        Point { y: 0, .. } => println!("at the ground"),
        // Can use shorthand `Point { x, y }` instead!
        Point { x: x, y: y } => println!("at ({}m, {}m)", x, y),
    }
}
