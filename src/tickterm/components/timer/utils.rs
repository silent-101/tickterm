use std::time::{Duration, Instant};

struct TimerSetterValue {
    hour: u32,
    min: u32,
    sec: u32,
}

pub struct TimerUtils {
    timer_setter_value: TimerSetterValue,
    target_duration: Duration,
    start_time: Option<Instant>,
    paused_remaining: Option<Duration>,
}

impl TimerUtils {
    pub fn new() -> Self {
        Self {
            timer_setter_value: TimerSetterValue {
                hour: 0,
                min: 0,
                sec: 0,
            },
            target_duration: Duration::ZERO,
            start_time: None,
            paused_remaining: None,
        }
    }

    pub fn start(&mut self) {
        let duration = if let Some(remaining) = self.paused_remaining {
            remaining
        } else {
            Duration::from_secs(
                (self.timer_setter_value.hour * 3600
                    + self.timer_setter_value.min * 60
                    + self.timer_setter_value.sec) as u64,
            )
        };
        self.target_duration = duration;
        self.start_time = Some(Instant::now());
        self.paused_remaining = None;
    }

    pub fn stop(&mut self) {
        if let Some(start) = self.start_time {
            let elapsed = start.elapsed();
            self.paused_remaining = Some(self.target_duration.saturating_sub(elapsed));
            self.start_time = None;
        }
    }

    pub fn reset(&mut self) {
        self.timer_setter_value.hour = 0;
        self.timer_setter_value.min = 0;
        self.timer_setter_value.sec = 0;
        self.target_duration = Duration::ZERO;
        self.start_time = None;
        self.paused_remaining = None;
    }

    pub fn get_remaining_time(&self) -> (u32, u32, u32) {
        let remaining = if let Some(start) = self.start_time {
            self.target_duration.saturating_sub(start.elapsed())
        } else if let Some(paused) = self.paused_remaining {
            paused
        } else {
            Duration::from_secs(
                (self.timer_setter_value.hour * 3600
                    + self.timer_setter_value.min * 60
                    + self.timer_setter_value.sec) as u64,
            )
        };

        let total_secs = remaining.as_secs();
        let hours = (total_secs / 3600) as u32;
        let minutes = ((total_secs % 3600) / 60) as u32;
        let seconds = (total_secs % 60) as u32;

        (hours, minutes, seconds)
    }

    pub fn is_timer_running(&self) -> bool {
        if let Some(start) = self.start_time {
            start.elapsed() < self.target_duration
        } else {
            false
        }
    }
    pub fn has_value(&self) -> bool {
        self.timer_setter_value.hour > 0
            || self.timer_setter_value.min > 0
            || self.timer_setter_value.sec > 0
    }
    pub fn increase_sec(&mut self) {
        self.timer_setter_value.sec = (self.timer_setter_value.sec + 1).min(59);
    }

    pub fn increase_min(&mut self) {
        self.timer_setter_value.min = (self.timer_setter_value.min + 1).min(59);
    }

    pub fn increase_hour(&mut self) {
        self.timer_setter_value.hour = (self.timer_setter_value.hour + 1).min(99);
    }

    pub fn decrease_sec(&mut self) {
        self.timer_setter_value.sec = self.timer_setter_value.sec.saturating_sub(1);
    }

    pub fn decrease_min(&mut self) {
        self.timer_setter_value.min = self.timer_setter_value.min.saturating_sub(1);
    }

    pub fn decrease_hour(&mut self) {
        self.timer_setter_value.hour = self.timer_setter_value.hour.saturating_sub(1);
    }

    pub fn get_sec(&self) -> String {
        let (_, _, sec) = self.get_remaining_time();
        self.format_time(sec)
    }

    pub fn get_min(&self) -> String {
        let (_, min, _) = self.get_remaining_time();
        self.format_time(min)
    }

    pub fn get_hour(&self) -> String {
        let (hour, _, _) = self.get_remaining_time();
        self.format_time(hour)
    }

    fn format_time(&self, time: u32) -> String {
        format!("{:02}", time)
    }
}
