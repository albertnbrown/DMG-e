pub mod cpu;
pub mod timer_control;

use cpu::CPU;
use timer_control::*;

use self::cpu::memory::{TIMER_MODULO_REGISTER, TIMER_REGISTER, TIMER_CONTROL_REGISTER};


const DIV_SPEED: TimerSpeed = TimerSpeed::Clock256;

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
            let reset_tima: bool = self.cpu.memory.read_byte(TIMER_REGISTER) == 0x00;
            let steps: usize = self.step_debug();
            self.handle_timers(steps, reset_tima);
            self.step_counter += steps;
        }
    }

    fn handle_timers(&mut self, steps: usize, reset: bool) {
        if reset && self.cpu.memory.read_byte(TIMER_REGISTER) != 0x00 {
            println!("This is the reset/write timer edge case.");
        }
        
        if !self.cpu.stopped {
            let div_increments: u8 = ((self.step_counter + steps) % usize::from(DIV_SPEED) - self.step_counter % usize::from(DIV_SPEED)) as u8;
            self.cpu.memory.increment_div(div_increments);
        }

        let tma = self.cpu.memory.read_byte(TIMER_MODULO_REGISTER);
        let mut tima = if reset { self.cpu.memory.flag_timer_interrrupt(); tma } else { self.cpu.memory.read_byte(TIMER_REGISTER) };
        let tima_control: TimerControl = TimerControl::from(self.cpu.memory.read_byte(TIMER_CONTROL_REGISTER));
        if tima_control.enabled {
            let tima_increments = ((self.step_counter + steps) % usize::from(tima_control.speed) - self.step_counter % usize::from(tima_control.speed)) as u8;
            let (mut new_tima, overflow) = tima.overflowing_add(tima_increments);
            if overflow && new_tima > 0x00 {
                new_tima = new_tima.wrapping_add(tma);
                self.cpu.memory.flag_timer_interrrupt();
            }
            tima = new_tima;
        }
        self.cpu.memory.write_byte(TIMER_REGISTER, tima);
    }

    fn step_debug(&mut self) -> usize {
        let steps: usize = self.cpu.step(self.step_counter);
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

        return steps;
    }
}