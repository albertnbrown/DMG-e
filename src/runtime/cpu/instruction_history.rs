use super::instruction::Instruction;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct InstructionHistory {
    pub inst: Instruction,
    pub pc: u16,
    pub spi: u32,
    pub nops: usize,
}

impl fmt::Display for InstructionHistory {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "inst {:?}, pc: 0x{:x}, spi: {}, nops: {}", self.inst, self.pc, self.spi, self.nops)
        write!(f, "inst {:?}, pc: 0x{:x}, spi: {}", self.inst, self.pc, self.spi)
    }
}

impl InstructionHistory {
    pub fn new() -> InstructionHistory {
        return InstructionHistory{
            inst:Instruction::NOP(),
            pc: 0,
            nops: 0,
            spi: 0
        };
    }
}