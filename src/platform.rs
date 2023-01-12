extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::libc::{c_int, exit};

use crate::chip8::{VIDEO_HEIGHT, VIDEO_WIDTH};

pub struct Platform {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    scale: u32,
}

impl Platform {
    pub fn init(sdl_context: &sdl2::Sdl, title: &str, width: u32, height: u32, scale: u32) -> Self {
        let video_subsys = sdl_context.video().unwrap();
        let window = video_subsys
            .window(title, width * scale, height * scale)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        return Platform {
            canvas,
            scale,
        };
    }

    pub fn update(&mut self, pixels: &[[u8; VIDEO_WIDTH]; VIDEO_HEIGHT]) {
        for (y, row) in pixels.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x as u32) * self.scale;
                let y = (y as u32) * self.scale;

                self.canvas.set_draw_color(color(col));
                let _ = self.canvas.fill_rect(sdl2::rect::Rect::new(x as i32, y as i32, self.scale, self.scale));
            }
        }
        self.canvas.present();
    }
}

fn color(value: u8) -> sdl2::pixels::Color {
    if value == 0 {
        sdl2::pixels::Color::RGB(0, 0, 0)
    } else {
        sdl2::pixels::Color::RGB(255, 255, 255)
    }
}

pub struct InputDriver {
    events: sdl2::EventPump,
}

impl InputDriver {
    pub fn init(sdl_context: &sdl2::Sdl) -> Self {
        InputDriver { events: sdl_context.event_pump().unwrap() }
    }

    pub fn poll(&mut self) -> Result<[u8; 16], ()> {
        for event in self.events.poll_iter() {
            if let Event::Quit { .. } = event {
                return Err(());
            };
        }

        let keys: Vec<Keycode> = self.events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        let mut chip8_keys = [0; 16];

        for key in keys {
            let index = match key {
                Keycode::Num1 => Some(0x1),
                Keycode::Num2 => Some(0x2),
                Keycode::Num3 => Some(0x3),
                Keycode::Num4 => Some(0xc),
                Keycode::Q => Some(0x4),
                Keycode::W => Some(0x5),
                Keycode::E => Some(0x6),
                Keycode::R => Some(0xd),
                Keycode::A => Some(0x7),
                Keycode::S => Some(0x8),
                Keycode::D => Some(0x9),
                Keycode::F => Some(0xe),
                Keycode::Z => Some(0xa),
                Keycode::X => Some(0x0),
                Keycode::C => Some(0xb),
                Keycode::V => Some(0xf),
                _ => None,
            };

            if let Some(i) = index {
                chip8_keys[i] = 1;
            }
        }

        Ok(chip8_keys)
    }
}