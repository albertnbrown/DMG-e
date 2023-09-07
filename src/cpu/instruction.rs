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
}

pub struct Carry {
  pub include_carry: bool,
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
      // 0x02 => Some(Instruction::INC(IncDecTarget::BC)),
      0x04 => Some(Instruction::INC(RegisterTarget::B)),
      0x05 => Some(Instruction::DEC(RegisterTarget::B)),
      0x0C => Some(Instruction::INC(RegisterTarget::C)),
      0x0D => Some(Instruction::DEC(RegisterTarget::C)),
      0x14 => Some(Instruction::INC(RegisterTarget::D)),
      0x15 => Some(Instruction::DEC(RegisterTarget::D)),
      0x1C => Some(Instruction::INC(RegisterTarget::E)),
      0x1D => Some(Instruction::DEC(RegisterTarget::E)),
      0x24 => Some(Instruction::INC(RegisterTarget::H)),
      0x25 => Some(Instruction::DEC(RegisterTarget::H)),
      0x2C => Some(Instruction::INC(RegisterTarget::L)),
      0x2D => Some(Instruction::DEC(RegisterTarget::L)),
      0x34 => Some(Instruction::INCmem(MemoryTarget::HL)),
      0x35 => Some(Instruction::DECmem(MemoryTarget::HL)),
      0x3C => Some(Instruction::INC(RegisterTarget::A)),
      0x3D => Some(Instruction::DEC(RegisterTarget::A)),
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
      0xB8 => Some(Instruction::CP(RegisterTarget::B)),
      0xB9 => Some(Instruction::CP(RegisterTarget::C)),
      0xBA => Some(Instruction::CP(RegisterTarget::D)),
      0xBB => Some(Instruction::CP(RegisterTarget::E)),
      0xBC => Some(Instruction::CP(RegisterTarget::H)),
      0xBD => Some(Instruction::CP(RegisterTarget::L)),
      0xBE => Some(Instruction::CPmem(MemoryTarget::HL)),
      0xBF => Some(Instruction::CP(RegisterTarget::A)),
      0xC6 => Some(Instruction::ADDn(Carry { include_carry: false })),
      0xCE => Some(Instruction::ADDn(Carry { include_carry: true })),
      0xD6 => Some(Instruction::SUBn(Carry { include_carry: false })),
      0xDE => Some(Instruction::SUBn(Carry { include_carry: true })),
      0xF6 => Some(Instruction::CPn()),
      0xFE => Some(Instruction::CPn()),
      _ => /* TODO: Add mapping for rest of instructions */ None
    }
  }
}