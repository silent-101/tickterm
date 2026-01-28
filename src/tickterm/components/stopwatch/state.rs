#[derive(Debug)]
pub enum BtnMode {
    Start,
    Stop,
    Reset,
    Set,
}

#[derive(Debug)]
pub struct BtnState {
    focused_on: BtnMode,
}

impl BtnState {
    pub fn new() -> Self {
        Self {
            focused_on: BtnMode::Start,
        }
    }
    pub fn get_state(&mut self) -> BtnMode {
        match self.focused_on {
            BtnMode::Start => BtnMode::Start,
            BtnMode::Stop => BtnMode::Stop,
            BtnMode::Reset => BtnMode::Reset,
            BtnMode::Set => BtnMode::Set,
        }
    }
    pub fn change_state(&mut self, state: BtnMode) {
        self.focused_on = state;
    }
}
