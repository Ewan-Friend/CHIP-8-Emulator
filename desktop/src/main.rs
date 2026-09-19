use chip8_core::*;

use std::env;
use std::fs::File;
use std::io::Read;
use std::time::Duration;
use std::thread;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::keyboard::Keycode;

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

const TICKS_PER_FRAME: usize = 10;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("cargo run path/to/your/ROM");
        return;
    }
    let filename = &args[1];

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Chip-8 emu", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
//    canvas.clear();
//    canvas.present();

    let mut chip8 = Chip8::new();
    let mut rom = File::open(filename).expect("Something went wrong when opening the file");
    let mut buffer: Vec<u8> = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();
    chip8.load(&buffer);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'gameloop: loop {
        for e in event_pump.poll_iter() {
            match e {
                Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::ESCAPE), .. }=> {
                    break 'gameloop;
                },
                Event::KeyDown { keycode: Some(key), ..} => {
                    if let Some(key) = keypress_value(key) {
                        chip8.keypress(key as usize, true);
                    }
                } 
                Event::KeyUp { keycode: Some(key), ..} => {
                    if let Some(key) = keypress_value(key) {
                        chip8.keypress(key as usize, false);
                    }
                } 
                _ => ()
            }
        }

        for _ in 0..TICKS_PER_FRAME {
            chip8.tick();
            thread::sleep(Duration::from_millis(2));
        }
        chip8.timer_tick();
        draw(&chip8, &mut canvas);
    }
}

fn draw(emu: &Chip8, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let buffer = emu.get_display();

    canvas.set_draw_color(Color::RGB(255, 255, 255));

    for (i, pixel) in buffer.iter().enumerate() {
        if *pixel {
            let x = (i % SCREEN_WIDTH) as u32;
            let y = (i / SCREEN_WIDTH) as u32;

            let r = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
            canvas.fill_rect(r).unwrap();
        }
    }
    canvas.present();
}

fn keypress_value(key: Keycode) -> Option<u8> {
   match key {
        Keycode::NUM_1 => Some(0x1),
        Keycode::NUM_2 => Some(0x2),
        Keycode::NUM_3 => Some(0x3),
        Keycode::NUM_4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::BACKSLASH => Some(0xA),
        Keycode::Z => Some(0x0),
        Keycode::X => Some(0xB),
        Keycode::C => Some(0xF),
        _ => None
   } 
}
