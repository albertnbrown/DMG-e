use std::fs;
use std::path::Path;

pub const DIVIDER_REGISTER: u16 = 0xFF04;
pub const TIMER_REGISTER: u16 = 0xFF05;
pub const TIMER_MODULO_REGISTER: u16 = 0xFF06;
pub const TIMER_CONTROL_REGISTER: u16 = 0xFF07;

pub const INTERRUPT_REGISTER: usize = 0xFF0F;
// interrupt bit layout
const VBLANK_BIT: u8 = 0;
const STAT_BIT: u8 = 1;
const TIMER_BIT: u8 = 2;
const SERIAL_BIT: u8 = 3;
const JOYPAD_BIT: u8 = 4;

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
        slice.iter_mut().for_each(|byte|  print!("{:>2x} ", byte));
        println!();
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        return self.memory[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        let index = address as usize;
        match address {
            DIVIDER_REGISTER => {
                self.memory[index] = 0x00;
            }

            0xC000..=0xDDFF => {
                self.memory[index] = value;
                self.memory[index + 0x2000] = value;
            }

            0xE000..=0xFDFF => {
                self.memory[index] = value;
                self.memory[index - 0x2000] = value;
            }

            _ => {
                self.memory[index] = value;
            }
        }
    }

    pub fn increment_div(&mut self, increments: u8) {
        self.memory[DIVIDER_REGISTER as usize] = increments.wrapping_add(self.memory[DIVIDER_REGISTER as usize]);
    }

    pub fn flag_timer_interrrupt(&mut self) {
        self.memory[INTERRUPT_REGISTER] = self.memory[INTERRUPT_REGISTER] | 1<<TIMER_BIT;
    }
}