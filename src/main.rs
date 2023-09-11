mod cpu;

use cpu::CPU;
use std::env;
use std::process::exit;
use ctrlc;

fn main() {
    // despite alleged default handling for SIGINT, I needed this for it to work
    ctrlc::set_handler(move || {
        exit(1);
    }).expect("Error setting Ctrl-C handler");

    let args: Vec<String> = env::args().collect();

    let mut cpu: CPU = CPU::initialize(args[1].clone());

    let mut step_counter: usize = 0;

    loop {
        step_counter += cpu.step();
        if cpu.memory.read_byte(0xFF02) == 0x81 {
            print!("{}", std::str::from_utf8(&[cpu.memory.read_byte(0xFF01)]).unwrap());
        }
    }
}
