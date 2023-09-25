mod cpu;

use cpu::CPU;
use std::env;
use std::process::exit;
use ctrlc;

fn main() {
    ctrlc::set_handler(move || {
        exit(1);
    }).expect("Error setting Ctrl-C handler");

    let args: Vec<String> = env::args().collect();

    let mut cpu: CPU = CPU::initialize(args[1].clone());

    let mut step_counter: usize = 0;

    let mut flag: bool = false;

    loop {
        step_counter += cpu.step(step_counter);
        if cpu.memory.read_byte(0xFF02) == 0x81 {
            // print!("{:x} ", cpu.memory.read_byte(0xFF01));
            print!("{}", std::str::from_utf8(&[cpu.memory.read_byte(0xFF01)]).unwrap());
            cpu.memory.write_byte(0xFF02, 0x00);
        }
        
        // if cpu.pc >= 0xC000 {
        //     flag = true;
        // }

        // if cpu.pc < 0xC000 && flag {
        //     flag = false;
        //     step_counter += cpu.step(step_counter);
        //     cpu.print_self();
        //     println!("{:x}", cpu.pc);
        // }

        // if step_counter % 10100 == 0 && cpu.pc >= 0xC000 {
        //     cpu.print_self();
        //     println!("{}", step_counter);
        // }

        // if cpu.pc == 0xc62c || cpu.pc == 0xc631 {
        //     cpu.memory.print_range(0xd801 - 50, 100);
        //     cpu.memory.print_range(0xcb23 - 10, 20);
        //     cpu.memory.print_range(0xc62c - 10, 20);
        //     cpu.memory.print_range(0xdd00, 4);
        //     cpu.memory.print_range(0xc671 - 10, 20);
        //     cpu.memory.print_range(0x0000, 40);
        //     println!("^^ output ^^");
        // }
    }
}
