extern crate sdl2;

use std::process::exit;
use std::{env, thread, time};
use crate::chip8::{Cpu, VIDEO_HEIGHT, VIDEO_WIDTH};
use crate::platform::{InputDriver, Platform};

mod chip8;
mod platform;

fn main() {
    let sleep = time::Duration::from_millis(2);
    let args: Vec<String> = env::args().collect();
    let scale = args[1].parse::<u32>().unwrap();
    let delay = args[2].parse::<u32>().unwrap();
    let filename = &args[3];
    let mut cpu = Cpu::init();
    let loaded = cpu.load_rom(filename);
    if !loaded {
        eprintln!("Could not load ROM from {filename}");
        exit(1);
    }

    let sdl_context = sdl2::init().unwrap();
    let mut platform = Platform::init(&sdl_context,"Chip-8 Emulator", VIDEO_WIDTH as u32, VIDEO_HEIGHT as u32, scale);
    let mut input = InputDriver::init(&sdl_context);

    while let Ok(keypad) = input.poll() {
        cpu.cycle(keypad);
        if cpu.draw_flag {
            platform.update(&cpu.video)
        }
        // sleep to slow down the emulation speed
        thread::sleep(sleep * delay);
    }
}
