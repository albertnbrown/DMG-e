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
        Instruction::ADD(target, carry_flag) => {
            let carry: u8 = if carry_flag.include_carry { self.registers.get_carry() } else { 0 };
            let value: u8 = self.fetch_register_target(target) + carry;
            self.registers.a = self.add(self.registers.a, value);
        }
        Instruction::ADDmem(target, carry_flag) => {
            let carry: u8 = if carry_flag.include_carry { self.registers.get_carry() } else { 0 };
            let value: u8 = self.fetch_memory_target(target) + carry;
            self.registers.a = self.add(self.registers.a, value);
        }
        Instruction::ADDn(carry_flag) => {
            let carry: u8 = if carry_flag.include_carry { self.registers.get_carry() } else { 0 };
            let value: u8 = self.fetch_n() + carry;
            self.registers.a = self.add(self.registers.a, value);
        }
        Instruction::SUB(target, carry_flag) => {
            let carry: u8 = if carry_flag.include_carry { self.registers.get_carry() } else { 0 };
            let value: u8 = self.fetch_register_target(target) + carry;
            self.registers.a = self.sub(self.registers.a, value);
        }
        Instruction::SUBmem(target, carry_flag) => {
            let carry: u8 = if carry_flag.include_carry { self.registers.get_carry() } else { 0 };
            let value: u8 = self.fetch_memory_target(target) + carry;
            self.registers.a = self.sub(self.registers.a, value);
        }
        Instruction::SUBn(carry_flag) => {
            let carry: u8 = if carry_flag.include_carry { self.registers.get_carry() } else { 0 };
            let value: u8 = self.fetch_n() + carry;
            self.registers.a = self.sub(self.registers.a, value);
        }
        Instruction::CP(target) => {
            let value: u8 = self.fetch_register_target(target);
            let _ = self.sub(self.registers.a, value); // sub but just for the effect on the flags
        }
        Instruction::CPmem(target) => {
            let value: u8 = self.fetch_memory_target(target);
            let _ = self.sub(self.registers.a, value); // sub but just for the effect on the flags
        }
        Instruction::CPn() => {
            let value: u8 = self.fetch_n();
            let _ = self.sub(self.registers.a, value); // sub but just for the effect on the flags
        }
        Instruction::INC(target) => {
            let carry = self.registers.get_carry(); // need to preserve carry value as this op does not change it
            let new_value: u8 = self.add(self.fetch_register_target(target), 1);
            if carry == 1 {self.registers.flag_carry();} else {self.registers.clear_carry();}
            self.set_register_target(target, new_value);
        }
        Instruction::INCmem(target) => {
            let carry = self.registers.get_carry(); // need to preserve carry value as this op does not change it
            let new_value: u8 = self.add(self.fetch_memory_target(target), 1);
            if carry == 1 {self.registers.flag_carry();} else {self.registers.clear_carry();}
            self.set_memory_target(target, new_value);
        }
        Instruction::DEC(target) => {
            let carry = self.registers.get_carry(); // need to preserve carry value as this op does not change it
            let new_value: u8 = self.sub(self.fetch_register_target(target), 1);
            if carry == 1 {self.registers.flag_carry();} else {self.registers.clear_carry();}
            self.set_register_target(target, new_value);
        }
        Instruction::DECmem(target) => {
            let carry = self.registers.get_carry(); // need to preserve carry value as this op does not change it
            let new_value: u8 = self.sub(self.fetch_memory_target(target), 1);
            if carry == 1 {self.registers.flag_carry();} else {self.registers.clear_carry();}
            self.set_memory_target(target, new_value);
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

    fn fetch_memory_target(&self, target: MemoryTarget) -> u8 {
        match target {
            MemoryTarget::AF => {
                return self.memory.read_byte(self.registers.get_af());
            }
            MemoryTarget::BC => {
                return self.memory.read_byte(self.registers.get_bc());
            }
            MemoryTarget::DE => {
                return self.memory.read_byte(self.registers.get_de());
            }
            MemoryTarget::HL => {
                return self.memory.read_byte(self.registers.get_hl());
            }
        }
    }

    fn set_memory_target(&mut self, target: MemoryTarget, value: u8) {
        match target {
            MemoryTarget::AF => {
                self.memory.set_byte(self.registers.get_af(), value);
            }
            MemoryTarget::BC => {
                self.memory.set_byte(self.registers.get_bc(), value);
            }
            MemoryTarget::DE => {
                self.memory.set_byte(self.registers.get_de(), value);
            }
            MemoryTarget::HL => {
                self.memory.set_byte(self.registers.get_hl(), value);
            }
        }
    }

    fn fetch_n(&mut self) -> u8 {
        let data: u8 = self.memory.read_byte(self.pc);
        self.pc += 1;
        return data;
    }

    fn fetch_nn(&mut self) -> u16 {
        let lsb: u8 = self.memory.read_byte(self.pc);
        self.pc += 1;
        let msb: u8 = self.memory.read_byte(self.pc);
        self.pc += 1;
        let data: u16 = (msb as u16) << 8 | lsb as u16;
        return data;
    }

    // adds to register a
    fn add(&mut self, base: u8, value: u8) -> u8 {
      let (new_value, did_overflow) = base.overflowing_add(value);
      if new_value == 0 { self.registers.flag_zero(); } else { self.registers.clear_zero() }
      self.registers.clear_subtract();
      if did_overflow { self.registers.flag_carry(); } else { self.registers.clear_carry() }
      if (base & 0xF) + (value & 0xF) > 0xF { self.registers.flag_half_carry(); } else { self.registers.clear_half_carry() }
      return new_value;
    }

    // subtracts from register a
    fn sub(&mut self, base: u8, value: u8) -> u8 {
      let (new_value, did_overflow) = base.overflowing_sub(value);
      if new_value == 0 { self.registers.flag_zero(); } else { self.registers.clear_zero() }
      self.registers.flag_subtract();
      if did_overflow { self.registers.flag_carry(); } else { self.registers.clear_carry() }
      let (_, half_carry) = (base & 0xF).overflowing_sub(value & 0xF);
      if  half_carry { self.registers.flag_half_carry(); } else { self.registers.clear_half_carry() }
      return new_value;
    }
  }
  
  