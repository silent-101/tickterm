use chrono::{DateTime, Duration, Utc};

#[derive(Debug, Clone)]
pub struct StopWatch {
    start_time: Option<DateTime<Utc>>,
    elapsed: Duration,
    hours: String,
    min: String,
    sec: String,
    cs: String,
    running: bool,
}

impl Default for StopWatch {
    fn default() -> Self {
        Self {
            start_time: None,
            elapsed: Duration::zero(),
            hours: "00".into(),
            min: "00".into(),
            sec: "00".into(),
            cs: "00".into(),
            running: false,
        }
    }
}

impl StopWatch {
    fn format_time(&self, time: u32) -> String {
        if time < 10 {
            format!("0{}", time)
        } else {
            time.to_string()
        }
    }

    pub fn start(&mut self) {
        if !self.running {
            self.start_time = Some(Utc::now());
            self.running = true;
        }
    }

    pub fn stop(&mut self) {
        if self.running {
            if let Some(start) = self.start_time {
                self.elapsed += Utc::now() - start;
            }
            self.start_time = None;
            self.running = false;
        }
    }

    pub fn reset(&mut self) {
        self.start_time = None;
        self.elapsed = Duration::zero();
        self.running = false;
        self.hours = "00".into();
        self.min = "00".into();
        self.sec = "00".into();
        self.cs = "00".into();
    }

    pub fn update(&mut self) {
        let total = if let Some(start) = self.start_time {
            self.elapsed + (Utc::now() - start)
        } else {
            self.elapsed
        };

        let total_ms = total.num_milliseconds();
        let ms = total_ms % 1000 / 10;
        let sec = (total_ms / 1000) % 60;
        let min = (total_ms / (60 * 1000)) % 60;
        let hour = total_ms / (60 * 60 * 1000);

        self.cs = self.format_time(ms as u32);
        self.sec = self.format_time(sec as u32);
        self.min = self.format_time(min as u32);
        self.hours = self.format_time(hour as u32);
    }
    pub fn get_ms(&self) -> String {
        self.cs.to_string()
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
    pub fn get_time(&self) -> String {
        format!("{}:{}:{}:{}", self.hours, self.min, self.sec, self.cs)
    }
}
