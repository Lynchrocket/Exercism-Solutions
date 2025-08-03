use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let h = (((hours + minutes / 60) % 24) + 24) % 24;
        let m = minutes % 60;
        Clock {
            hours: if m < 0 { (h-1 + 24) % 24 } else { h },
            minutes: (m + 60) % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let h = ((self.hours + (self.minutes + minutes) / 60) % 24 + 24) % 24;
        let m = (self.minutes + minutes) % 60;
        Clock {
            hours: if m < 0 { (h-1 + 24) % 24 } else { h },
            minutes: (m + 60) % 60,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.minutes == other.minutes && self.hours == other.hours
    }
}