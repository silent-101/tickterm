use chrono::{self, Local, Timelike};

#[derive(Debug, Clone)]
pub struct Time {
    min: String,
    hours: String,
    sec: String,
    am_or_pm: String,
}

impl Default for Time {
    fn default() -> Self {
        let mut time = Self {
            min: String::new(),
            hours: String::new(),
            sec: String::new(),
            am_or_pm: String::new(),
        };
        time.set_new_time();
        time
    }
}

impl Time {
    pub fn get_am_or_pm(&self) -> String {
        self.am_or_pm.to_string()
    }
    pub fn set_new_time(&mut self) {
        let dt = Local::now();
        let hour_24 = dt.hour();

        self.sec = self.formate_time(dt.second());
        self.min = self.formate_time(dt.minute());

        // Convert to 12-hour format
        let hour_12 = match hour_24 {
            0 => 12,
            1..=12 => hour_24,
            _ => hour_24 - 12,
        };
        self.hours = self.formate_time(hour_12);

        // Determine AM or PM
        self.am_or_pm = if hour_24 < 12 { "am" } else { "pm" }.to_string();
    }
    pub fn get_time(&self) -> String {
        format!("{}:{}:{}", self.hours, self.min, self.sec)
    }
    fn formate_time(&mut self, time: u32) -> String {
        if time < 10 {
            format!("0{}", time)
        } else {
            time.to_string()
        }
    }
}
