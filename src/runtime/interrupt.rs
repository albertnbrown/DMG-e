use super::cpu::memory::{VBLANK_BIT, STAT_BIT, TIMER_BIT, SERIAL_BIT, JOYPAD_BIT};

#[derive(Clone, Copy, Debug)]
pub enum Interrupt {
    VBlank,
    LCDSTAT,
    Timer,
    Serial,
    Joypad,
    None,
}

impl std::convert::From<u8> for Interrupt {
    fn from(register: u8) -> Self {
        if (register >> VBLANK_BIT) & 1 == 1 {
            return Interrupt::VBlank;
        }
        if (register >> STAT_BIT) & 1 == 1 {
            return Interrupt::LCDSTAT;
        }
        if (register >> TIMER_BIT) & 1 == 1 {
            return Interrupt::Timer;
        }
        if (register >> SERIAL_BIT) & 1 == 1 {
            return Interrupt::Serial;
        }
        if (register >> JOYPAD_BIT) & 1 == 1 {
            return Interrupt::Joypad;
        }
        return Interrupt::None;
    }
}

impl std::convert::From<Interrupt> for u8 {
    fn from(interrupt: Interrupt) -> Self {
        match interrupt {
            Interrupt::VBlank => {
                return 1 << VBLANK_BIT;
            }
            Interrupt::LCDSTAT => {
                return 1 << STAT_BIT;
            }
            Interrupt::Timer => {
                return 1 << TIMER_BIT;
            }
            Interrupt::Serial => {
                return 1 << SERIAL_BIT;
            }
            Interrupt::Joypad => {
                return 1 << JOYPAD_BIT;
            }
            Interrupt::None => {
                return 0;
            }
        }
    }
}

impl std::convert::From<Interrupt> for u16 {
    fn from(interrupt: Interrupt) -> Self {
        match interrupt {
            Interrupt::VBlank => {
                return 0x40;
            }
            Interrupt::LCDSTAT => {
                return 0x48;
            }
            Interrupt::Timer => {
                return 0x50;
            }
            Interrupt::Serial => {
                return 0x58;
            }
            Interrupt::Joypad => {
                return 0x60;
            }
            Interrupt::None => {
                panic!("Attempted to call None interrupt");
            }
        }
    }
}

