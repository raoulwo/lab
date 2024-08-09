enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

impl RoughTime {
    fn to_english(&self) -> String {
        match self {
            RoughTime::InThePast(units, 1) => format!("1 {} ago", units.singular()),
            RoughTime::InThePast(units, count) => format!("{} {} ago", count, units.plural()),
            RoughTime::JustNow => format!("just now"),
            RoughTime::InTheFuture(units, 1) => format!("1 {} from now", units.singular()),
            RoughTime::InTheFuture(units, count) => {
                format!("{} {} from now", count, units.plural())
            }
        }
    }
}

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
    let one_hour_ago = RoughTime::InThePast(TimeUnit::Hours, 1);
    let just_now = RoughTime::JustNow;
    let one_week_from_now = RoughTime::InTheFuture(TimeUnit::Weeks, 1);
    let two_days_from_now = RoughTime::InTheFuture(TimeUnit::Days, 2);

    assert_eq!(one_hour_ago.to_english(), "1 hour ago".to_string());
    assert_eq!(just_now.to_english(), "just now".to_string());
    assert_eq!(
        one_week_from_now.to_english(),
        "1 week from now".to_string()
    );
    assert_eq!(
        two_days_from_now.to_english(),
        "2 days from now".to_string()
    );
}
