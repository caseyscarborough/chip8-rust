use std::process::exit;
use std::{thread, time};
use crate::chip8::Cpu;

mod chip8;

fn main() {
    let filename = "Test-Opcodes.ch8";
    let mut cpu = Cpu::init();
    let loaded = cpu.load_rom(filename);
    if !loaded {
        eprintln!("Could not load ROM from {filename}");
        exit(1);
    }
    let mut quit = false;
    while !quit {
        cpu.cycle();

        // sleep to slow down the emulation speed
        let sleep = time::Duration::from_micros(1200);
        thread::sleep(sleep);
    }
}
