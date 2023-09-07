pub struct Memory {
    memory: [u8; 0xFFFF]
}
  
impl Memory {
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn set_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}