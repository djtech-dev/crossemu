
pub struct Timer {
    time: u8
}
impl Timer {
    pub fn new() -> Timer {
        Timer { time: u8::MAX }
    }
    pub fn tick(&mut self) {
        if self.time > 0 {
            self.time -= 1;
        }
    }
}

pub struct SoundTimer {
    time: u8,
}
impl SoundTimer {
    pub fn new() -> SoundTimer {
        SoundTimer { time: u8::MAX }
    }
    pub fn tick(&mut self) {
        if self.time > 0 {
            self.time -= 1;
        } else {
            unimplemented!("Play a beep sound using Frontend::beep()");
        }

    }
}