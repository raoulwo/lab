fn describe_point(x: i32, y: i32) -> &'static str {
    use std::cmp::Ordering::*;

    match (x.cmp(&0), y.cmp(&0)) {
        (Equal, Equal) => "at the origin",
        (_, Equal) => "on the x axis",
        (Equal, _) => "on the y axis",
        (Greater, Greater) => "in the first quadrant",
        (Less, Greater) => "in the second quadrant",
        _ => "somewhere else",
    }
}

fn main() {
    assert_eq!(describe_point(0, 0), "at the origin");
    assert_eq!(describe_point(1, 0), "on the x axis");
    assert_eq!(describe_point(1, 2), "in the first quadrant");
    assert_eq!(describe_point(1, -2), "somewhere else");
}
