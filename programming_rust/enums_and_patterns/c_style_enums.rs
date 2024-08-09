// Basic C-style enum (elements stored as ints starting with 0 by default)
// Rust stores C-style enums in the smallest integer type that can fit all values.
enum Element {
    Earth,
    Wind,
    Fire,
}

enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    // ...
}

// Like struct, enums can derive traits as well
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
    Months,
    Years,
}

// Enums can have methods
impl TimeUnit {
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Weeks => "weeks",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

fn main() {
    // Casting from enum to int is allowed
    assert_eq!(HttpStatus::Ok as i32, 200);
}
