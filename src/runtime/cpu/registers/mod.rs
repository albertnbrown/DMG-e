mod flags_register;

use flags_register::FlagsRegister;

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    f: FlagsRegister,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn initialize() -> Registers {
        return Registers { a: 0, b: 0, c:0, d: 0, e: 0, h: 0, l: 0, f: FlagsRegister { zero: false, subtract: false, half_carry: false, carry: false }};
    }

    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8
        | self.c as u16
    }
  
    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }
    
    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8
        | self.e as u16
    }
  
    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8
        | self.l as u16
    }
  
    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8
        | u8::from(self.f) as u16
    }
  
    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = FlagsRegister::from((value & 0xFF) as u8);
    }

    pub fn get_zero(&self) -> u8 {
        return if self.f.zero { 1 } else { 0 }
    }

    pub fn get_subtract(&self) -> u8 {
        return if self.f.subtract { 1 } else { 0 }
    }

    pub fn get_carry(&self) -> u8 {
        return if self.f.carry { 1 } else { 0 }
    }

    pub fn get_half_carry(&self) -> u8 {
        return if self.f.half_carry { 1 } else { 0 }
    }

    pub fn flag_zero(&mut self) {
        self.f.zero = true;
    }

    pub fn clear_zero(&mut self) {
        self.f.zero = false;
    }

    pub fn flag_subtract(&mut self) {
        self.f.subtract = true;
    }

    pub fn clear_subtract(&mut self) {
        self.f.subtract = false;
    }

    pub fn flag_carry(&mut self) {
        self.f.carry = true;
    }

    pub fn clear_carry(&mut self) {
        self.f.carry = false;
    }

    pub fn flag_half_carry(&mut self) {
        self.f.half_carry = true;
    }

    pub fn clear_half_carry(&mut self) {
        self.f.half_carry = false;
    }
  }