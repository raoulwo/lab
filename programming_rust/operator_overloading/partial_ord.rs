use std::cmp::{Ordering, PartialOrd};

// PartialOrd extends PartialEq
#[derive(Debug, PartialEq)]
struct Interval<T> {
    lower: T, // inclusive
    upper: T, // exclusive
}

impl<T: PartialOrd> PartialOrd for Interval<T> {
    // For PartialOrd we return Option<Ordering>,
    // for Ord we simply return Ordering.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.lower >= other.upper {
            Some(Ordering::Greater)
        } else if self.upper <= other.lower {
            Some(Ordering::Less)
        } else {
            // If intervals overlap we don't have order
            None
        }
    }
}

fn main() {
    assert!(
        Interval {
            lower: 10,
            upper: 20
        } < Interval {
            lower: 20,
            upper: 40
        }
    );
    assert!(Interval { lower: 1, upper: 2 } >= Interval { lower: 0, upper: 1 });
    assert!(
        Interval {
            lower: 10,
            upper: 20
        } <= Interval {
            lower: 10,
            upper: 20
        }
    );

    let left = Interval { lower: 0, upper: 8 };
    let right = Interval { lower: 4, upper: 9 };

    // No ordering, neither < nor >=
    assert!(!(left < right));
    assert!(!(left >= right));
}
