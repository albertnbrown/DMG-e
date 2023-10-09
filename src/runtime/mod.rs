pub mod cpu;
pub mod timer_control;
pub mod interrupt;

use cpu::CPU;
use timer_control::*;
use interrupt::*;

use self::cpu::memory::{TIMER_MODULO_REGISTER, TIMER_REGISTER, TIMER_CONTROL_REGISTER, INTERRUPT_REQUEST_REGISTER, INTERRUPT_ENABLE_REGISTER};


const DIV_SPEED: TimerSpeed = TimerSpeed::Clock256;

pub struct Runtime {
    cpu: CPU,
    step_counter: usize,
    master_interrupt_enabled: bool,
    delay_timer_reset: bool,
    old_tma: u8,
    debug_flag: bool,
}

impl Runtime {

    pub fn initialize(file_name: String) -> Runtime {
        return Runtime {
            cpu: CPU::initialize(file_name),
            step_counter: 0,
            master_interrupt_enabled: false,
            delay_timer_reset: false,
            old_tma: 0,
            debug_flag: false,
        }
    }

    pub fn run(&mut self) {
        loop {
            // TODO: Get rid of the old_tma pattern
            self.old_tma = self.cpu.memory.read_byte(TIMER_MODULO_REGISTER);
            let steps: usize = self.step_debug();
            self.handle_timers(steps);
            self.step_counter += steps;
        }
    }

    fn handle_timers(&mut self, steps: usize) {
        // if self.delay_timer_reset && self.cpu.memory.read_byte(TIMER_REGISTER) != 0x00 {
        //     println!("This is the reset/write timer edge case.");
        // }
        
        if !self.cpu.stopped {
            let div_increments: u8 = calc_increments(DIV_SPEED, self.step_counter, steps);
            self.cpu.memory.increment_div(div_increments);
        }

        let mut tma: u8 = self.cpu.memory.read_byte(TIMER_MODULO_REGISTER);
        if self.old_tma != tma {
            println!("OLD_TMA RELEVANT");
            tma = self.old_tma;
        }
        let mut tima: u8 = if self.delay_timer_reset {
            self.cpu.memory.flag_timer_interrrupt();
            self.delay_timer_reset = false;
            tma 
        } else {
            self.cpu.memory.read_byte(TIMER_REGISTER)
        };
        let tima_control: TimerControl = TimerControl::from(self.cpu.memory.read_byte(TIMER_CONTROL_REGISTER));
        if tima_control.enabled {
            let tima_increments: u8 = calc_increments(tima_control.speed, self.step_counter, steps);
            let (mut new_tima, overflow) = tima.overflowing_add(tima_increments);
            if overflow {
                if new_tima == 0x00 {
                    self.delay_timer_reset = true;
                } else {
                    new_tima = new_tima.wrapping_add(tma);
                    self.cpu.memory.flag_timer_interrrupt();
                }
            }
            tima = new_tima;
            self.cpu.write_byte_debug(TIMER_REGISTER, tima);
        }
    }

    fn handle_interrupts(&mut self) -> usize {
        if !self.master_interrupt_enabled { return 0; }
        let interrupt: Interrupt = self.check_interrupts();
        
        match interrupt {
            Interrupt::None => {
                return 0;
            }
            _ => {
                // println!("Interrupted!");
                self.cpu.call(u16::from(interrupt));
                self.master_interrupt_enabled = false;
                self.cpu.master_interrupt_request = false;
                let interrupt_requests = self.cpu.memory.read_byte(INTERRUPT_REQUEST_REGISTER);
                self.cpu.write_byte_debug(INTERRUPT_REQUEST_REGISTER, interrupt_requests - u8::from(interrupt));
                self.debug_flag = true;
                return 5;
            }
        }
    }

    fn check_interrupts(&self) -> Interrupt {
        return Interrupt::from(
            self.cpu.memory.read_byte(INTERRUPT_REQUEST_REGISTER) &
            self.cpu.memory.read_byte(INTERRUPT_ENABLE_REGISTER)
        )
    }

    fn step_debug(&mut self) -> usize {
        // let tma = self.cpu.memory.read_byte(TIMER_MODULO_REGISTER);
        // let tcr = self.cpu.memory.read_byte(TIMER_CONTROL_REGISTER);
        // let ifff = self.cpu.memory.read_byte(INTERRUPT_REQUEST_REGISTER);
        // let ieee = self.cpu.memory.read_byte(INTERRUPT_ENABLE_REGISTER);

        let mut steps: usize = self.cpu.step(self.step_counter);
        steps += self.handle_interrupts();
        self.master_interrupt_enabled = self.cpu.master_interrupt_request;

        // if tma != self.cpu.memory.read_byte(TIMER_MODULO_REGISTER) { self.debug_flag = true; }
        // if tcr != self.cpu.memory.read_byte(TIMER_CONTROL_REGISTER) { self.debug_flag = true; }
        // if ifff != self.cpu.memory.read_byte(INTERRUPT_REQUEST_REGISTER) { self.debug_flag = true; }
        // if ieee != self.cpu.memory.read_byte(INTERRUPT_ENABLE_REGISTER) { self.debug_flag = true; }

        if self.cpu.memory.read_byte(0xFF02) == 0x81 {
            // print!("{:x} ", self.cpu.memory.read_byte(0xFF01));
            print!("{}", std::str::from_utf8(&[self.cpu.memory.read_byte(0xFF01)]).unwrap());
            self.cpu.write_byte_debug(0xFF02, 0x00);
        }
        
        if self.debug_flag {
            // println!("~~~~~~~~~~~~~~~~~~~~~~~~~DEBUG PRINTOUT~~~~~~~~~~~~~~~~~~~~~~~~~");
            // println!("{}", self.cpu.registers.a);
            // println!("{}", self.cpu.registers.get_bc());
            // println!("{}", self.cpu.memory.read_byte(TIMER_MODULO_REGISTER));
            // println!("{}", self.cpu.memory.read_byte(TIMER_CONTROL_REGISTER));
            // self.cpu.memory.print_range(INTERRUPT_REQUEST_REGISTER as usize, 1);
            // self.cpu.memory.print_range(INTERRUPT_ENABLE_REGISTER as usize, 1);

            // steps += self.cpu.step(self.step_counter);
            // steps += self.handle_interrupts();
            // self.master_interrupt_enabled = self.cpu.master_interrupt_request;
            
            // self.cpu.print_self();
            // println!("{}", self.cpu.registers.a);
            // println!("{}", self.step_counter);
            
            // self.cpu.memory.print_range(INTERRUPT_REQUEST_REGISTER as usize, 1);
            // self.cpu.memory.print_range(INTERRUPT_ENABLE_REGISTER as usize, 1);
            // println!("{}", self.master_interrupt_enabled);
            // println!("{}", self.cpu.master_interrupt_request);
            self.debug_flag = !self.debug_flag;
        }

        // if self.cpu.master_interrupt_request {println!("master interrupt requested");}
        // if self.master_interrupt_enabled {println!("master interrupt enabled"); self.debug_flag = true;}

        // if self.cpu.pc >= 0xC000 {
        //     flag = true;
        // }

        // if self.cpu.pc < 0xC000 && flag {
        //     flag = false;
        //     self.step_debug();
        //     self.cpu.print_self();
        //     println!("{:x}", self.cpu.pc);
        // }

        // if self.step_counter % 434505 < 200 && self.step_counter % 20 < 2 && self.cpu.pc >= 0xC000 {
        //     self.debug_flag = true;
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