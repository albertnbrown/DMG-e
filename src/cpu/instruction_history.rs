use crate::cpu::instruction::Instruction;
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
        fmt::Display::fmt(&*self, f)
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