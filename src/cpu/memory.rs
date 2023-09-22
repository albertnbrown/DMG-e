use std::fs;
use std::path::Path;

pub struct Memory {
    memory: [u8; 0x10000]
}
  
impl Memory {
    pub fn initialize(file_name: String) -> Memory {
        let mut memory: [u8; 0x10000] = [0; 0x10000];

        let string_path: String = format!("./{}", file_name);
        let filepath: &Path = Path::new(&string_path);

        assert!(filepath.exists(), "{:#?}", filepath.display());

        let contents: Vec<u8> = fs::read(
            filepath,
        ).expect(
            &format!("path {} to file not found", filepath.display()),
        );

        let program_slice: &mut [u8] = &mut memory[0x0000 .. contents.len()];
        program_slice.iter_mut().enumerate().for_each(|(index, slot)| {
            *slot = contents[index];
        });

        return Memory { memory };
    }

    pub fn print_range(&mut self, start: usize, len: usize) {
        let slice = &mut self.memory[start..start+len];
        slice.iter_mut().for_each(|byte|  print!("{:x} ", byte));
        println!();
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        return self.memory[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
        
        if address >= 0xC000 && address <= 0xDDFF {
            self.memory[address as usize + 0x2000] = value;
        }
    }
}