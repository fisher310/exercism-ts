use time::PrimitiveDateTime as DateTime;
use std::time::Duration;
use std::ops::Add;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let d = Duration::from_secs(1_000_000_000);
    start.add(d)
}
