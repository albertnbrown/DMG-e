mod runtime;

use runtime::Runtime;
use std::env;
use std::process::exit;
use ctrlc;

fn main() {
    ctrlc::set_handler(move || {
        exit(1);
    }).expect("Error setting Ctrl-C handler");

    let args: Vec<String> = env::args().collect();

    let mut runtime: Runtime = Runtime::initialize(args[1].clone());

    runtime.run();
}
