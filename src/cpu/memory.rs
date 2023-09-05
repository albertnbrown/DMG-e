pub struct Memory {
    memory: [u8; 0xFFFF]
}
  
impl Memory {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}