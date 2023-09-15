// https://exercism.org/tracks/rust/exercises/clock

use std::fmt;

pub struct Clock {
    seconds: i32,
}

impl Clock {
    pub fn new(hour: i32, minute: i32, second: i32) -> Clock {
        let sec = hour * 3600 + minute * 60 + second;
        Clock { seconds: sec }
    }
    pub fn add_seconds(&self, secs: i32) -> Clock {
        let sec = (self.seconds + secs) % 86400;
        Clock::new(0, 0, sec)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hour = self.seconds / 3600;
        let minute = (self.seconds % 3600) / 60;
        let second = self.seconds % 60;
        write!(f, "{:02}:{:02}:{:02}", hour, minute, second)
    }
}
