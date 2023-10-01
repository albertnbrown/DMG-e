#[derive(Clone, Copy, Debug)]
pub enum Instruction {
  // non-prefixed instructions
  NOP(), // does nothing
  ADD(RegisterTarget, bool), // add register target to register A with bool flag for including carry
  ADDmem(DoubleRegisterTarget, bool), // add value at memory target to register A with bool flag for including carry
  ADDn(bool), // add immediate byte to register A (increments pc) with bool flag for including carry
  ADD16(DoubleRegisterTarget), // adds the double register to the double register HL
  ADD16SP(), // adds the stack pointer to the double register HL
  SUB(RegisterTarget, bool), // subtract register target from register A with bool flag for including carry
  SUBmem(DoubleRegisterTarget, bool), // subtract value at memory target from register A with bool flag for including carry
  SUBn(bool), // subtract immediate byte from register A (increments pc) with bool flag for including carry
  CP(RegisterTarget), // compares register target to register a
  CPmem(DoubleRegisterTarget), // compares value at memory target to register a
  CPn(), // compares immediate byte to register A (increments pc)
  INC(RegisterTarget), // increments register target
  INCmem(DoubleRegisterTarget), // increments value at memory target
  INC16(DoubleRegisterTarget), // increment the 16 bit value in the double register
  INCSP(), // increment the stack pointer
  DEC(RegisterTarget), // decrements register target
  DECmem(DoubleRegisterTarget), // decrements value at memory target
  DEC16(DoubleRegisterTarget), // decrement the 16 bit value in the double register
  DECSP(), // decrement the stack pointer
  AND(RegisterTarget), // logical & between register target and register a, store in register a
  ANDmem(DoubleRegisterTarget), // logical & between value at memory target and register a, store in register a
  ANDn(), // logical & between immediate byte and register a, store in register a
  XOR(RegisterTarget), // logical ^ between register target and register a, store in register a
  XORmem(DoubleRegisterTarget), // logical ^ between value at memory target and register a, store in register a
  XORn(), // logical ^ between immediate byte and register a, store in register a
  OR(RegisterTarget), // logical | between register target and register a, store in register a
  ORmem(DoubleRegisterTarget), // logical | between value at memory target and register a, store in register a
  ORn(), // logical | between immediate byte and register a, store in register a
  CCF(), // flip carry flag
  SCF(), // set carry flag to true
  CPL(), // complement register a
  DAA(), // retain binary decimal form after adding or subtracting binary decimals
  JumpNN(Conditional), // jump to the memory address stored in nn
  JumpHL(), // jump to the pc stored in HL
  JumpRn(Conditional), // jump relatively by the signed amount stored in n
  CallNN(Conditional), // push pc to stack and jump to nn
  Return(Conditional), // pop a pc from the stack and jump to it
  CallI(InvariantFunction), // pop a pc from the stack and jump to it
  LoadRR(RegisterTarget, RegisterTarget), // load data from the second register into the first register
  LoadRN(RegisterTarget), // load the immediate data n into the specified register
  LoadRMem(RegisterTarget, DoubleRegisterTarget, PostOp), // load the data at the memory target into the specified register and then perform postop on memory target
  LoadMemR(DoubleRegisterTarget, RegisterTarget, PostOp), // write the data in the register to the memory target and then perform postop on memory target
  LoadMemN(DoubleRegisterTarget), // write the data in n to the memory target
  LoadRNN(RegisterTarget), // write the data in the memory specified by nn to the register target
  LoadNNR(RegisterTarget), // write the data the register target to the memory specified by nn
  LoadRHighR(RegisterTarget, RegisterTarget), // write to the first register the data stored in 0xFF00 + the value in the second register
  LoadHighRR(RegisterTarget, RegisterTarget), // write to the memory in 0xFF00 + the value in the second register to the first register
  LoadRHighN(RegisterTarget), // write to the first register the data stored in 0xFF00 + n
  LoadHighNR(RegisterTarget), // write to the memory in 0xFF00 + n to the first register
  LoadRRNN(DoubleRegisterTarget), // load nn into the double register
  LoadNNSP(), // put the data at the stack pointer in the memory denoted at the register indexed by nn
  LoadSPNN(), // put the immediate data nn into the stack pointer
  LoadSPRR(DoubleRegisterTarget), // set the stack pointer to the double register
  LoadRRSPn(DoubleRegisterTarget), // set the double register equal to the stack pointer plus the signed immediate data n
  ADDSPn(), // adds the signed immediate data n to the stack pointer
  PushRR(DoubleRegisterTarget), // push the double register value to the stack
  PopRR(DoubleRegisterTarget), // pop from the stack to the double register

  DI(), // disable interrupts

  // prefixed instructions
  Reset(u8, RegisterTarget), // set the bit indexed by the first parameter on the register target to zero
  ResetMem(u8, DoubleRegisterTarget), // set the bit indexed by the first parameter on the byte in memory indexed by the double register to zero
  Set(u8, RegisterTarget), // set the bit indexed by the first parameter on the register target to one
  SetMem(u8, DoubleRegisterTarget), // set the bit indexed by the first parameter on the byte in memory indexed by the double register to one
  BitCopy(u8, RegisterTarget), // copy the complement of the bit indexed by the first parameter of the register target into the zero flag
  BitCopyMem(u8, DoubleRegisterTarget), // copy the complement of the bit indexed by the first parameter of the byte in memory indexed by the double register into the zero flag
  LeftShift(ShiftOp, RegisterTarget), // shift acts on the register target
  LeftShiftMem(ShiftOp, DoubleRegisterTarget), // shift acts on the memory at the double register address
  RightShift(ShiftOp, RegisterTarget), // shift acts on the register target
  RightShiftMem(ShiftOp, DoubleRegisterTarget), // shift acts on the memory at the double register address
  Swap(RegisterTarget), // swaps first 4 bits for last 4 bits in the register
  SwapMem(DoubleRegisterTarget), // swaps first 4 bits for last 4 bits in the 
}

#[derive(Clone, Copy, Debug)]
pub enum PostOp {
  Nop, Increment, Decrement,
}

#[derive(Clone, Copy, Debug)]
pub enum ShiftOp {
  Rotate, RotateZ, IncludeCarry, IncludeCarryZ, Arithmetic, Logical
}

#[derive(Clone, Copy, Debug)]
pub enum Conditional {
  ZeroFlag,
  NotZeroFlag,
  CarryFlag,
  NotCarryFlag,
  Unconditional,
}

#[derive(Clone, Copy, Debug)]
pub enum InvariantFunction {
  F00, F08, F10, F18, F20, F28, F30, F38,
}

#[derive(Clone, Copy, Debug)]
pub enum RegisterTarget {
  A, B, C, D, E, H, L,
}

#[derive(Clone, Copy, Debug)]
pub enum DoubleRegisterTarget {
  AF, BC, DE, HL,
}

// impl fmt::Display for Instruction {
//   #[inline]
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//       write!(f, "{}", self)
//   }
// }

impl Instruction {
  pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
    if prefixed {
      Instruction::from_byte_prefixed(byte)
    } else {
      Instruction::from_byte_not_prefixed(byte)
    }
  }

  fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
    match byte {
      0x00 => Some(Instruction::NOP()),
      0x01 => Some(Instruction::LoadRRNN(DoubleRegisterTarget::BC)),
      0x02 => Some(Instruction::LoadMemR(DoubleRegisterTarget::BC, RegisterTarget::A, PostOp::Nop)),
      0x03 => Some(Instruction::INC16(DoubleRegisterTarget::BC)),
      0x04 => Some(Instruction::INC(RegisterTarget::B)),
      0x05 => Some(Instruction::DEC(RegisterTarget::B)),
      0x06 => Some(Instruction::LoadRN(RegisterTarget::B)),
      0x07 => Some(Instruction::LeftShift(ShiftOp::RotateZ, RegisterTarget::A)),
      0x08 => Some(Instruction::LoadNNSP()),
      0x09 => Some(Instruction::ADD16(DoubleRegisterTarget::BC)),
      0x0A => Some(Instruction::LoadRMem(RegisterTarget::A, DoubleRegisterTarget::BC, PostOp::Nop)),
      0x0B => Some(Instruction::DEC16(DoubleRegisterTarget::BC)),
      0x0C => Some(Instruction::INC(RegisterTarget::C)),
      0x0D => Some(Instruction::DEC(RegisterTarget::C)),
      0x0E => Some(Instruction::LoadRN(RegisterTarget::C)),
      0x0F => Some(Instruction::RightShift(ShiftOp::RotateZ, RegisterTarget::A)),
      
      0x11 => Some(Instruction::LoadRRNN(DoubleRegisterTarget::DE)),
      0x12 => Some(Instruction::LoadMemR(DoubleRegisterTarget::DE, RegisterTarget::A, PostOp::Nop)),
      0x13 => Some(Instruction::INC16(DoubleRegisterTarget::DE)),
      0x14 => Some(Instruction::INC(RegisterTarget::D)),
      0x15 => Some(Instruction::DEC(RegisterTarget::D)),
      0x16 => Some(Instruction::LoadRN(RegisterTarget::D)),
      0x17 => Some(Instruction::LeftShift(ShiftOp::IncludeCarryZ, RegisterTarget::A)),
      0x18 => Some(Instruction::JumpRn(Conditional::Unconditional)),
      0x19 => Some(Instruction::ADD16(DoubleRegisterTarget::DE)),
      0x1A => Some(Instruction::LoadRMem(RegisterTarget::A, DoubleRegisterTarget::DE, PostOp::Nop)),
      0x1B => Some(Instruction::DEC16(DoubleRegisterTarget::DE)),
      0x1C => Some(Instruction::INC(RegisterTarget::E)),
      0x1D => Some(Instruction::DEC(RegisterTarget::E)),
      0x1E => Some(Instruction::LoadRN(RegisterTarget::E)),
      0x1F => Some(Instruction::RightShift(ShiftOp::IncludeCarryZ, RegisterTarget::A)),
      
      0x20 => Some(Instruction::JumpRn(Conditional::NotZeroFlag)),
      0x21 => Some(Instruction::LoadRRNN(DoubleRegisterTarget::HL)),
      0x22 => Some(Instruction::LoadMemR(DoubleRegisterTarget::HL, RegisterTarget::A, PostOp::Increment)),
      0x23 => Some(Instruction::INC16(DoubleRegisterTarget::HL)),
      0x24 => Some(Instruction::INC(RegisterTarget::H)),
      0x25 => Some(Instruction::DEC(RegisterTarget::H)),
      0x26 => Some(Instruction::LoadRN(RegisterTarget::H)),
      0x27 => Some(Instruction::DAA()),
      0x28 => Some(Instruction::JumpRn(Conditional::ZeroFlag)),
      0x29 => Some(Instruction::ADD16(DoubleRegisterTarget::HL)),
      0x2A => Some(Instruction::LoadRMem(RegisterTarget::A, DoubleRegisterTarget::HL, PostOp::Increment)),
      0x2B => Some(Instruction::DEC16(DoubleRegisterTarget::HL)),
      0x2C => Some(Instruction::INC(RegisterTarget::L)),
      0x2D => Some(Instruction::DEC(RegisterTarget::L)),
      0x2E => Some(Instruction::LoadRN(RegisterTarget::L)),
      0x2F => Some(Instruction::CPL()),
      
      0x30 => Some(Instruction::JumpRn(Conditional::NotCarryFlag)),
      0x31 => Some(Instruction::LoadSPNN()),
      0x32 => Some(Instruction::LoadMemR(DoubleRegisterTarget::HL, RegisterTarget::A, PostOp::Decrement)),
      0x33 => Some(Instruction::INCSP()),
      0x34 => Some(Instruction::INCmem(DoubleRegisterTarget::HL)),
      0x35 => Some(Instruction::DECmem(DoubleRegisterTarget::HL)),
      0x36 => Some(Instruction::LoadMemN(DoubleRegisterTarget::HL)),
      0x37 => Some(Instruction::SCF()),
      0x38 => Some(Instruction::JumpRn(Conditional::CarryFlag)),
      0x39 => Some(Instruction::ADD16SP()),
      0x3A => Some(Instruction::LoadRMem(RegisterTarget::A, DoubleRegisterTarget::HL, PostOp::Decrement)),
      0x3B => Some(Instruction::DECSP()),
      0x3C => Some(Instruction::INC(RegisterTarget::A)),
      0x3D => Some(Instruction::DEC(RegisterTarget::A)),
      0x3E => Some(Instruction::LoadRN(RegisterTarget::A)),
      0x3F => Some(Instruction::CCF()),

      0x40 => Some(Instruction::LoadRR(RegisterTarget::B, RegisterTarget::B)),
      0x41 => Some(Instruction::LoadRR(RegisterTarget::B, RegisterTarget::C)),
      0x42 => Some(Instruction::LoadRR(RegisterTarget::B, RegisterTarget::D)),
      0x43 => Some(Instruction::LoadRR(RegisterTarget::B, RegisterTarget::E)),
      0x44 => Some(Instruction::LoadRR(RegisterTarget::B, RegisterTarget::H)),
      0x45 => Some(Instruction::LoadRR(RegisterTarget::B, RegisterTarget::L)),
      0x46 => Some(Instruction::LoadRMem(RegisterTarget::B, DoubleRegisterTarget::HL, PostOp::Nop)),
      0x47 => Some(Instruction::LoadRR(RegisterTarget::B, RegisterTarget::A)),
      0x48 => Some(Instruction::LoadRR(RegisterTarget::C, RegisterTarget::B)),
      0x49 => Some(Instruction::LoadRR(RegisterTarget::C, RegisterTarget::C)),
      0x4A => Some(Instruction::LoadRR(RegisterTarget::C, RegisterTarget::D)),
      0x4B => Some(Instruction::LoadRR(RegisterTarget::C, RegisterTarget::E)),
      0x4C => Some(Instruction::LoadRR(RegisterTarget::C, RegisterTarget::H)),
      0x4D => Some(Instruction::LoadRR(RegisterTarget::C, RegisterTarget::L)),
      0x4E => Some(Instruction::LoadRMem(RegisterTarget::C, DoubleRegisterTarget::HL, PostOp::Nop)),
      0x4F => Some(Instruction::LoadRR(RegisterTarget::C, RegisterTarget::A)),

      0x50 => Some(Instruction::LoadRR(RegisterTarget::D, RegisterTarget::B)),
      0x51 => Some(Instruction::LoadRR(RegisterTarget::D, RegisterTarget::C)),
      0x52 => Some(Instruction::LoadRR(RegisterTarget::D, RegisterTarget::D)),
      0x53 => Some(Instruction::LoadRR(RegisterTarget::D, RegisterTarget::E)),
      0x54 => Some(Instruction::LoadRR(RegisterTarget::D, RegisterTarget::H)),
      0x55 => Some(Instruction::LoadRR(RegisterTarget::D, RegisterTarget::L)),
      0x56 => Some(Instruction::LoadRMem(RegisterTarget::D, DoubleRegisterTarget::HL, PostOp::Nop)),
      0x57 => Some(Instruction::LoadRR(RegisterTarget::D, RegisterTarget::A)),
      0x58 => Some(Instruction::LoadRR(RegisterTarget::E, RegisterTarget::B)),
      0x59 => Some(Instruction::LoadRR(RegisterTarget::E, RegisterTarget::C)),
      0x5A => Some(Instruction::LoadRR(RegisterTarget::E, RegisterTarget::D)),
      0x5B => Some(Instruction::LoadRR(RegisterTarget::E, RegisterTarget::E)),
      0x5C => Some(Instruction::LoadRR(RegisterTarget::E, RegisterTarget::H)),
      0x5D => Some(Instruction::LoadRR(RegisterTarget::E, RegisterTarget::L)),
      0x5E => Some(Instruction::LoadRMem(RegisterTarget::E, DoubleRegisterTarget::HL, PostOp::Nop)),
      0x5F => Some(Instruction::LoadRR(RegisterTarget::E, RegisterTarget::A)),

      0x60 => Some(Instruction::LoadRR(RegisterTarget::H, RegisterTarget::B)),
      0x61 => Some(Instruction::LoadRR(RegisterTarget::H, RegisterTarget::C)),
      0x62 => Some(Instruction::LoadRR(RegisterTarget::H, RegisterTarget::D)),
      0x63 => Some(Instruction::LoadRR(RegisterTarget::H, RegisterTarget::E)),
      0x64 => Some(Instruction::LoadRR(RegisterTarget::H, RegisterTarget::H)),
      0x65 => Some(Instruction::LoadRR(RegisterTarget::H, RegisterTarget::L)),
      0x66 => Some(Instruction::LoadRMem(RegisterTarget::H, DoubleRegisterTarget::HL, PostOp::Nop)),
      0x67 => Some(Instruction::LoadRR(RegisterTarget::H, RegisterTarget::A)),
      0x68 => Some(Instruction::LoadRR(RegisterTarget::L, RegisterTarget::B)),
      0x69 => Some(Instruction::LoadRR(RegisterTarget::L, RegisterTarget::C)),
      0x6A => Some(Instruction::LoadRR(RegisterTarget::L, RegisterTarget::D)),
      0x6B => Some(Instruction::LoadRR(RegisterTarget::L, RegisterTarget::E)),
      0x6C => Some(Instruction::LoadRR(RegisterTarget::L, RegisterTarget::H)),
      0x6D => Some(Instruction::LoadRR(RegisterTarget::L, RegisterTarget::L)),
      0x6E => Some(Instruction::LoadRMem(RegisterTarget::L, DoubleRegisterTarget::HL, PostOp::Nop)),
      0x6F => Some(Instruction::LoadRR(RegisterTarget::L, RegisterTarget::A)),

      0x70 => Some(Instruction::LoadMemR(DoubleRegisterTarget::HL, RegisterTarget::B, PostOp::Nop)),
      0x71 => Some(Instruction::LoadMemR(DoubleRegisterTarget::HL, RegisterTarget::C, PostOp::Nop)),
      0x72 => Some(Instruction::LoadMemR(DoubleRegisterTarget::HL, RegisterTarget::D, PostOp::Nop)),
      0x73 => Some(Instruction::LoadMemR(DoubleRegisterTarget::HL, RegisterTarget::E, PostOp::Nop)),
      0x74 => Some(Instruction::LoadMemR(DoubleRegisterTarget::HL, RegisterTarget::H, PostOp::Nop)),
      0x75 => Some(Instruction::LoadMemR(DoubleRegisterTarget::HL, RegisterTarget::L, PostOp::Nop)),
      0x76 => None, // Halt
      0x77 => Some(Instruction::LoadMemR(DoubleRegisterTarget::HL, RegisterTarget::A, PostOp::Nop)),
      0x78 => Some(Instruction::LoadRR(RegisterTarget::A, RegisterTarget::B)),
      0x79 => Some(Instruction::LoadRR(RegisterTarget::A, RegisterTarget::C)),
      0x7A => Some(Instruction::LoadRR(RegisterTarget::A, RegisterTarget::D)),
      0x7B => Some(Instruction::LoadRR(RegisterTarget::A, RegisterTarget::E)),
      0x7C => Some(Instruction::LoadRR(RegisterTarget::A, RegisterTarget::H)),
      0x7D => Some(Instruction::LoadRR(RegisterTarget::A, RegisterTarget::L)),
      0x7E => Some(Instruction::LoadRMem(RegisterTarget::A, DoubleRegisterTarget::HL, PostOp::Nop)),
      0x7F => Some(Instruction::LoadRR(RegisterTarget::A, RegisterTarget::A)),

      0x80 => Some(Instruction::ADD(RegisterTarget::B, false)),
      0x81 => Some(Instruction::ADD(RegisterTarget::C, false)),
      0x82 => Some(Instruction::ADD(RegisterTarget::D, false)),
      0x83 => Some(Instruction::ADD(RegisterTarget::E, false)),
      0x84 => Some(Instruction::ADD(RegisterTarget::H, false)),
      0x85 => Some(Instruction::ADD(RegisterTarget::L, false)),
      0x86 => Some(Instruction::ADDmem(DoubleRegisterTarget::HL, false)),
      0x87 => Some(Instruction::ADD(RegisterTarget::A, false)),
      0x88 => Some(Instruction::ADD(RegisterTarget::B, true)),
      0x89 => Some(Instruction::ADD(RegisterTarget::C, true)),
      0x8A => Some(Instruction::ADD(RegisterTarget::D, true)),
      0x8B => Some(Instruction::ADD(RegisterTarget::E, true)),
      0x8C => Some(Instruction::ADD(RegisterTarget::H, true)),
      0x8D => Some(Instruction::ADD(RegisterTarget::L, true)),
      0x8E => Some(Instruction::ADDmem(DoubleRegisterTarget::HL, true)),
      0x8F => Some(Instruction::ADD(RegisterTarget::A, true)),
      
      0x90 => Some(Instruction::SUB(RegisterTarget::B, false)),
      0x91 => Some(Instruction::SUB(RegisterTarget::C, false)),
      0x92 => Some(Instruction::SUB(RegisterTarget::D, false)),
      0x93 => Some(Instruction::SUB(RegisterTarget::E, false)),
      0x94 => Some(Instruction::SUB(RegisterTarget::H, false)),
      0x95 => Some(Instruction::SUB(RegisterTarget::L, false)),
      0x96 => Some(Instruction::SUBmem(DoubleRegisterTarget::HL, false)),
      0x97 => Some(Instruction::SUB(RegisterTarget::A, false)),
      0x98 => Some(Instruction::SUB(RegisterTarget::B, true)),
      0x99 => Some(Instruction::SUB(RegisterTarget::C, true)),
      0x9A => Some(Instruction::SUB(RegisterTarget::D, true)),
      0x9B => Some(Instruction::SUB(RegisterTarget::E, true)),
      0x9C => Some(Instruction::SUB(RegisterTarget::H, true)),
      0x9D => Some(Instruction::SUB(RegisterTarget::L, true)),
      0x9E => Some(Instruction::SUBmem(DoubleRegisterTarget::HL, true)),
      0x9F => Some(Instruction::SUB(RegisterTarget::A, true)),
      
      0xA0 => Some(Instruction::AND(RegisterTarget::B)),
      0xA1 => Some(Instruction::AND(RegisterTarget::C)),
      0xA2 => Some(Instruction::AND(RegisterTarget::D)),
      0xA3 => Some(Instruction::AND(RegisterTarget::E)),
      0xA4 => Some(Instruction::AND(RegisterTarget::H)),
      0xA5 => Some(Instruction::AND(RegisterTarget::L)),
      0xA6 => Some(Instruction::ANDmem(DoubleRegisterTarget::HL)),
      0xA7 => Some(Instruction::AND(RegisterTarget::A)),
      0xA8 => Some(Instruction::XOR(RegisterTarget::B)),
      0xA9 => Some(Instruction::XOR(RegisterTarget::C)),
      0xAA => Some(Instruction::XOR(RegisterTarget::D)),
      0xAB => Some(Instruction::XOR(RegisterTarget::E)),
      0xAC => Some(Instruction::XOR(RegisterTarget::H)),
      0xAD => Some(Instruction::XOR(RegisterTarget::L)),
      0xAE => Some(Instruction::XORmem(DoubleRegisterTarget::HL)),
      0xAF => Some(Instruction::XOR(RegisterTarget::A)),
      
      0xB0 => Some(Instruction::OR(RegisterTarget::B)),
      0xB1 => Some(Instruction::OR(RegisterTarget::C)),
      0xB2 => Some(Instruction::OR(RegisterTarget::D)),
      0xB3 => Some(Instruction::OR(RegisterTarget::E)),
      0xB4 => Some(Instruction::OR(RegisterTarget::H)),
      0xB5 => Some(Instruction::OR(RegisterTarget::L)),
      0xB6 => Some(Instruction::ORmem(DoubleRegisterTarget::HL)),
      0xB7 => Some(Instruction::OR(RegisterTarget::A)),
      0xB8 => Some(Instruction::CP(RegisterTarget::B)),
      0xB9 => Some(Instruction::CP(RegisterTarget::C)),
      0xBA => Some(Instruction::CP(RegisterTarget::D)),
      0xBB => Some(Instruction::CP(RegisterTarget::E)),
      0xBC => Some(Instruction::CP(RegisterTarget::H)),
      0xBD => Some(Instruction::CP(RegisterTarget::L)),
      0xBE => Some(Instruction::CPmem(DoubleRegisterTarget::HL)),
      0xBF => Some(Instruction::CP(RegisterTarget::A)),
      
      0xC0 => Some(Instruction::Return(Conditional::NotZeroFlag)),
      0xC1 => Some(Instruction::PopRR(DoubleRegisterTarget::BC)),
      0xC2 => Some(Instruction::JumpNN(Conditional::NotZeroFlag)),
      0xC3 => Some(Instruction::JumpNN(Conditional::Unconditional)),
      0xC4 => Some(Instruction::CallNN(Conditional::NotZeroFlag)),
      0xC5 => Some(Instruction::PushRR(DoubleRegisterTarget::BC)),
      0xC6 => Some(Instruction::ADDn(false)),
      0xC7 => Some(Instruction::CallI(InvariantFunction::F00)),
      0xC8 => Some(Instruction::Return(Conditional::ZeroFlag)),
      0xC9 => Some(Instruction::Return(Conditional::Unconditional)),
      0xCA => Some(Instruction::JumpNN(Conditional::ZeroFlag)),
      0xCC => Some(Instruction::CallNN(Conditional::ZeroFlag)),
      0xCD => Some(Instruction::CallNN(Conditional::Unconditional)),
      0xCE => Some(Instruction::ADDn(true)),
      0xCF => Some(Instruction::CallI(InvariantFunction::F08)),
      
      0xD0 => Some(Instruction::Return(Conditional::NotCarryFlag)),
      0xD1 => Some(Instruction::PopRR(DoubleRegisterTarget::DE)),
      0xD2 => Some(Instruction::JumpNN(Conditional::NotCarryFlag)),
      0xD4 => Some(Instruction::CallNN(Conditional::NotCarryFlag)),
      0xD5 => Some(Instruction::PushRR(DoubleRegisterTarget::DE)),
      0xD6 => Some(Instruction::SUBn(false)),
      0xD7 => Some(Instruction::CallI(InvariantFunction::F10)),
      0xD8 => Some(Instruction::Return(Conditional::CarryFlag)),
      0xD9 => Some(Instruction::Return(Conditional::Unconditional)), // RETI placeholder
      0xDA => Some(Instruction::JumpNN(Conditional::CarryFlag)),
      0xDC => Some(Instruction::CallNN(Conditional::CarryFlag)),
      0xDE => Some(Instruction::SUBn(true)),
      0xDF => Some(Instruction::CallI(InvariantFunction::F18)),
      
      0xE0 => Some(Instruction::LoadHighNR(RegisterTarget::A)),
      0xE1 => Some(Instruction::PopRR(DoubleRegisterTarget::HL)),
      0xE2 => Some(Instruction::LoadHighRR(RegisterTarget::C, RegisterTarget::A)),
      0xE5 => Some(Instruction::PushRR(DoubleRegisterTarget::HL)),
      0xE6 => Some(Instruction::ANDn()),
      0xE7 => Some(Instruction::CallI(InvariantFunction::F20)),
      0xE8 => Some(Instruction::ADDSPn()),
      0xE9 => Some(Instruction::JumpHL()),
      0xEA => Some(Instruction::LoadNNR(RegisterTarget::A)),
      0xEE => Some(Instruction::XORn()),
      0xEF => Some(Instruction::CallI(InvariantFunction::F28)),
      
      0xF0 => Some(Instruction::LoadRHighN(RegisterTarget::A)),
      0xF1 => Some(Instruction::PopRR(DoubleRegisterTarget::AF)),
      0xF2 => Some(Instruction::LoadRHighR(RegisterTarget::A, RegisterTarget::C)),
      0xF3 => Some(Instruction::DI()),
      0xF5 => Some(Instruction::PushRR(DoubleRegisterTarget::AF)),
      0xF6 => Some(Instruction::ORn()),
      0xF7 => Some(Instruction::CallI(InvariantFunction::F30)),
      0xF8 => Some(Instruction::LoadRRSPn(DoubleRegisterTarget::HL)),
      0xF9 => Some(Instruction::LoadSPRR(DoubleRegisterTarget::HL)),
      0xFA => Some(Instruction::LoadRNN(RegisterTarget::A)),
      0xFB => Some(Instruction::NOP()),
      0xFE => Some(Instruction::CPn()),
      0xFF => Some(Instruction::CallI(InvariantFunction::F38)),
      _ => None
    }
  }

  fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
    match byte {
      0x00 => Some(Instruction::LeftShift(ShiftOp::Rotate, RegisterTarget::B)),
      0x01 => Some(Instruction::LeftShift(ShiftOp::Rotate, RegisterTarget::C)),
      0x02 => Some(Instruction::LeftShift(ShiftOp::Rotate, RegisterTarget::D)),
      0x03 => Some(Instruction::LeftShift(ShiftOp::Rotate, RegisterTarget::E)),
      0x04 => Some(Instruction::LeftShift(ShiftOp::Rotate, RegisterTarget::H)),
      0x05 => Some(Instruction::LeftShift(ShiftOp::Rotate, RegisterTarget::L)),
      0x06 => Some(Instruction::LeftShiftMem(ShiftOp::Rotate, DoubleRegisterTarget::HL)),
      0x07 => Some(Instruction::LeftShift(ShiftOp::Rotate, RegisterTarget::A)),
      0x08 => Some(Instruction::RightShift(ShiftOp::Rotate, RegisterTarget::B)),
      0x09 => Some(Instruction::RightShift(ShiftOp::Rotate, RegisterTarget::C)),
      0x0A => Some(Instruction::RightShift(ShiftOp::Rotate, RegisterTarget::D)),
      0x0B => Some(Instruction::RightShift(ShiftOp::Rotate, RegisterTarget::E)),
      0x0C => Some(Instruction::RightShift(ShiftOp::Rotate, RegisterTarget::H)),
      0x0D => Some(Instruction::RightShift(ShiftOp::Rotate, RegisterTarget::L)),
      0x0E => Some(Instruction::RightShiftMem(ShiftOp::Rotate, DoubleRegisterTarget::HL)),
      0x0F => Some(Instruction::RightShift(ShiftOp::Rotate, RegisterTarget::A)),

      0x10 => Some(Instruction::LeftShift(ShiftOp::IncludeCarry, RegisterTarget::B)),
      0x11 => Some(Instruction::LeftShift(ShiftOp::IncludeCarry, RegisterTarget::C)),
      0x12 => Some(Instruction::LeftShift(ShiftOp::IncludeCarry, RegisterTarget::D)),
      0x13 => Some(Instruction::LeftShift(ShiftOp::IncludeCarry, RegisterTarget::E)),
      0x14 => Some(Instruction::LeftShift(ShiftOp::IncludeCarry, RegisterTarget::H)),
      0x15 => Some(Instruction::LeftShift(ShiftOp::IncludeCarry, RegisterTarget::L)),
      0x16 => Some(Instruction::LeftShiftMem(ShiftOp::IncludeCarry, DoubleRegisterTarget::HL)),
      0x17 => Some(Instruction::LeftShift(ShiftOp::IncludeCarry, RegisterTarget::A)),
      0x18 => Some(Instruction::RightShift(ShiftOp::IncludeCarry, RegisterTarget::B)),
      0x19 => Some(Instruction::RightShift(ShiftOp::IncludeCarry, RegisterTarget::C)),
      0x1A => Some(Instruction::RightShift(ShiftOp::IncludeCarry, RegisterTarget::D)),
      0x1B => Some(Instruction::RightShift(ShiftOp::IncludeCarry, RegisterTarget::E)),
      0x1C => Some(Instruction::RightShift(ShiftOp::IncludeCarry, RegisterTarget::H)),
      0x1D => Some(Instruction::RightShift(ShiftOp::IncludeCarry, RegisterTarget::L)),
      0x1E => Some(Instruction::RightShiftMem(ShiftOp::IncludeCarry, DoubleRegisterTarget::HL)),
      0x1F => Some(Instruction::RightShift(ShiftOp::IncludeCarry, RegisterTarget::A)),

      0x20 => Some(Instruction::LeftShift(ShiftOp::Arithmetic, RegisterTarget::B)),
      0x21 => Some(Instruction::LeftShift(ShiftOp::Arithmetic, RegisterTarget::C)),
      0x22 => Some(Instruction::LeftShift(ShiftOp::Arithmetic, RegisterTarget::D)),
      0x23 => Some(Instruction::LeftShift(ShiftOp::Arithmetic, RegisterTarget::E)),
      0x24 => Some(Instruction::LeftShift(ShiftOp::Arithmetic, RegisterTarget::H)),
      0x25 => Some(Instruction::LeftShift(ShiftOp::Arithmetic, RegisterTarget::L)),
      0x26 => Some(Instruction::LeftShiftMem(ShiftOp::Arithmetic, DoubleRegisterTarget::HL)),
      0x27 => Some(Instruction::LeftShift(ShiftOp::Arithmetic, RegisterTarget::A)),
      0x28 => Some(Instruction::RightShift(ShiftOp::Arithmetic, RegisterTarget::B)),
      0x29 => Some(Instruction::RightShift(ShiftOp::Arithmetic, RegisterTarget::C)),
      0x2A => Some(Instruction::RightShift(ShiftOp::Arithmetic, RegisterTarget::D)),
      0x2B => Some(Instruction::RightShift(ShiftOp::Arithmetic, RegisterTarget::E)),
      0x2C => Some(Instruction::RightShift(ShiftOp::Arithmetic, RegisterTarget::H)),
      0x2D => Some(Instruction::RightShift(ShiftOp::Arithmetic, RegisterTarget::L)),
      0x2E => Some(Instruction::RightShiftMem(ShiftOp::Arithmetic, DoubleRegisterTarget::HL)),
      0x2F => Some(Instruction::RightShift(ShiftOp::Arithmetic, RegisterTarget::A)),

      0x30 => Some(Instruction::Swap(RegisterTarget::B)),
      0x31 => Some(Instruction::Swap(RegisterTarget::C)),
      0x32 => Some(Instruction::Swap(RegisterTarget::D)),
      0x33 => Some(Instruction::Swap(RegisterTarget::E)),
      0x34 => Some(Instruction::Swap(RegisterTarget::H)),
      0x35 => Some(Instruction::Swap(RegisterTarget::L)),
      0x36 => Some(Instruction::SwapMem(DoubleRegisterTarget::HL)),
      0x37 => Some(Instruction::Swap(RegisterTarget::A)),
      0x38 => Some(Instruction::RightShift(ShiftOp::Logical, RegisterTarget::B)),
      0x39 => Some(Instruction::RightShift(ShiftOp::Logical, RegisterTarget::C)),
      0x3A => Some(Instruction::RightShift(ShiftOp::Logical, RegisterTarget::D)),
      0x3B => Some(Instruction::RightShift(ShiftOp::Logical, RegisterTarget::E)),
      0x3C => Some(Instruction::RightShift(ShiftOp::Logical, RegisterTarget::H)),
      0x3D => Some(Instruction::RightShift(ShiftOp::Logical, RegisterTarget::L)),
      0x3E => Some(Instruction::RightShiftMem(ShiftOp::Logical, DoubleRegisterTarget::HL)),
      0x3F => Some(Instruction::RightShift(ShiftOp::Logical, RegisterTarget::A)),

      0x40 => Some(Instruction::BitCopy(0, RegisterTarget::B)),
      0x41 => Some(Instruction::BitCopy(0, RegisterTarget::C)),
      0x42 => Some(Instruction::BitCopy(0, RegisterTarget::D)),
      0x43 => Some(Instruction::BitCopy(0, RegisterTarget::E)),
      0x44 => Some(Instruction::BitCopy(0, RegisterTarget::H)),
      0x45 => Some(Instruction::BitCopy(0, RegisterTarget::L)),
      0x46 => Some(Instruction::BitCopyMem(0, DoubleRegisterTarget::HL)),
      0x47 => Some(Instruction::BitCopy(0, RegisterTarget::A)),
      0x48 => Some(Instruction::BitCopy(1, RegisterTarget::B)),
      0x49 => Some(Instruction::BitCopy(1, RegisterTarget::C)),
      0x4A => Some(Instruction::BitCopy(1, RegisterTarget::D)),
      0x4B => Some(Instruction::BitCopy(1, RegisterTarget::E)),
      0x4C => Some(Instruction::BitCopy(1, RegisterTarget::H)),
      0x4D => Some(Instruction::BitCopy(1, RegisterTarget::L)),
      0x4E => Some(Instruction::BitCopyMem(1, DoubleRegisterTarget::HL)),
      0x4F => Some(Instruction::BitCopy(1, RegisterTarget::A)),

      0x50 => Some(Instruction::BitCopy(2, RegisterTarget::B)),
      0x51 => Some(Instruction::BitCopy(2, RegisterTarget::C)),
      0x52 => Some(Instruction::BitCopy(2, RegisterTarget::D)),
      0x53 => Some(Instruction::BitCopy(2, RegisterTarget::E)),
      0x54 => Some(Instruction::BitCopy(2, RegisterTarget::H)),
      0x55 => Some(Instruction::BitCopy(2, RegisterTarget::L)),
      0x56 => Some(Instruction::BitCopyMem(2, DoubleRegisterTarget::HL)),
      0x57 => Some(Instruction::BitCopy(2, RegisterTarget::A)),
      0x58 => Some(Instruction::BitCopy(3, RegisterTarget::B)),
      0x59 => Some(Instruction::BitCopy(3, RegisterTarget::C)),
      0x5A => Some(Instruction::BitCopy(3, RegisterTarget::D)),
      0x5B => Some(Instruction::BitCopy(3, RegisterTarget::E)),
      0x5C => Some(Instruction::BitCopy(3, RegisterTarget::H)),
      0x5D => Some(Instruction::BitCopy(3, RegisterTarget::L)),
      0x5E => Some(Instruction::BitCopyMem(3, DoubleRegisterTarget::HL)),
      0x5F => Some(Instruction::BitCopy(3, RegisterTarget::A)),

      0x60 => Some(Instruction::BitCopy(4, RegisterTarget::B)),
      0x61 => Some(Instruction::BitCopy(4, RegisterTarget::C)),
      0x62 => Some(Instruction::BitCopy(4, RegisterTarget::D)),
      0x63 => Some(Instruction::BitCopy(4, RegisterTarget::E)),
      0x64 => Some(Instruction::BitCopy(4, RegisterTarget::H)),
      0x65 => Some(Instruction::BitCopy(4, RegisterTarget::L)),
      0x66 => Some(Instruction::BitCopyMem(4, DoubleRegisterTarget::HL)),
      0x67 => Some(Instruction::BitCopy(4, RegisterTarget::A)),
      0x68 => Some(Instruction::BitCopy(5, RegisterTarget::B)),
      0x69 => Some(Instruction::BitCopy(5, RegisterTarget::C)),
      0x6A => Some(Instruction::BitCopy(5, RegisterTarget::D)),
      0x6B => Some(Instruction::BitCopy(5, RegisterTarget::E)),
      0x6C => Some(Instruction::BitCopy(5, RegisterTarget::H)),
      0x6D => Some(Instruction::BitCopy(5, RegisterTarget::L)),
      0x6E => Some(Instruction::BitCopyMem(5, DoubleRegisterTarget::HL)),
      0x6F => Some(Instruction::BitCopy(5, RegisterTarget::A)),

      0x70 => Some(Instruction::BitCopy(6, RegisterTarget::B)),
      0x71 => Some(Instruction::BitCopy(6, RegisterTarget::C)),
      0x72 => Some(Instruction::BitCopy(6, RegisterTarget::D)),
      0x73 => Some(Instruction::BitCopy(6, RegisterTarget::E)),
      0x74 => Some(Instruction::BitCopy(6, RegisterTarget::H)),
      0x75 => Some(Instruction::BitCopy(6, RegisterTarget::L)),
      0x76 => Some(Instruction::BitCopyMem(6, DoubleRegisterTarget::HL)),
      0x77 => Some(Instruction::BitCopy(6, RegisterTarget::A)),
      0x78 => Some(Instruction::BitCopy(7, RegisterTarget::B)),
      0x79 => Some(Instruction::BitCopy(7, RegisterTarget::C)),
      0x7A => Some(Instruction::BitCopy(7, RegisterTarget::D)),
      0x7B => Some(Instruction::BitCopy(7, RegisterTarget::E)),
      0x7C => Some(Instruction::BitCopy(7, RegisterTarget::H)),
      0x7D => Some(Instruction::BitCopy(7, RegisterTarget::L)),
      0x7E => Some(Instruction::BitCopyMem(7, DoubleRegisterTarget::HL)),
      0x7F => Some(Instruction::BitCopy(7, RegisterTarget::A)),
      
      0x80 => Some(Instruction::Reset(0, RegisterTarget::B)),
      0x81 => Some(Instruction::Reset(0, RegisterTarget::C)),
      0x82 => Some(Instruction::Reset(0, RegisterTarget::D)),
      0x83 => Some(Instruction::Reset(0, RegisterTarget::E)),
      0x84 => Some(Instruction::Reset(0, RegisterTarget::H)),
      0x85 => Some(Instruction::Reset(0, RegisterTarget::L)),
      0x86 => Some(Instruction::ResetMem(0, DoubleRegisterTarget::HL)),
      0x87 => Some(Instruction::Reset(0, RegisterTarget::A)),
      0x88 => Some(Instruction::Reset(1, RegisterTarget::B)),
      0x89 => Some(Instruction::Reset(1, RegisterTarget::C)),
      0x8A => Some(Instruction::Reset(1, RegisterTarget::D)),
      0x8B => Some(Instruction::Reset(1, RegisterTarget::E)),
      0x8C => Some(Instruction::Reset(1, RegisterTarget::H)),
      0x8D => Some(Instruction::Reset(1, RegisterTarget::L)),
      0x8E => Some(Instruction::ResetMem(1, DoubleRegisterTarget::HL)),
      0x8F => Some(Instruction::Reset(1, RegisterTarget::A)),

      0x90 => Some(Instruction::Reset(2, RegisterTarget::B)),
      0x91 => Some(Instruction::Reset(2, RegisterTarget::C)),
      0x92 => Some(Instruction::Reset(2, RegisterTarget::D)),
      0x93 => Some(Instruction::Reset(2, RegisterTarget::E)),
      0x94 => Some(Instruction::Reset(2, RegisterTarget::H)),
      0x95 => Some(Instruction::Reset(2, RegisterTarget::L)),
      0x96 => Some(Instruction::ResetMem(2, DoubleRegisterTarget::HL)),
      0x97 => Some(Instruction::Reset(2, RegisterTarget::A)),
      0x98 => Some(Instruction::Reset(3, RegisterTarget::B)),
      0x99 => Some(Instruction::Reset(3, RegisterTarget::C)),
      0x9A => Some(Instruction::Reset(3, RegisterTarget::D)),
      0x9B => Some(Instruction::Reset(3, RegisterTarget::E)),
      0x9C => Some(Instruction::Reset(3, RegisterTarget::H)),
      0x9D => Some(Instruction::Reset(3, RegisterTarget::L)),
      0x9E => Some(Instruction::ResetMem(3, DoubleRegisterTarget::HL)),
      0x9F => Some(Instruction::Reset(3, RegisterTarget::A)),

      0xA0 => Some(Instruction::Reset(4, RegisterTarget::B)),
      0xA1 => Some(Instruction::Reset(4, RegisterTarget::C)),
      0xA2 => Some(Instruction::Reset(4, RegisterTarget::D)),
      0xA3 => Some(Instruction::Reset(4, RegisterTarget::E)),
      0xA4 => Some(Instruction::Reset(4, RegisterTarget::H)),
      0xA5 => Some(Instruction::Reset(4, RegisterTarget::L)),
      0xA6 => Some(Instruction::ResetMem(4, DoubleRegisterTarget::HL)),
      0xA7 => Some(Instruction::Reset(4, RegisterTarget::A)),
      0xA8 => Some(Instruction::Reset(5, RegisterTarget::B)),
      0xA9 => Some(Instruction::Reset(5, RegisterTarget::C)),
      0xAA => Some(Instruction::Reset(5, RegisterTarget::D)),
      0xAB => Some(Instruction::Reset(5, RegisterTarget::E)),
      0xAC => Some(Instruction::Reset(5, RegisterTarget::H)),
      0xAD => Some(Instruction::Reset(5, RegisterTarget::L)),
      0xAE => Some(Instruction::ResetMem(5, DoubleRegisterTarget::HL)),
      0xAF => Some(Instruction::Reset(5, RegisterTarget::A)),

      0xB0 => Some(Instruction::Reset(6, RegisterTarget::B)),
      0xB1 => Some(Instruction::Reset(6, RegisterTarget::C)),
      0xB2 => Some(Instruction::Reset(6, RegisterTarget::D)),
      0xB3 => Some(Instruction::Reset(6, RegisterTarget::E)),
      0xB4 => Some(Instruction::Reset(6, RegisterTarget::H)),
      0xB5 => Some(Instruction::Reset(6, RegisterTarget::L)),
      0xB6 => Some(Instruction::ResetMem(6, DoubleRegisterTarget::HL)),
      0xB7 => Some(Instruction::Reset(6, RegisterTarget::A)),
      0xB8 => Some(Instruction::Reset(7, RegisterTarget::B)),
      0xB9 => Some(Instruction::Reset(7, RegisterTarget::C)),
      0xBA => Some(Instruction::Reset(7, RegisterTarget::D)),
      0xBB => Some(Instruction::Reset(7, RegisterTarget::E)),
      0xBC => Some(Instruction::Reset(7, RegisterTarget::H)),
      0xBD => Some(Instruction::Reset(7, RegisterTarget::L)),
      0xBE => Some(Instruction::ResetMem(7, DoubleRegisterTarget::HL)),
      0xBF => Some(Instruction::Reset(7, RegisterTarget::A)),
      
      0xC0 => Some(Instruction::Set(0, RegisterTarget::B)),
      0xC1 => Some(Instruction::Set(0, RegisterTarget::C)),
      0xC2 => Some(Instruction::Set(0, RegisterTarget::D)),
      0xC3 => Some(Instruction::Set(0, RegisterTarget::E)),
      0xC4 => Some(Instruction::Set(0, RegisterTarget::H)),
      0xC5 => Some(Instruction::Set(0, RegisterTarget::L)),
      0xC6 => Some(Instruction::SetMem(0, DoubleRegisterTarget::HL)),
      0xC7 => Some(Instruction::Set(0, RegisterTarget::A)),
      0xC8 => Some(Instruction::Set(1, RegisterTarget::B)),
      0xC9 => Some(Instruction::Set(1, RegisterTarget::C)),
      0xCA => Some(Instruction::Set(1, RegisterTarget::D)),
      0xCB => Some(Instruction::Set(1, RegisterTarget::E)),
      0xCC => Some(Instruction::Set(1, RegisterTarget::H)),
      0xCD => Some(Instruction::Set(1, RegisterTarget::L)),
      0xCE => Some(Instruction::SetMem(1, DoubleRegisterTarget::HL)),
      0xCF => Some(Instruction::Set(1, RegisterTarget::A)),

      0xD0 => Some(Instruction::Set(2, RegisterTarget::B)),
      0xD1 => Some(Instruction::Set(2, RegisterTarget::C)),
      0xD2 => Some(Instruction::Set(2, RegisterTarget::D)),
      0xD3 => Some(Instruction::Set(2, RegisterTarget::E)),
      0xD4 => Some(Instruction::Set(2, RegisterTarget::H)),
      0xD5 => Some(Instruction::Set(2, RegisterTarget::L)),
      0xD6 => Some(Instruction::SetMem(2, DoubleRegisterTarget::HL)),
      0xD7 => Some(Instruction::Set(2, RegisterTarget::A)),
      0xD8 => Some(Instruction::Set(3, RegisterTarget::B)),
      0xD9 => Some(Instruction::Set(3, RegisterTarget::C)),
      0xDA => Some(Instruction::Set(3, RegisterTarget::D)),
      0xDB => Some(Instruction::Set(3, RegisterTarget::E)),
      0xDC => Some(Instruction::Set(3, RegisterTarget::H)),
      0xDD => Some(Instruction::Set(3, RegisterTarget::L)),
      0xDE => Some(Instruction::SetMem(3, DoubleRegisterTarget::HL)),
      0xDF => Some(Instruction::Set(3, RegisterTarget::A)),

      0xE0 => Some(Instruction::Set(4, RegisterTarget::B)),
      0xE1 => Some(Instruction::Set(4, RegisterTarget::C)),
      0xE2 => Some(Instruction::Set(4, RegisterTarget::D)),
      0xE3 => Some(Instruction::Set(4, RegisterTarget::E)),
      0xE4 => Some(Instruction::Set(4, RegisterTarget::H)),
      0xE5 => Some(Instruction::Set(4, RegisterTarget::L)),
      0xE6 => Some(Instruction::SetMem(4, DoubleRegisterTarget::HL)),
      0xE7 => Some(Instruction::Set(4, RegisterTarget::A)),
      0xE8 => Some(Instruction::Set(5, RegisterTarget::B)),
      0xE9 => Some(Instruction::Set(5, RegisterTarget::C)),
      0xEA => Some(Instruction::Set(5, RegisterTarget::D)),
      0xEB => Some(Instruction::Set(5, RegisterTarget::E)),
      0xEC => Some(Instruction::Set(5, RegisterTarget::H)),
      0xED => Some(Instruction::Set(5, RegisterTarget::L)),
      0xEE => Some(Instruction::SetMem(5, DoubleRegisterTarget::HL)),
      0xEF => Some(Instruction::Set(5, RegisterTarget::A)),

      0xF0 => Some(Instruction::Set(6, RegisterTarget::B)),
      0xF1 => Some(Instruction::Set(6, RegisterTarget::C)),
      0xF2 => Some(Instruction::Set(6, RegisterTarget::D)),
      0xF3 => Some(Instruction::Set(6, RegisterTarget::E)),
      0xF4 => Some(Instruction::Set(6, RegisterTarget::H)),
      0xF5 => Some(Instruction::Set(6, RegisterTarget::L)),
      0xF6 => Some(Instruction::SetMem(6, DoubleRegisterTarget::HL)),
      0xF7 => Some(Instruction::Set(6, RegisterTarget::A)),
      0xF8 => Some(Instruction::Set(7, RegisterTarget::B)),
      0xF9 => Some(Instruction::Set(7, RegisterTarget::C)),
      0xFA => Some(Instruction::Set(7, RegisterTarget::D)),
      0xFB => Some(Instruction::Set(7, RegisterTarget::E)),
      0xFC => Some(Instruction::Set(7, RegisterTarget::H)),
      0xFD => Some(Instruction::Set(7, RegisterTarget::L)),
      0xFE => Some(Instruction::SetMem(7, DoubleRegisterTarget::HL)),
      0xFF => Some(Instruction::Set(7, RegisterTarget::A)),
    }
  }
}