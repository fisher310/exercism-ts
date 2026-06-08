use std::fmt::Display;
#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let m = minutes % 60;
        let n = minutes / 60;
        let mn = if m < 0 { 60 + m } else { m };
        let h = (hours + n) % 24;
        let h = if m < 0 {h - 1} else {h};
        let h = if h < 0 { 24 + h } else { h };
        Clock {
            hours: h,
            minutes: mn,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let m = self.minutes + minutes;
        let mn = m % 60;
        let minutes = if mn < 0 { 60 + mn } else { mn };

        let mh = m / 60;
        let h = (self.hours + mh) % 24;
        let hours = if mn < 0 { h - 1} else {h };
        let hours = if hours < 0 { 24 + hours } else { hours };
        Clock { hours, minutes }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:02}:{:02}", self.hours, self.minutes).as_str())
    }
}