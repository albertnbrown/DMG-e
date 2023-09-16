mod registers;
mod memory;
mod instruction;
mod instruction_history;

use registers::Registers;
use memory::Memory;
use instruction::*;
use instruction_history::InstructionHistory;
use std::fmt;

const DEBUG_INSTRUCTIONS_PER_LINE: usize = 3;
const HISTORY_SIZE: usize = 99; // make divisible by DEBUG_INSTRUCTIONS_PER_LINE for good printing
const PADDING_WIDTH: usize = 63;

pub struct CPU {
    registers: Registers,
    pc: u16,
    sp: u16,
    pub memory: Memory,
    nop_count: usize,
    instruction_history: [InstructionHistory; HISTORY_SIZE],
}

impl fmt::Display for CPU {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, chunk) in self.instruction_history.chunks(DEBUG_INSTRUCTIONS_PER_LINE).rev().enumerate() {
            let mut write = "".to_owned();
            for (j, history_e) in chunk.iter().rev().enumerate() {
                write.push_str(
                    &format!("{0:<2$} - {1} ",
                    HISTORY_SIZE - i*DEBUG_INSTRUCTIONS_PER_LINE - j,
                    history_e,
                    (HISTORY_SIZE as f64).log10().floor() as usize + 1),
                );
                let padding_len: usize = ((j+1)*PADDING_WIDTH).checked_sub(write.chars().count()).unwrap_or(0);
                write.push_str(&" ".repeat(padding_len))
            }
            writeln!(f, "{}", &write)?;
        }
        write!(f,"^^ Instruction History Tail ^^")
    }
}

impl CPU {
    pub fn initialize(file_name: String) -> CPU {
        return CPU {
            registers: Registers::initialize(),
            pc: 0x0100,
            sp: 0xFFFE,
            memory: Memory::initialize(file_name),
            nop_count: 0,
            instruction_history: [InstructionHistory::new(); HISTORY_SIZE],
        }
    }

    // returns the number of machine cycles taken by the step
    pub fn step(&mut self, step_count: usize) -> usize {
        //fetch
        let mut mem_cycles: usize = 0;
        let mut instruction_byte: u8 = self.memory.read_byte(self.pc);
        self.pc += 1;
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.memory.read_byte(self.pc); // pc has already been incremented
            self.pc += 1; // increment again to put in expected location
            mem_cycles += 1;
        }
    
        // decode
        if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
            // println!("{:?} pc: {:x}", instruction, self.pc);
            match instruction {
                Instruction::NOP() => {
                    self.nop_count += 1;
                }
                _ => {
                    self.instruction_history[HISTORY_SIZE-1] = InstructionHistory {
                        inst: instruction,
                        pc: self.pc,
                        nops: self.nop_count,
                        spi: (0x10000 - self.sp as u32)/2 - 1,
                    };
                    self.instruction_history.rotate_right(1);
                    self.nop_count = 0;
                }
            }
            mem_cycles += self.execute(instruction);
        } else {
            println!("{}", self);
            println!("failed on step: {}", step_count);
            let description = format!("0x{}{:x}", if prefixed { "cb" } else { "" }, instruction_byte);
            panic!("Unkown instruction found for: {}", description)
        };

        if self.pc >= 0xFFFD {
            println!("{}", self);
            println!("failed on step: {}", step_count);
            panic!("pc too high");
        }

        if self.sp == 0x0 {
            println!("{}", self);
            println!("failed on step: {}", step_count);
            panic!("sp too high");
        }

        // if self.pc == 0xC000 {
        //     self.memory.print_range(self.pc as usize, 50);
        // }

        return mem_cycles;
    }

    // returns the number of machine cycles taken by the instruction
    fn execute(&mut self, instruction: Instruction) -> usize {
      match instruction {
        Instruction::NOP() => { return 1; }
        Instruction::ADD(target, include_carry) => {
            let carry: u8 = if include_carry { self.registers.get_carry() } else { 0 };
            let value: u8 = self.get_register_target(target) + carry;
            self.registers.a = self.add(self.registers.a, value);
            return 1;
        }
        Instruction::ADDmem(target, include_carry) => {
            let carry: u8 = if include_carry { self.registers.get_carry() } else { 0 };
            let value: u8 = self.get_memory_target(target) + carry;
            self.registers.a = self.add(self.registers.a, value);
            return 2;
        }
        Instruction::ADDn(include_carry) => {
            let carry: u8 = if include_carry { self.registers.get_carry() } else { 0 };
            let value: u8 = self.get_n() + carry;
            self.registers.a = self.add(self.registers.a, value);
            return 2;
        }
        Instruction::ADD16(source) => {
            let value: u16 = self.get_double_register_target(source);
            self.registers.l = self.add(self.registers.l, (value & 0x00FF) as u8);
            let carry: u8 = self.registers.get_carry();
            self.registers.h = self.add(self.registers.h, ((value & 0xFF00) >> 8) as u8 + carry);
            return 2;
        }
        Instruction::ADD16SP() => {
            let value: u16 = self.sp;
            self.registers.l = self.add(self.registers.l, (value & 0x00FF) as u8);
            let carry: u8 = self.registers.get_carry();
            self.registers.h = self.add(self.registers.h, ((value & 0xFF00) >> 8) as u8 + carry);
            return 2;
        }
        Instruction::SUB(target, include_carry) => {
            let carry: u8 = if include_carry { self.registers.get_carry() } else { 0 };
            let value: u8 = self.get_register_target(target) + carry;
            self.registers.a = self.sub(self.registers.a, value);
            return 1;
        }
        Instruction::SUBmem(target, include_carry) => {
            let carry: u8 = if include_carry { self.registers.get_carry() } else { 0 };
            let value: u8 = self.get_memory_target(target) + carry;
            self.registers.a = self.sub(self.registers.a, value);
            return 2;
        }
        Instruction::SUBn(include_carry) => {
            let carry: u8 = if include_carry { self.registers.get_carry() } else { 0 };
            let value: u8 = self.get_n() + carry;
            self.registers.a = self.sub(self.registers.a, value);
            return 2;
        }
        Instruction::CP(target) => {
            let value: u8 = self.get_register_target(target);
            let _ = self.sub(self.registers.a, value); // sub but just for the effect on the flags
            return 1;
        }
        Instruction::CPmem(target) => {
            let value: u8 = self.get_memory_target(target);
            let _ = self.sub(self.registers.a, value); // sub but just for the effect on the flags
            return 2;
        }
        Instruction::CPn() => {
            let value: u8 = self.get_n();
            let _ = self.sub(self.registers.a, value); // sub but just for the effect on the flags
            return 2;
        }
        Instruction::INC(target) => {
            let carry: u8 = self.registers.get_carry(); // need to preserve carry value as this op does not change it
            let new_value: u8 = self.add(self.get_register_target(target), 1);
            if carry == 1 {self.registers.flag_carry();} else {self.registers.clear_carry();}
            self.set_register_target(target, new_value);
            return 1;
        }
        Instruction::INCmem(target) => {
            let carry: u8 = self.registers.get_carry(); // need to preserve carry value as this op does not change it
            let new_value: u8 = self.add(self.get_memory_target(target), 1);
            if carry == 1 {self.registers.flag_carry();} else {self.registers.clear_carry();}
            self.set_memory_target(target, new_value);
            return 3;
        }
        Instruction::INC16(target) => {
            // we are not changing any of the flags
            let (new_value, _) = self.get_double_register_target(target).overflowing_add(1);
            self.set_double_register_target(target, new_value);
            return 2;
        }
        Instruction::INCSP() => {
            // we are not changing any of the flags
            let (new_value, _) = self.sp.overflowing_add(1);
            self.sp = new_value;
            return 2;
        }
        Instruction::DEC(target) => {
            let carry: u8 = self.registers.get_carry(); // need to preserve carry value as this op does not change it
            let new_value: u8 = self.sub(self.get_register_target(target), 1);
            if carry == 1 {self.registers.flag_carry();} else {self.registers.clear_carry();}
            self.set_register_target(target, new_value);
            return 1;
        }
        Instruction::DECmem(target) => {
            let carry: u8 = self.registers.get_carry(); // need to preserve carry value as this op does not change it
            let new_value: u8 = self.sub(self.get_memory_target(target), 1);
            if carry == 1 {self.registers.flag_carry();} else {self.registers.clear_carry();}
            self.set_memory_target(target, new_value);
            return 3;
        }
        Instruction::DEC16(target) => {
            // we are not changing any of the flags
            let (new_value, _) = self.get_double_register_target(target).overflowing_sub(1);
            self.set_double_register_target(target, new_value);
            return 2;
        }
        Instruction::DECSP() => {
            // we are not changing any of the flags
            let (new_value, _) = self.sp.overflowing_sub(1);
            self.sp = new_value;
            return 2;
        }
        Instruction::AND(target) => {
            self.registers.a = self.logical_and(self.registers.a, self.get_register_target(target));
            return 1;
        }
        Instruction::ANDmem(target) => {
            self.registers.a = self.logical_and(self.registers.a, self.get_memory_target(target));
            return 2;
        }
        Instruction::ANDn() => {
            let value: u8 = self.get_n(); // mutates pc so must be called separately
            self.registers.a = self.logical_and(self.registers.a, value);
            return 2;
        }
        Instruction::XOR(target) => {
            self.registers.a = self.logical_xor(self.registers.a, self.get_register_target(target));
            return 1;
        }
        Instruction::XORmem(target) => {
            self.registers.a = self.logical_xor(self.registers.a, self.get_memory_target(target));
            return 2;
        }
        Instruction::XORn() => {
            let value: u8 = self.get_n(); // mutates pc so must be called separately
            self.registers.a = self.logical_xor(self.registers.a, value);
            return 2;
        }
        Instruction::OR(target) => {
            self.registers.a = self.logical_or(self.registers.a, self.get_register_target(target));
            return 1;
        }
        Instruction::ORmem(target) => {
            self.registers.a = self.logical_or(self.registers.a, self.get_memory_target(target));
            return 2;
        }
        Instruction::ORn() => {
            let value: u8 = self.get_n(); // mutates pc so must be called separately
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
            // need to increment PC by geting outside the conditional
            let new_location: u16 = self.get_nn();
            let do_jump: bool = self.check_conditional(condition);
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
            // need to increment PC by geting outside the conditional
            let n: u8 = self.get_n();
            let do_jump: bool = self.check_conditional(condition);
            if do_jump {
                // parens make sure that PC only overflows if the instruction is bad
                self.pc = ((self.pc as i32) + (n as i8) as i32) as u16;
                cycles += 1;
            }
            return cycles;
        }
        Instruction::CallNN(condition) => {
            let mut cycles: usize = 3;
            let new_location: u16 = self.get_nn();
            let do_call: bool = self.check_conditional(condition);
            if do_call {
                self.push(self.pc);
                self.pc = new_location;
                cycles += 3;
            }
            return cycles;
        }
        Instruction::Return(condition) => {
            let mut cycles: usize = 2;
            let do_call: bool = self.check_conditional(condition);
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
        Instruction::LoadRR(destination, source) => {
            self.set_register_target(destination, self.get_register_target(source));
            return 1;
        }
        Instruction::LoadRN(destination) => {
            let n: u8 = self.get_n();
            self.set_register_target(destination, n);
            return 2;
        }
        Instruction::LoadRMem(destination_register, memory_source, post_op) => {
            let data: u8 = self.get_memory_target(memory_source);
            self.set_register_target(destination_register, data);
            self.do_post_op(memory_source, post_op);
            return 2;
        }
        Instruction::LoadMemR(memory_destination, source_register, post_op) => {
            let data: u8 = self.get_register_target(source_register);
            self.set_memory_target(memory_destination, data);
            self.do_post_op(memory_destination, post_op);
            return 2;
        }
        Instruction::LoadMemN(destination) => {
            let n: u8 = self.get_n();
            self.set_memory_target(destination, n);
            return 3;
        }
        Instruction::LoadRNN(destination) => {
            let nn: u16 = self.get_nn();
            let data: u8 = self.memory.read_byte(nn);
            self.set_register_target(destination, data);
            return 4;
        }
        Instruction::LoadNNR(source) => {
            let nn: u16 = self.get_nn();
            let data: u8 = self.get_register_target(source);
            self.memory.write_byte(nn, data);
            return 4;
        }
        Instruction::LoadRHighR(destination, offset) => {
            let offset: u8 = self.get_register_target(offset);
            let data: u8 = self.memory.read_byte(0xFF00_u16 + offset as u16);
            self.set_register_target(destination, data);
            return 2;
        }
        Instruction::LoadHighRR(offset, source) => {
            let offset: u8 = self.get_register_target(offset);
            let data: u8 = self.get_register_target(source);
            self.memory.write_byte(0xFF00_u16 + offset as u16, data);
            return 2;
        }
        Instruction::LoadRHighN(destination) => {
            let offset: u8 = self.get_n();
            let data: u8 = self.memory.read_byte(0xFF00_u16 + offset as u16);
            self.set_register_target(destination, data);
            return 3;
        }
        Instruction::LoadHighNR(source) => {
            let offset: u8 = self.get_n();
            let data: u8 = self.get_register_target(source);
            self.memory.write_byte(0xFF00_u16 + offset as u16, data);
            return 3;
        }
        Instruction::LoadRRNN(destination) => {
            let data = self.get_nn();
            self.set_double_register_target(destination, data);
            return 3;
        }
        Instruction::LoadNNSP() => {
            let destination = self.get_nn();
            self.memory.write_byte(destination, (self.sp & 0x00FF) as u8);
            self.memory.write_byte(destination + 1, (self.sp & 0xFF00) as u8);
            return 5;
        }
        Instruction::LoadSPNN() => {
            let data = self.get_nn();
            self.sp = data;
            return 3;
        }
        Instruction::LoadSPRR(source) => {
            self.sp = self.get_double_register_target(source);
            return 2;
        }
        Instruction::LoadRRSPn(destination) => {
            let n: u8 = self.get_n();
            // parens make sure that you never get overflow or underflow unless instruction is bad
            let data: u16 = ((self.sp as i32) + (n as i8) as i32) as u16;
            self.set_double_register_target(destination, data);
            self.registers.clear_zero();
            self.registers.clear_subtract();
            return 3;
        }
        Instruction::ADDSPn() => {
            let n: u8 = self.get_n();
            // parens make sure that you never get overflow or underflow unless instruction is bad
            let new_value: u16 = ((self.sp as i32) + (n as i8) as i32) as u16;
            self.sp = new_value;
            self.registers.clear_zero();
            self.registers.clear_subtract();
            return 4;
        }
        Instruction::PushRR(source) => {
            self.push(self.get_double_register_target(source));
            return 4;
        }
        Instruction::PopRR(destination) => {
            let data: u16 = self.pop();
            self.set_double_register_target(destination, data);
            return 3;
        }
        Instruction::Reset(bit_index, target) => {
            if bit_index > 7 {panic!("bad bit index passed to Reset instruction");}
            let bit_finder: u8 = 1 << bit_index;
            let target_value: u8 = self.get_register_target(target);
            if target_value & bit_finder > 0 {
                self.set_register_target(target, target_value - bit_finder);
            }
            return 1;
        }
        Instruction::ResetMem(bit_index, mem_target) => {
            if bit_index > 7 {panic!("bad bit index passed to Reset instruction");}
            let bit_finder: u8 = 1 << bit_index;
            let target_value: u8 = self.get_memory_target(mem_target);
            if target_value & bit_finder > 0 {
                self.set_memory_target(mem_target, target_value - bit_finder);
            }
            return 3;
        }
        Instruction::Set(bit_index, target) => {
            if bit_index > 7 {panic!("bad bit index passed to Set instruction");}
            let bit_finder: u8 = 1 << bit_index;
            let target_value: u8 = self.get_register_target(target);
            self.set_register_target(target, target_value | bit_finder);
            return 1;
        }
        Instruction::SetMem(bit_index, target_address) => {
            if bit_index > 7 {panic!("bad bit index passed to Set instruction");}
            let bit_finder: u8 = 1 << bit_index;
            let target_value: u8 = self.get_memory_target(target_address);
            self.set_memory_target(target_address, target_value | bit_finder);
            return 3;
        }
        Instruction::BitCopy(bit_index, source) => {
            if bit_index > 7 {panic!("bad bit index passed to Bit Copy instruction");}
            let bit_finder: u8 = 1 << bit_index;
            if self.get_register_target(source) & bit_finder > 0 {self.registers.clear_zero();} else {self.registers.flag_zero();}
            self.registers.clear_subtract();
            self.registers.flag_half_carry();
            return 1;
        }
        Instruction::BitCopyMem(bit_index, source_address) => {
            if bit_index > 7 {panic!("bad bit index passed to Bit Copy instruction");}
            let bit_finder: u8 = 1 << bit_index;
            if self.get_memory_target(source_address) & bit_finder > 0 {self.registers.clear_zero();} else {self.registers.flag_zero();}
            self.registers.clear_subtract();
            self.registers.flag_half_carry();
            return 2;
        }
        Instruction::LeftShift(operation, target) => {
            let value: u8 = self.get_register_target(target);
            let carry_out: u8 = (value & (1 << 7)) >> 7;
            let mut new_value: u8 = value << 1; // rust naturally left truncates
            match operation {
                ShiftOp::IncludeCarry => {
                    new_value += self.registers.get_carry();
                }
                ShiftOp::Rotate => {
                    new_value += carry_out;
                }
                _ => {
                    // value is already correct
                }
            }
            self.set_register_target(target, new_value);
            if carry_out == 1 {self.registers.flag_carry();} else {self.registers.clear_carry();}
            if new_value == 0 {self.registers.flag_zero();} else {self.registers.clear_zero();}
            self.registers.clear_subtract();
            self.registers.clear_half_carry();
            return 1;
        }
        Instruction::LeftShiftMem(operation, target) => {
            let value: u8 = self.get_memory_target(target);
            let carry_out: u8 = (value & (1 << 7)) >> 7;
            let mut new_value: u8 = value << 1; // rust naturally left truncates
            match operation {
                ShiftOp::IncludeCarry => {
                    new_value += self.registers.get_carry();
                }
                ShiftOp::Rotate => {
                    new_value += carry_out;
                }
                _ => {
                    // value is already correct
                }
            }
            self.set_memory_target(target, new_value);
            if carry_out == 1 {self.registers.flag_carry();} else {self.registers.clear_carry();}
            if new_value == 0 {self.registers.flag_zero();} else {self.registers.clear_zero();}
            self.registers.clear_subtract();
            self.registers.clear_half_carry();
            return 3;
        }
        Instruction::RightShift(operation, target) => {
            let value: u8 = self.get_register_target(target);
            let carry_out: u8 = value & 1;
            let mut new_value: u8 = value >> 1; // rust naturally left truncates
            match operation {
                ShiftOp::Arithmetic => {
                    new_value += (new_value & (1 << 6)) << 1;
                }
                ShiftOp::IncludeCarry => {
                    new_value += self.registers.get_carry() << 7;
                }
                ShiftOp::Rotate => {
                    new_value += carry_out << 7;
                }
                ShiftOp::Logical => {
                    // value is already correct
                }
            }
            self.set_register_target(target, new_value);
            if carry_out == 1 {self.registers.flag_carry();} else {self.registers.clear_carry();}
            if new_value == 0 {self.registers.flag_zero();} else {self.registers.clear_zero();}
            self.registers.clear_subtract();
            self.registers.clear_half_carry();
            return 1;
        }
        Instruction::RightShiftMem(operation, target) => {
            let value: u8 = self.get_memory_target(target);
            let carry_out: u8 = value & 1;
            let mut new_value: u8 = value >> 1; // rust naturally left truncates
            match operation {
                ShiftOp::Arithmetic => {
                    new_value += (new_value & (1 << 6)) << 1;
                }
                ShiftOp::IncludeCarry => {
                    new_value += self.registers.get_carry() << 7;
                }
                ShiftOp::Rotate => {
                    new_value += carry_out << 7;
                }
                ShiftOp::Logical => {
                    // value is already correct
                }
            }
            self.set_memory_target(target, new_value);
            if carry_out == 1 {self.registers.flag_carry();} else {self.registers.clear_carry();}
            if new_value == 0 {self.registers.flag_zero();} else {self.registers.clear_zero();}
            self.registers.clear_subtract();
            self.registers.clear_half_carry();
            return 3;
        }
        Instruction::Swap(target) => {
            let value: u8 = self.get_register_target(target);
            let lsb: u8 = value & 0x0F;
            let msb: u8 = value & 0xF0;
            self.set_register_target(target, (lsb << 4) | msb);
            self.registers.clear_subtract();
            self.registers.clear_half_carry();
            self.registers.clear_carry();
            return 1;
        }
        Instruction::SwapMem(target) => {
            let value: u8 = self.get_memory_target(target);
            let lsb: u8 = value & 0x0F;
            let msb: u8 = value & 0xF0;
            self.set_memory_target(target, (lsb << 4) | msb);
            self.registers.clear_subtract();
            self.registers.clear_half_carry();
            self.registers.clear_carry();
            return 3;
        }
        Instruction::DI() => {
            return 1;
        }
      }
    }

    // get register from enum
    fn get_register_target(&self, target: RegisterTarget) -> u8 {
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

    // set register from enum
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

    fn get_double_register_target(&self, target: DoubleRegisterTarget) -> u16 {
        match target {
            DoubleRegisterTarget::AF => {
                return self.registers.get_af();
            }
            DoubleRegisterTarget::BC => {
                return self.registers.get_bc();
            }
            DoubleRegisterTarget::DE => {
                return self.registers.get_de();
            }
            DoubleRegisterTarget::HL => {
                return self.registers.get_hl();
            }
        }
    }

    fn set_double_register_target(&mut self, target: DoubleRegisterTarget, value: u16) {
        match target {
            DoubleRegisterTarget::AF => {
                self.registers.set_af(value);
            }
            DoubleRegisterTarget::BC => {
                self.registers.set_bc(value);
            }
            DoubleRegisterTarget::DE => {
                self.registers.set_de(value);
            }
            DoubleRegisterTarget::HL => {
                self.registers.set_hl(value);
            }
        }
    }

    // get memory byte from location specified by the double register from enum
    fn get_memory_target(&self, target: DoubleRegisterTarget) -> u8 {
        match target {
            DoubleRegisterTarget::AF => {
                return self.memory.read_byte(self.registers.get_af());
            }
            DoubleRegisterTarget::BC => {
                return self.memory.read_byte(self.registers.get_bc());
            }
            DoubleRegisterTarget::DE => {
                return self.memory.read_byte(self.registers.get_de());
            }
            DoubleRegisterTarget::HL => {
                return self.memory.read_byte(self.registers.get_hl());
            }
        }
    }

    // set memory byte at location specified by the double register from enum
    fn set_memory_target(&mut self, target: DoubleRegisterTarget, value: u8) {
        match target {
            DoubleRegisterTarget::AF => {
                self.memory.write_byte(self.registers.get_af(), value);
            }
            DoubleRegisterTarget::BC => {
                self.memory.write_byte(self.registers.get_bc(), value);
            }
            DoubleRegisterTarget::DE => {
                self.memory.write_byte(self.registers.get_de(), value);
            }
            DoubleRegisterTarget::HL => {
                self.memory.write_byte(self.registers.get_hl(), value);
            }
        }
    }

    // get the next byte after PC (increments PC)
    fn get_n(&mut self) -> u8 {
        let data: u8 = self.memory.read_byte(self.pc);
        self.pc += 1;
        return data;
    }

    // get the next two bytes after PC (increments PC twice) and return as 16 bit little endian number
    fn get_nn(&mut self) -> u16 {
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
        self.memory.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);
    
        self.sp = self.sp.wrapping_sub(1);
        self.memory.write_byte(self.sp, (value & 0xFF) as u8);
    }

    // pop from the stack
    fn pop(&mut self) -> u16 {
        self.sp = self.sp.wrapping_add(1);
        let lsb = self.memory.read_byte(self.sp) as u16;
    
        self.sp = self.sp.wrapping_add(1);
        let msb = self.memory.read_byte(self.sp) as u16;
    
        return (msb << 8) | lsb;
      }
    
    // applies the post op to the byte in memory at the location specified by the double register enum
    fn do_post_op(&mut self, target: DoubleRegisterTarget, post_op: PostOp) {
        // we don't use add and sub here because we're not setting flags
        let data = self.get_double_register_target(target);
        match post_op {
            PostOp::Nop => {
            }
            PostOp::Increment => {
                let (new_val, _) = data.overflowing_add(1);
                self.set_double_register_target(target, new_val);
            }
            PostOp::Decrement => {
                let (new_val, _) = data.overflowing_sub(1);
                self.set_double_register_target(target, new_val);
            }
        }
    }
}
  
  