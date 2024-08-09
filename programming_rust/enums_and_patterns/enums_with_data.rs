// Enums in Rust are known in other languages as sum types,
// algebraic types or discriminated unions. They are more
// powerful than regular enums in C/C++ since they can hold data!

// NOTE: All constructors/fields of enums share the visibility of the enum itself.

enum RoughTime {
    // The variants `InThePast` and `InTheFuture` are *tuple variants*.
    // They have constructors like tuple-like structs to create new values.
    InThePast(TimeUnit, u32),
    // The variant `JustNow` mirrors a unit-like struct since it doesn't
    // hold any value
    JustNow,
    InTheFuture(TimeUnit, u32),
}

struct Point {
    x: f64,
    y: f64,
}

const ORIGIN: Point = Point { x: 0.0, y: 0.0 };

// Enums can also have struct variants
enum Shape {
    Circle {
        center: Point,
        radius: f64,
    },
    Rectangle {
        upper_left: Point,
        lower_right: Point,
    },
}

// NOTE: In memory, enums with data are stored as small integer tags
// together with enough memory to hold the fields of its largest variant.

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
    // Initializing values from tuple variants
    let four_years_ago = RoughTime::InThePast(TimeUnit::Years, 4);
    let three_hours_from_now = RoughTime::InTheFuture(TimeUnit::Hours, 3);

    // Initializing a value from struct variant
    let unit_circle = Shape::Circle {
        center: ORIGIN,
        radius: 1.0,
    };
}
