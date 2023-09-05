pub enum Instruction {
    ADD(RegisterTarget), // add register target to register a
}
  
pub enum RegisterTarget {
    A, B, C, D, E, H, L,
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
        0x80 => Some(Instruction::ADD(RegisterTarget::B)),
        0x81 => Some(Instruction::ADD(RegisterTarget::C)),
        0x82 => Some(Instruction::ADD(RegisterTarget::D)),
        0x83 => Some(Instruction::ADD(RegisterTarget::E)),
        0x84 => Some(Instruction::ADD(RegisterTarget::H)),
        0x85 => Some(Instruction::ADD(RegisterTarget::L)),
        0x87 => Some(Instruction::ADD(RegisterTarget::A)),
        _ => /* TODO: Add mapping for rest of instructions */ None
      }
    }
  }