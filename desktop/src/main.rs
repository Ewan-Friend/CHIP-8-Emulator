use chip8_core::*;
use std::env;
use sdl2::event::Event;

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("cargo run path/to/your/ROM");
        return;
    }

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Chip-8 emu", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut chip8 = Emu::new();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'gameloop: loop {
        for e in event_pump.poll_iter() {
            match e {
                Event::Quit{..} => {
                    break 'gameloop;
                },
                _ => ()
            }
        }
    }
}
