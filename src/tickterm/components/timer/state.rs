#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialEq, Eq)]
pub enum TimerMode {
    SetterMode,
    RunningMode,
    NormalMode,
}

#[derive(Debug)]
pub enum TimerSagemnt {
    Hour,
    Min,
    Sec,
}

pub struct TimerCtx {
    mode: TimerMode,
    timer_sagment: TimerSagemnt,
}

impl TimerCtx {
    pub fn new() -> Self {
        Self {
            mode: TimerMode::NormalMode,
            timer_sagment: TimerSagemnt::Sec,
        }
    }
    pub fn set_timer_mode(&mut self, mode: TimerMode) {
        self.mode = mode
    }
    pub fn set_timer_sagment(&mut self, sagment: TimerSagemnt) {
        self.timer_sagment = sagment
    }
    pub fn get_timer_mode(&self) -> &TimerMode {
        &self.mode
    }
    pub fn get_timer_sagment(&self) -> &TimerSagemnt {
        &self.timer_sagment
    }
}
