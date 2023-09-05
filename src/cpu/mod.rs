mod registers;
mod memory;

use registers::Registers;
use memory::MemoryBus;

struct CPU {
    registers: Registers,
    pc: u16,
    bus: MemoryBus,
}
  
  