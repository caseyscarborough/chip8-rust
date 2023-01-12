use rand;
use rand::Rng;
use std::fs::File;
use std::io::Read;
use std::process::exit;

const MEMORY_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const REGISTERS: usize = 16;
pub const VIDEO_WIDTH: usize = 64;
pub const VIDEO_HEIGHT: usize = 32;
const KEY_COUNT: usize = 16;
const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Cpu {
    pub video: [[u8; VIDEO_WIDTH]; VIDEO_HEIGHT],
    pub draw_flag: bool,
    registers: [u8; REGISTERS],
    memory: [u8; MEMORY_SIZE],
    index: usize,
    pc: usize,
    stack: [usize; STACK_SIZE],
    sp: usize,
    delay_timer: u8,
    sound_timer: u8,
    opcode: u16,
    keypad: [u8; KEY_COUNT],
    vx: usize,
    vy: usize,
    kk: u8,
    nnn: usize,
}

impl Cpu {
    pub fn init() -> Self {
        let mut memory = [0u8; MEMORY_SIZE];
        for i in 0..FONTSET.len() {
            memory[i] = FONTSET[i];
        }
        Cpu {
            registers: [0; REGISTERS],
            memory,
            index: 0,
            pc: 0x200,
            stack: [0; STACK_SIZE],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            opcode: 0,
            video: [[0u8; VIDEO_WIDTH]; VIDEO_HEIGHT],
            keypad: [0; KEY_COUNT],
            draw_flag: false,
            vx: 0,
            vy: 0,
            kk: 0,
            nnn: 0,
        }
    }

    pub fn load_rom(&mut self, filename: &str) -> bool {
        println!("Loading ROM from file: {filename}");
        let mut res = File::open(filename);
        if res.is_err() {
            return false;
        }
        let mut file = res.unwrap();
        // we can only read 3584 bytes of the 4K available because we start at address 0x200
        let mut buffer = [0u8; 3584];
        let bytes = if let Ok(bytes) = file.read(&mut buffer) {
            bytes
        } else {
            0
        };

        for (i, &byte) in buffer.iter().enumerate() {
            let addr = 0x200 + i;
            if addr < 4096 {
                self.memory[addr] = byte;
            } else {
                break;
            }
        }
        println!("Read {bytes} bytes from {filename}");
        println!("Successfully loaded ROM into memory!");
        return true;
    }

    pub fn cycle(&mut self, keypad: [u8; KEY_COUNT]) {
        self.keypad = keypad;
        self.opcode = (((self.memory[self.pc] as u16) << 8) | (self.memory[self.pc + 1] as u16));
        self.pc += 2;
        self.vx = self.get_vx();
        self.vy = self.get_vy();
        self.kk = self.get_kk();
        self.nnn = self.get_nnn();

        match self.opcode & 0xF000 {
            0x0000 => match self.opcode & 0x000F {
                0x0000 => self.op_00e0(),
                0x000E => self.op_00ee(),
                _ => self.op_null(),
            },
            0x1000 => self.op_1nnn(),
            0x2000 => self.op_2nnn(),
            0x3000 => self.op_3xkk(),
            0x4000 => self.op_4xkk(),
            0x5000 => self.op_5xy0(),
            0x6000 => self.op_6xkk(),
            0x7000 => self.op_7xkk(),
            0x8000 => match self.opcode & 0x000F {
                0x0000 => self.op_8xy0(),
                0x0001 => self.op_8xy1(),
                0x0002 => self.op_8xy2(),
                0x0003 => self.op_8xy3(),
                0x0004 => self.op_8xy4(),
                0x0005 => self.op_8xy5(),
                0x0006 => self.op_8xy6(),
                0x0007 => self.op_8xy7(),
                0x000E => self.op_8xye(),
                _ => self.op_null(),
            },
            0x9000 => self.op_9xy0(),
            0xA000 => self.op_annn(),
            0xB000 => self.op_bnnn(),
            0xC000 => self.op_cxkk(),
            0xD000 => self.op_dxyn(),
            0xE000 => match self.opcode & 0x00FF {
                0x009E => self.op_ex9e(),
                0x00A1 => self.op_exa1(),
                _ => self.op_null(),
            },
            0xF000 => match self.opcode & 0x00FF {
                0x0007 => self.op_fx07(),
                0x000A => self.op_fx0a(),
                0x0015 => self.op_fx15(),
                0x0018 => self.op_fx18(),
                0x001E => self.op_fx1e(),
                0x0029 => self.op_fx29(),
                0x0033 => self.op_fx33(),
                0x0055 => self.op_fx55(),
                0x0065 => self.op_fx65(),
                _ => self.op_null(),
            },
            _ => self.op_null()
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    fn get_vx(&mut self) -> usize {
        ((self.opcode & 0x0F00) >> 8 as u8) as usize
    }

    fn get_vy(&mut self) -> usize {
        ((self.opcode & 0x00F0) >> 4 as u8) as usize
    }

    fn get_kk(&mut self) -> u8 {
        (self.opcode & 0x00FF) as u8
    }

    fn get_nnn(&mut self) -> usize {
        (self.opcode & 0x0FFF) as usize
    }

    // clear the display
    fn op_00e0(&mut self) {
        for x in 0..VIDEO_WIDTH {
            for y in 0..VIDEO_HEIGHT {
                self.video[y][x] = 0;
            }
        }
        self.draw_flag = true;
    }

    // return from subroutine
    fn op_00ee(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp];
    }

    fn op_1nnn(&mut self) {
        self.pc = self.nnn;
    }

    fn op_2nnn(&mut self) {
        self.stack[self.sp] = self.pc;
        self.sp += 1;
        self.pc = self.nnn;
    }

    fn op_3xkk(&mut self) {
        if self.registers[self.vx] == self.kk {
            self.pc += 2;
        }
    }

    fn op_4xkk(&mut self) {
        if self.registers[self.vx] != self.kk {
            self.pc += 2;
        }
    }

    fn op_5xy0(&mut self) {
        if self.registers[self.vx] == self.registers[self.vy] {
            self.pc += 2;
        }
    }

    fn op_6xkk(&mut self) {
        self.registers[self.vx] = self.kk;
    }

    fn op_7xkk(&mut self) {
        let result = self.registers[self.vx] as u16 + self.kk as u16;
        self.registers[self.vx] = result as u8;
    }

    fn op_8xy0(&mut self) {
        self.registers[self.vx] = self.registers[self.vy];
    }

    fn op_8xy1(&mut self) {
        self.registers[self.vx] |= self.registers[self.vy];
    }

    fn op_8xy2(&mut self) {
        self.registers[self.vx] &= self.registers[self.vy];
    }

    fn op_8xy3(&mut self) {
        self.registers[self.vx] ^= self.registers[self.vy];
    }

    fn op_8xy4(&mut self) {
        let sum = self.registers[self.vx] as u16 + self.registers[self.vy] as u16;
        self.registers[0xF] = if sum > 255 {
            1
        } else {
            0
        };
        self.registers[self.vx] = (sum & 0xFF) as u8;
    }

    fn op_8xy5(&mut self) {
        self.registers[0xF] = if self.registers[self.vy] > self.registers[self.vx] {
            0
        } else {
            1
        };
        self.registers[self.vx] = self.registers[self.vx].wrapping_sub(self.registers[self.vy]);
    }

    fn op_8xy6(&mut self) {
        self.registers[0xF] = self.registers[self.vx] & 0x1;
        self.registers[self.vx] >>= 1;
    }

    fn op_8xy7(&mut self) {
        self.registers[0xF] = if self.registers[self.vy] > self.registers[self.vx] {
            1
        } else {
            0
        };
        self.registers[self.vx] = self.registers[self.vy] - self.registers[self.vx];
    }

    fn op_8xye(&mut self) {
        self.registers[0xF] = (self.registers[self.vx] & 0x80) >> 7;
        self.registers[self.vx] <<= 1;
    }

    fn op_9xy0(&mut self) {
        if self.registers[self.vx] != self.registers[self.vy] {
            self.pc += 2;
        }
    }

    fn op_annn(&mut self) {
        self.index = self.nnn;
    }

    fn op_bnnn(&mut self) {
        self.pc = (self.registers[0] as usize) + self.nnn;
    }

    fn op_cxkk(&mut self) {
        let mut rng = rand::thread_rng();
        self.registers[self.vx] = rng.gen::<u8>() & self.kk;
    }

    fn op_dxyn(&mut self) {
        self.registers[0xF] = 0;
        for byte in 0..(self.opcode & 0x000F) {
            let y = (self.registers[self.vy] as usize + byte as usize) % VIDEO_HEIGHT;
            for bit in 0..8 {
                let x = (self.registers[self.vx] as usize + bit) % VIDEO_WIDTH;
                let color = (self.memory[self.index + byte as usize] >> (7 - bit)) & 1;
                self.registers[0xF] |= color & self.video[y][x];
                self.video[y][x] ^= color;
            }
        }
        self.draw_flag = true;
    }

    fn op_ex9e(&mut self) {
        if self.keypad[self.registers[self.vx] as usize] > 0 {
            self.pc += 2;
        }
    }

    fn op_exa1(&mut self) {
        if self.keypad[self.registers[self.vx] as usize] == 0 {
            self.pc += 2;
        }
    }

    fn op_fx07(&mut self) {
        self.registers[self.vx] = self.delay_timer;
    }

    fn op_fx0a(&mut self) {
        for i in 0..16 {
            if self.keypad[i] != 0 {
                self.registers[self.vx] = i as u8;
                return;
            }
        }
        self.pc -= 2;
    }

    fn op_fx15(&mut self) {
        self.delay_timer = self.registers[self.vx];
    }

    fn op_fx18(&mut self) {
        self.sound_timer = self.registers[self.vx];
    }

    fn op_fx1e(&mut self) {
        self.registers[0xF] = if self.index + (self.registers[self.vx] as usize) > 0xFFF {
            1
        } else {
            0
        };
        self.index += self.registers[self.vx] as usize;
    }

    fn op_fx29(&mut self) {
        self.index = 5 * self.registers[self.vx] as usize;
    }

    fn op_fx33(&mut self) {
        let mut value = self.registers[self.vx];
        self.memory[self.index + 2] = value % 10;
        value /= 10;
        self.memory[self.index + 1] = value % 10;
        value /= 10;
        self.memory[self.index] = value % 10;
    }

    fn op_fx55(&mut self) {
        for i in 0..=self.vx {
            self.memory[self.index + i] = self.registers[i];
        }
        self.index = (self.vx as usize) + 1;
    }

    fn op_fx65(&mut self) {
        for i in 0..=self.vx {
            self.registers[i] = self.memory[self.index + i];
        }
        self.index = (self.vx as usize) + 1;
    }

    fn op_null(&mut self) {
        eprintln!("Unknown opcode {:#06x}", self.opcode);
        exit(1)
    }
}