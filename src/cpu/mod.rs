mod registers;
mod memory;
mod instruction;

use registers::Registers;
use memory::Memory;
use instruction::*;

struct CPU {
    registers: Registers,
    pc: u16,
    sp: u16,
    memory: Memory,
}

impl CPU {
    // returns the number of machine cycles taken by the step
    fn step(&mut self) -> usize {
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
          return self.execute(instruction);
        } else {
          let description = format!("0x{}{:x}", if prefixed { "cb" } else { "" }, instruction_byte);
          panic!("Unkown instruction found for: {}", description)
        };
    }

    // returns the number of machine cycles taken by the instruction
    fn execute(&mut self, instruction: Instruction) -> usize {
      match instruction {
        Instruction::ADD(target, carry_flag) => {
            let carry: u8 = if carry_flag.include_carry { self.registers.get_carry() } else { 0 };
            let value: u8 = self.fetch_register_target(target) + carry;
            self.registers.a = self.add(self.registers.a, value);
            return 1;
        }
        Instruction::ADDmem(target, carry_flag) => {
            let carry: u8 = if carry_flag.include_carry { self.registers.get_carry() } else { 0 };
            let value: u8 = self.fetch_memory_target(target) + carry;
            self.registers.a = self.add(self.registers.a, value);
            return 2;
        }
        Instruction::ADDn(carry_flag) => {
            let carry: u8 = if carry_flag.include_carry { self.registers.get_carry() } else { 0 };
            let value: u8 = self.fetch_n() + carry;
            self.registers.a = self.add(self.registers.a, value);
            return 2;
        }
        Instruction::SUB(target, carry_flag) => {
            let carry: u8 = if carry_flag.include_carry { self.registers.get_carry() } else { 0 };
            let value: u8 = self.fetch_register_target(target) + carry;
            self.registers.a = self.sub(self.registers.a, value);
            return 1;
        }
        Instruction::SUBmem(target, carry_flag) => {
            let carry: u8 = if carry_flag.include_carry { self.registers.get_carry() } else { 0 };
            let value: u8 = self.fetch_memory_target(target) + carry;
            self.registers.a = self.sub(self.registers.a, value);
            return 2;
        }
        Instruction::SUBn(carry_flag) => {
            let carry: u8 = if carry_flag.include_carry { self.registers.get_carry() } else { 0 };
            let value: u8 = self.fetch_n() + carry;
            self.registers.a = self.sub(self.registers.a, value);
            return 2;
        }
        Instruction::CP(target) => {
            let value: u8 = self.fetch_register_target(target);
            let _ = self.sub(self.registers.a, value); // sub but just for the effect on the flags
            return 1;
        }
        Instruction::CPmem(target) => {
            let value: u8 = self.fetch_memory_target(target);
            let _ = self.sub(self.registers.a, value); // sub but just for the effect on the flags
            return 2;
        }
        Instruction::CPn() => {
            let value: u8 = self.fetch_n();
            let _ = self.sub(self.registers.a, value); // sub but just for the effect on the flags
            return 2;
        }
        Instruction::INC(target) => {
            let carry = self.registers.get_carry(); // need to preserve carry value as this op does not change it
            let new_value: u8 = self.add(self.fetch_register_target(target), 1);
            if carry == 1 {self.registers.flag_carry();} else {self.registers.clear_carry();}
            self.set_register_target(target, new_value);
            return 1;
        }
        Instruction::INCmem(target) => {
            let carry = self.registers.get_carry(); // need to preserve carry value as this op does not change it
            let new_value: u8 = self.add(self.fetch_memory_target(target), 1);
            if carry == 1 {self.registers.flag_carry();} else {self.registers.clear_carry();}
            self.set_memory_target(target, new_value);
            return 3;
        }
        Instruction::DEC(target) => {
            let carry = self.registers.get_carry(); // need to preserve carry value as this op does not change it
            let new_value: u8 = self.sub(self.fetch_register_target(target), 1);
            if carry == 1 {self.registers.flag_carry();} else {self.registers.clear_carry();}
            self.set_register_target(target, new_value);
            return 1;
        }
        Instruction::DECmem(target) => {
            let carry = self.registers.get_carry(); // need to preserve carry value as this op does not change it
            let new_value: u8 = self.sub(self.fetch_memory_target(target), 1);
            if carry == 1 {self.registers.flag_carry();} else {self.registers.clear_carry();}
            self.set_memory_target(target, new_value);
            return 3;
        }
        Instruction::AND(target) => {
            self.registers.a = self.logical_and(self.registers.a, self.fetch_register_target(target));
            return 1;
        }
        Instruction::ANDmem(target) => {
            self.registers.a = self.logical_and(self.registers.a, self.fetch_memory_target(target));
            return 2;
        }
        Instruction::ANDn() => {
            let value = self.fetch_n(); // mutates pc so must be called separately
            self.registers.a = self.logical_and(self.registers.a, value);
            return 2;
        }
        Instruction::XOR(target) => {
            self.registers.a = self.logical_xor(self.registers.a, self.fetch_register_target(target));
            return 1;
        }
        Instruction::XORmem(target) => {
            self.registers.a = self.logical_xor(self.registers.a, self.fetch_memory_target(target));
            return 2;
        }
        Instruction::XORn() => {
            let value = self.fetch_n(); // mutates pc so must be called separately
            self.registers.a = self.logical_xor(self.registers.a, value);
            return 2;
        }
        Instruction::OR(target) => {
            self.registers.a = self.logical_or(self.registers.a, self.fetch_register_target(target));
            return 1;
        }
        Instruction::ORmem(target) => {
            self.registers.a = self.logical_or(self.registers.a, self.fetch_memory_target(target));
            return 2;
        }
        Instruction::ORn() => {
            let value = self.fetch_n(); // mutates pc so must be called separately
            self.registers.a = self.logical_or(self.registers.a, value);
            return 2;
        }
        Instruction::CCF() => {
            if self.registers.get_carry() == 1 {
                self.registers.clear_carry();
            } else {
                self.registers.flag_carry();
            }
            self.registers.clear_subtract();
            self.registers.clear_half_carry();
            return 1;
        }
        Instruction::SCF() => {
            self.registers.flag_carry();
            self.registers.clear_subtract();
            self.registers.clear_half_carry();
            return 1;
        }
        Instruction::CPL() => {
            self.registers.a = !self.registers.a;
            self.registers.flag_subtract();
            self.registers.flag_half_carry();
            return 1;
        }
        Instruction::DAA() => {
            let mut adjuster: u8 = 0;
            let subtract: bool = self.registers.get_subtract() == 1;
            if self.registers.get_carry() == 1 || (!subtract && self.registers.a > 0x99) {
                adjuster += 0x60;
                self.registers.flag_carry(); // for if the second clause is true but not the first
            }
            if self.registers.get_half_carry() == 1 || (!subtract && self.registers.a & 0x0F > 0x09) {
                adjuster += 0x06;
            }
            if subtract {
                let (new_value, _) = self.registers.a.overflowing_sub(adjuster);
                self.registers.a = new_value;
            } else {
                let (new_value, _) = self.registers.a.overflowing_add(adjuster);
                self.registers.a = new_value;
            }
            self.registers.clear_half_carry();
            if self.registers.a == 0 {self.registers.flag_zero();} else {self.registers.clear_zero();}
            return 1;
        }
        Instruction::JumpNN(condition) => {
            let mut cycles: usize = 3;
            // need to increment PC by fetching outside the conditional
            let new_location = self.fetch_nn();
            let do_jump = self.check_conditional(condition);
            if do_jump {
                self.pc = new_location;
                cycles += 1;
            }
            return cycles;
        }
        Instruction::JumpHL() => {
            self.pc = self.registers.get_hl();
            return 1;
        }
        Instruction::JumpRn(condition) => {
            let mut cycles: usize = 2;
            // need to increment PC by fetching outside the conditional
            let n = self.fetch_n();
            let do_jump = self.check_conditional(condition);
            if do_jump {
                self.pc = (((self.pc + (n as u16)) as i32) + (i8::MIN as i32)) as u16;
                cycles += 1;
            }
            return cycles;
        }
        Instruction::CallNN(condition) => {
            let mut cycles: usize = 3;
            let new_location = self.fetch_nn();
            let do_call = self.check_conditional(condition);
            if do_call {
                self.push(self.pc);
                self.pc = new_location;
                cycles += 3;
            }
            return cycles;
        }
        Instruction::Return(condition) => {
            let mut cycles: usize = 2;
            let do_call = self.check_conditional(condition);
            if do_call {
                self.pc =  self.pop();
                match condition {
                    Conditional::Unconditional => {cycles += 2;}
                    _ => {cycles += 3;}
                }
            }
            return cycles;
        }
        Instruction::CallI(target) => {
            self.push(self.pc);
            self.pc = self.deref_invariant_function(target);
            return 4;
        }
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

    // adds and sets flags
    fn add(&mut self, base: u8, value: u8) -> u8 {
      let (new_value, did_overflow) = base.overflowing_add(value);
      if new_value == 0 { self.registers.flag_zero(); } else { self.registers.clear_zero() }
      self.registers.clear_subtract();
      if did_overflow { self.registers.flag_carry(); } else { self.registers.clear_carry() }
      if (base & 0xF) + (value & 0xF) > 0xF { self.registers.flag_half_carry(); } else { self.registers.clear_half_carry() }
      return new_value;
    }

    // subtracts and sets flags
    fn sub(&mut self, base: u8, value: u8) -> u8 {
      let (new_value, did_overflow) = base.overflowing_sub(value);
      if new_value == 0 { self.registers.flag_zero(); } else { self.registers.clear_zero() }
      self.registers.flag_subtract();
      if did_overflow { self.registers.flag_carry(); } else { self.registers.clear_carry() }
      let (_, half_carry) = (base & 0xF).overflowing_sub(value & 0xF);
      if  half_carry { self.registers.flag_half_carry(); } else { self.registers.clear_half_carry() }
      return new_value;
    }

    // logical and
    fn logical_and(&mut self, base: u8, value: u8) -> u8 {
        let new_value = base & value;
        if new_value == 0 { self.registers.flag_zero(); } else { self.registers.clear_zero() }
        self.registers.clear_subtract();
        self.registers.flag_half_carry();
        self.registers.clear_carry();
        return new_value;
    }

    // logical xor
    fn logical_xor(&mut self, base: u8, value: u8) -> u8 {
        let new_value = base ^ value;
        if new_value == 0 { self.registers.flag_zero(); } else { self.registers.clear_zero() }
        self.registers.clear_subtract();
        self.registers.clear_half_carry();
        self.registers.clear_carry();
        return new_value;
    }

    // logical or
    fn logical_or(&mut self, base: u8, value: u8) -> u8 {
        let new_value = base | value;
        if new_value == 0 { self.registers.flag_zero(); } else { self.registers.clear_zero() }
        self.registers.clear_subtract();
        self.registers.clear_half_carry();
        self.registers.clear_carry();
        return new_value;
    }

    // for parsing conditional expressions
    fn check_conditional(&self, conditional: Conditional) -> bool {
        match conditional {
            Conditional::ZeroFlag => {
                if self.registers.get_zero() == 1 {return true;} else {return false;}
            }
            Conditional::NotZeroFlag => {
                if self.registers.get_zero() == 0 {return true;} else {return false;}
            }
            Conditional::CarryFlag => {
                if self.registers.get_carry() == 1 {return true;} else {return false;}
            }
            Conditional::NotCarryFlag => {
                if self.registers.get_carry() == 0 {return true;} else {return false;}
            }
            Conditional::Unconditional => {
                return true;
            }
        }
    }

    fn deref_invariant_function(&self, function: InvariantFunction) -> u16 {
        match function {
            InvariantFunction::F00 => {
                return 0x0000_u16;
            }
            InvariantFunction::F08 => {
                return 0x0008_u16;
            }
            InvariantFunction::F10 => {
                return 0x0010_u16;
            }
            InvariantFunction::F18 => {
                return 0x0018_u16;
            }
            InvariantFunction::F20 => {
                return 0x0020_u16;
            }
            InvariantFunction::F28 => {
                return 0x0028_u16;
            }
            InvariantFunction::F30 => {
                return 0x0030_u16;
            }
            InvariantFunction::F38 => {
                return 0x0038_u16;
            }
        }
    }

    // push to the stack
    fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.memory.set_byte(self.sp, ((value & 0xFF00) >> 8) as u8);
    
        self.sp = self.sp.wrapping_sub(1);
        self.memory.set_byte(self.sp, (value & 0xFF) as u8);
    }

    // pop from the stack
    fn pop(&mut self) -> u16 {
        let lsb = self.memory.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
    
        let msb = self.memory.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
    
        return (msb << 8) | lsb;
      }
}
  
  