#[derive(Clone, Copy)]
pub enum TimerSpeed {
    Clock1024,
    Clock16,
    Clock64,
    Clock256,
}

pub struct TimerControl {
    pub enabled: bool,
    pub speed: TimerSpeed,
}

impl std::convert::From<u8> for TimerSpeed {
    fn from(index: u8) -> Self {
        match index {
            0 => {
                TimerSpeed::Clock1024
            }
            1 => {
                TimerSpeed::Clock16
            }
            2 => {
                TimerSpeed::Clock64
            }
            3 => {
                TimerSpeed::Clock256
            }
            _ => {
                panic!("TimerSpeed u8 conversion: unreachable case.")
            }
        }
    }
}

// using mem-cycles for timers atm
impl std::convert::From<TimerSpeed> for usize {
    fn from(speed: TimerSpeed) -> Self {
        match speed {
            TimerSpeed::Clock1024 => {
                256
            }
            TimerSpeed::Clock16 => {
                4
            }
            TimerSpeed::Clock64 => {
                16
            }
            TimerSpeed::Clock256 => {
                64
            }
        }
    }
}

impl std::convert::From<u8> for TimerControl {
    fn from(byte: u8) -> Self {
        let enabled = (byte >> 2) & 1 == 1;
        
        let speed_index = byte & 0b11;

        TimerControl {
            enabled,
            speed: TimerSpeed::from(speed_index),
        }
    }
}