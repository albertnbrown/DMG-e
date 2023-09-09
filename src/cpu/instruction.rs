pub enum Instruction {
  ADD(RegisterTarget, Carry), // add register target to register a
  ADDmem(MemoryTarget, Carry), // add value at memory target to register a (only implements for HL)
  ADDn(Carry), // add immediate byte to register a (increments pc)
  SUB(RegisterTarget, Carry), // subtract register target from register a
  SUBmem(MemoryTarget, Carry), // subtract value at memory target from register a (only implements for HL)
  SUBn(Carry), // subtract immediate byte from register a (increments pc)
  CP(RegisterTarget), // compares register target to register a
  CPmem(MemoryTarget), // compares value at memory target to register a (only implements for HL)
  CPn(), // compares immediate byte to register a (increments pc)
  INC(RegisterTarget), // increments register target
  INCmem(MemoryTarget), // increments value at memory target
  DEC(RegisterTarget), // decrements register target
  DECmem(MemoryTarget), // decrements value at memory target
  AND(RegisterTarget), // logical & between register target and register a, store in register a
  ANDmem(MemoryTarget), // logical & between value at memory target and register a, store in register a
  ANDn(), // logical & between immediate byte and register a, store in register a
  XOR(RegisterTarget), // logical ^ between register target and register a, store in register a
  XORmem(MemoryTarget), // logical ^ between value at memory target and register a, store in register a
  XORn(), // logical ^ between immediate byte and register a, store in register a
  OR(RegisterTarget), // logical | between register target and register a, store in register a
  ORmem(MemoryTarget), // logical | between value at memory target and register a, store in register a
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
}

pub struct Carry {
  pub include_carry: bool,
}

pub enum Conditional {
  ZeroFlag,
  NotZeroFlag,
  CarryFlag,
  NotCarryFlag,
  Unconditional,
}

pub enum InvariantFunction {
  F00, F08, F10, F18, F20, F28, F30, F38,
}

#[derive(Clone, Copy)]
pub enum RegisterTarget {
  A, B, C, D, E, H, L,
}

#[derive(Clone, Copy)]
pub enum MemoryTarget {
  AF, BC, DE, HL,
}

impl Instruction {
  pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
    if prefixed {
      Instruction::from_byte_prefixed(byte)
    } else {
      Instruction::from_byte_not_prefixed(byte)
    }
  }

  fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
    match byte {
      // 0x00 => Some(Instruction::RLC(PrefixTarget::B)),
      _ => /* TODO: Add mapping for rest of instructions */ None
    }
  }

  fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
    match byte {
      0x04 => Some(Instruction::INC(RegisterTarget::B)),
      0x05 => Some(Instruction::DEC(RegisterTarget::B)),
      0x0C => Some(Instruction::INC(RegisterTarget::C)),
      0x0D => Some(Instruction::DEC(RegisterTarget::C)),
      0x14 => Some(Instruction::INC(RegisterTarget::D)),
      0x15 => Some(Instruction::DEC(RegisterTarget::D)),
      0x18 => Some(Instruction::JumpRn(Conditional::Unconditional)),
      0x1C => Some(Instruction::INC(RegisterTarget::E)),
      0x1D => Some(Instruction::DEC(RegisterTarget::E)),
      0x20 => Some(Instruction::JumpRn(Conditional::NotZeroFlag)),
      0x24 => Some(Instruction::INC(RegisterTarget::H)),
      0x25 => Some(Instruction::DEC(RegisterTarget::H)),
      0x27 => Some(Instruction::DAA()),
      0x28 => Some(Instruction::JumpRn(Conditional::ZeroFlag)),
      0x2C => Some(Instruction::INC(RegisterTarget::L)),
      0x2D => Some(Instruction::DEC(RegisterTarget::L)),
      0x2F => Some(Instruction::CPL()),
      0x30 => Some(Instruction::JumpRn(Conditional::NotCarryFlag)),
      0x34 => Some(Instruction::INCmem(MemoryTarget::HL)),
      0x35 => Some(Instruction::DECmem(MemoryTarget::HL)),
      0x37 => Some(Instruction::SCF()),
      0x38 => Some(Instruction::JumpRn(Conditional::CarryFlag)),
      0x3C => Some(Instruction::INC(RegisterTarget::A)),
      0x3D => Some(Instruction::DEC(RegisterTarget::A)),
      0x3F => Some(Instruction::CCF()),
      0x80 => Some(Instruction::ADD(RegisterTarget::B, Carry {include_carry: false})),
      0x81 => Some(Instruction::ADD(RegisterTarget::C, Carry {include_carry: false})),
      0x82 => Some(Instruction::ADD(RegisterTarget::D, Carry {include_carry: false})),
      0x83 => Some(Instruction::ADD(RegisterTarget::E, Carry {include_carry: false})),
      0x84 => Some(Instruction::ADD(RegisterTarget::H, Carry {include_carry: false})),
      0x85 => Some(Instruction::ADD(RegisterTarget::L, Carry {include_carry: false})),
      0x86 => Some(Instruction::ADDmem(MemoryTarget::HL, Carry {include_carry: false})),
      0x87 => Some(Instruction::ADD(RegisterTarget::A, Carry {include_carry: false})),
      0x88 => Some(Instruction::ADD(RegisterTarget::B, Carry {include_carry: true})),
      0x89 => Some(Instruction::ADD(RegisterTarget::C, Carry {include_carry: true})),
      0x8A => Some(Instruction::ADD(RegisterTarget::D, Carry {include_carry: true})),
      0x8B => Some(Instruction::ADD(RegisterTarget::E, Carry {include_carry: true})),
      0x8C => Some(Instruction::ADD(RegisterTarget::H, Carry {include_carry: true})),
      0x8D => Some(Instruction::ADD(RegisterTarget::L, Carry {include_carry: true})),
      0x8E => Some(Instruction::ADDmem(MemoryTarget::HL, Carry {include_carry: true})),
      0x8F => Some(Instruction::ADD(RegisterTarget::A, Carry {include_carry: true})),
      0x90 => Some(Instruction::SUB(RegisterTarget::B, Carry {include_carry: false})),
      0x91 => Some(Instruction::SUB(RegisterTarget::C, Carry {include_carry: false})),
      0x92 => Some(Instruction::SUB(RegisterTarget::D, Carry {include_carry: false})),
      0x93 => Some(Instruction::SUB(RegisterTarget::E, Carry {include_carry: false})),
      0x94 => Some(Instruction::SUB(RegisterTarget::H, Carry {include_carry: false})),
      0x95 => Some(Instruction::SUB(RegisterTarget::L, Carry {include_carry: false})),
      0x96 => Some(Instruction::SUBmem(MemoryTarget::HL, Carry {include_carry: false})),
      0x97 => Some(Instruction::SUB(RegisterTarget::A, Carry {include_carry: false})),
      0x98 => Some(Instruction::SUB(RegisterTarget::B, Carry {include_carry: true})),
      0x99 => Some(Instruction::SUB(RegisterTarget::C, Carry {include_carry: true})),
      0x9A => Some(Instruction::SUB(RegisterTarget::D, Carry {include_carry: true})),
      0x9B => Some(Instruction::SUB(RegisterTarget::E, Carry {include_carry: true})),
      0x9C => Some(Instruction::SUB(RegisterTarget::H, Carry {include_carry: true})),
      0x9D => Some(Instruction::SUB(RegisterTarget::L, Carry {include_carry: true})),
      0x9E => Some(Instruction::SUBmem(MemoryTarget::HL, Carry {include_carry: true})),
      0x9F => Some(Instruction::SUB(RegisterTarget::A, Carry {include_carry: true})),
      0xA0 => Some(Instruction::AND(RegisterTarget::B)),
      0xA1 => Some(Instruction::AND(RegisterTarget::C)),
      0xA2 => Some(Instruction::AND(RegisterTarget::D)),
      0xA3 => Some(Instruction::AND(RegisterTarget::E)),
      0xA4 => Some(Instruction::AND(RegisterTarget::H)),
      0xA5 => Some(Instruction::AND(RegisterTarget::L)),
      0xA6 => Some(Instruction::ANDmem(MemoryTarget::HL)),
      0xA7 => Some(Instruction::AND(RegisterTarget::A)),
      0xA8 => Some(Instruction::XOR(RegisterTarget::B)),
      0xA9 => Some(Instruction::XOR(RegisterTarget::C)),
      0xAA => Some(Instruction::XOR(RegisterTarget::D)),
      0xAB => Some(Instruction::XOR(RegisterTarget::E)),
      0xAC => Some(Instruction::XOR(RegisterTarget::H)),
      0xAD => Some(Instruction::XOR(RegisterTarget::L)),
      0xAE => Some(Instruction::XORmem(MemoryTarget::HL)),
      0xAF => Some(Instruction::XOR(RegisterTarget::A)),
      0xB0 => Some(Instruction::OR(RegisterTarget::B)),
      0xB1 => Some(Instruction::OR(RegisterTarget::C)),
      0xB2 => Some(Instruction::OR(RegisterTarget::D)),
      0xB3 => Some(Instruction::OR(RegisterTarget::E)),
      0xB4 => Some(Instruction::OR(RegisterTarget::H)),
      0xB5 => Some(Instruction::OR(RegisterTarget::L)),
      0xB6 => Some(Instruction::ORmem(MemoryTarget::HL)),
      0xB7 => Some(Instruction::OR(RegisterTarget::A)),
      0xB8 => Some(Instruction::CP(RegisterTarget::B)),
      0xB9 => Some(Instruction::CP(RegisterTarget::C)),
      0xBA => Some(Instruction::CP(RegisterTarget::D)),
      0xBB => Some(Instruction::CP(RegisterTarget::E)),
      0xBC => Some(Instruction::CP(RegisterTarget::H)),
      0xBD => Some(Instruction::CP(RegisterTarget::L)),
      0xBE => Some(Instruction::CPmem(MemoryTarget::HL)),
      0xBF => Some(Instruction::CP(RegisterTarget::A)),
      0xC0 => Some(Instruction::Return(Conditional::NotZeroFlag)),
      0xC2 => Some(Instruction::JumpNN(Conditional::NotZeroFlag)),
      0xC3 => Some(Instruction::JumpNN(Conditional::Unconditional)),
      0xC4 => Some(Instruction::CallNN(Conditional::NotZeroFlag)),
      0xC6 => Some(Instruction::ADDn(Carry { include_carry: false })),
      0xC7 => Some(Instruction::CallI(InvariantFunction::F00)),
      0xC8 => Some(Instruction::JumpNN(Conditional::ZeroFlag)),
      0xCE => Some(Instruction::ADDn(Carry { include_carry: true })),
      0xCF => Some(Instruction::CallI(InvariantFunction::F08)),
      0xD0 => Some(Instruction::Return(Conditional::NotCarryFlag)),
      0xD2 => Some(Instruction::JumpNN(Conditional::NotCarryFlag)),
      0xD4 => Some(Instruction::CallNN(Conditional::NotCarryFlag)),
      0xD6 => Some(Instruction::SUBn(Carry { include_carry: false })),
      0xD7 => Some(Instruction::CallI(InvariantFunction::F10)),
      0xD8 => Some(Instruction::JumpNN(Conditional::CarryFlag)),
      0xDE => Some(Instruction::SUBn(Carry { include_carry: true })),
      0xDF => Some(Instruction::CallI(InvariantFunction::F18)),
      0xE6 => Some(Instruction::ANDn()),
      0xE7 => Some(Instruction::CallI(InvariantFunction::F20)),
      0xEE => Some(Instruction::XORn()),
      0xE9 => Some(Instruction::JumpHL()),
      0xEF => Some(Instruction::CallI(InvariantFunction::F28)),
      0xF6 => Some(Instruction::ORn()),
      0xF7 => Some(Instruction::CallI(InvariantFunction::F30)),
      0xFE => Some(Instruction::CPn()),
      0xFF => Some(Instruction::CallI(InvariantFunction::F38)),
      _ => /* TODO: Add mapping for rest of instructions */ None
    }
  }
}