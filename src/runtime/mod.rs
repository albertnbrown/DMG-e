pub mod cpu;

use cpu::CPU;

pub struct Runtime {
    cpu: CPU,
    step_counter: usize,
}

impl Runtime {

    pub fn initialize(file_name: String) -> Runtime {
        return Runtime {
            cpu: CPU::initialize(file_name),
            step_counter: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.step_counter += self.cpu.step(self.step_counter);
            if self.cpu.memory.read_byte(0xFF02) == 0x81 {
                // print!("{:x} ", self.cpu.memory.read_byte(0xFF01));
                print!("{}", std::str::from_utf8(&[self.cpu.memory.read_byte(0xFF01)]).unwrap());
                self.cpu.memory.write_byte(0xFF02, 0x00);
            }
            
            // if self.cpu.pc >= 0xC000 {
            //     flag = true;
            // }
    
            // if self.cpu.pc < 0xC000 && flag {
            //     flag = false;
            //     step_counter += self.cpu.step(self.step_counter);
            //     self.cpu.print_self();
            //     println!("{:x}", self.cpu.pc);
            // }
    
            // if step_counter % 10100 == 0 && self.cpu.pc >= 0xC000 {
            //     self.cpu.print_self();
            //     println!("{}", self.step_counter);
            // }
    
            // if self.cpu.pc == 0xc62c || self.cpu.pc == 0xc631 {
            //     self.cpu.memory.print_range(0xd801 - 50, 100);
            //     self.cpu.memory.print_range(0xcb23 - 10, 20);
            //     self.cpu.memory.print_range(0xc62c - 10, 20);
            //     self.cpu.memory.print_range(0xdd00, 4);
            //     self.cpu.memory.print_range(0xc671 - 10, 20);
            //     self.cpu.memory.print_range(0x0000, 40);
            //     println!("^^ output ^^");
            // }
        }
    }
}