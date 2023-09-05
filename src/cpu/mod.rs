mod registers;
mod memory;
mod instruction;

use registers::Registers;
use memory::Memory;
use instruction::*;

struct CPU {
    registers: Registers,
    pc: u16,
    memory: Memory,
}

impl CPU {
    fn step(&mut self) {
        //fetch
        let mut instruction_byte = self.memory.read_byte(self.pc);
        self.pc += 1;
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.memory.read_byte(self.pc); // pc has already been incremented
            self.pc += 1; // increment again to put in expected location
        }
    
        // decode
        if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
          self.execute(instruction)
        } else {
          let description = format!("0x{}{:x}", if prefixed { "cb" } else { "" }, instruction_byte);
          panic!("Unkown instruction found for: {}", description)
        };
    }

    fn execute(&mut self, instruction: Instruction) {
      match instruction {
        Instruction::ADD(target) => {
            let value = self.fetch_register_target(target);
            let new_value = self.add(value);
            self.registers.a = new_value;
        }
        _ => { /* TODO: support more instructions */ }
      }
    }

    // map enum to register
    fn fetch_register_target(&self, target: RegisterTarget) -> u8 {
        match target {
            RegisterTarget::A => {
                return self.registers.a;
            }
            RegisterTarget::B => {
                return self.registers.b;
            }
            RegisterTarget::C => {
                return self.registers.c;
            }
            RegisterTarget::D => {
                return self.registers.d;
            }
            RegisterTarget::E => {
                return self.registers.e;
            }
            RegisterTarget::H => {
                return self.registers.h;
            }
            RegisterTarget::L => {
                return self.registers.l;
            }
        }
    }

    // map enum to register
    fn set_register_target(&mut self, target: RegisterTarget, value: u8) {
        match target {
            RegisterTarget::A => {
                self.registers.a = value;
            }
            RegisterTarget::B => {
                self.registers.b = value;
            }
            RegisterTarget::C => {
                self.registers.c = value;
            }
            RegisterTarget::D => {
                self.registers.d = value;
            }
            RegisterTarget::E => {
                self.registers.e = value;
            }
            RegisterTarget::H => {
                self.registers.h = value;
            }
            RegisterTarget::L => {
                self.registers.l = value;
            }
        }
    }
  
    // adds to register a
    fn add(&mut self, value: u8) -> u8 {
      let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
      if new_value == 0 { self.registers.flag_zero(); } else { self.registers.clear_zero() }
      self.registers.clear_subtract();
      if did_overflow { self.registers.flag_carry(); } else { self.registers.clear_carry() }
      if (self.registers.a & 0xF) + (value & 0xF) > 0xF { self.registers.flag_half_carry(); } else { self.registers.clear_half_carry() }
      return new_value;
    }
  }
  
  