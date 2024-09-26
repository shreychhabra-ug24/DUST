#[no_mangle]
pub extern "C" fn rust_main() {
    run_doom();
}

use std::env;

extern "C" {
    fn D_DoomMain();
}

pub fn run_doom() {
    let args: Vec<String> = env::args().collect();
    let myargc = args.len();
    let myargv = args;

    unsafe {
        D_DoomMain();
    }
}


