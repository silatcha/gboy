

pub  extern crate sdl2;
extern crate minifb;
use std::path::Path;
mod cpu;
use std::env;
use std::fs;
use std::io;
use std::io::Read;

use cpu::CPU;


use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::Sdl;
use std::time::Duration;



                    

// Window Constants
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;
const TICKS_PER_FRAME: usize = 10;
const ENLARGEMENT_FACTOR: usize = 1;
const WINDOW: [usize; 2] = [(64 * ENLARGEMENT_FACTOR), (32 * ENLARGEMENT_FACTOR)];
const NUMBER_OF_PIXELS: usize = 23040;

// Op Cycle
const ONE_SECOND_IN_MICROS: usize = 1000000000;
const ONE_SECOND_IN_CYCLES: usize = 4190000;
const ONE_FRAME_IN_CYCLES: usize = 70224;

fn main()
{

    let argv: Vec<_> = std::env::args().collect();
/* 
    if argv.len() < 2 {
        println!("Usage: {} <rom-file>", argv[0]);
        return;use std::path::Path;
    }
*/
    
 

   let args: Vec<String> = env::args().collect();

   // Check if the correct number of arguments is provided
   if args.len() != 2 {
       eprintln!("Usage: {} {} <boot_rom_path> <game_rom_path>", args[0],args.len());
       return;
   }


  let boot_rom_data = read_rom_data(&args[1]);

   let mut cpu = CPU::new(boot_rom_data);

   
      // Setup SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Chip-8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
 
     let mut display=[0 as u32;160];

       // Main emulation loop
    
       'gameloop: loop {
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..}=> {
                    break 'gameloop;
                },
                Event::KeyDown{keycode: Some(key), ..} => {
                   
                        cpu.bus.keypad.keydown(key);
                    
                },
                Event::KeyUp{keycode: Some(key), ..} => {
                    cpu.bus.keypad.keyup(key);
                },
                _ => ()
            }
        }
        cpu.bus.gpu.render_frame();
        for _ in 0..TICKS_PER_FRAME {
            cpu.step();
        }
       // chip8.tick_timers();
        draw_screen(&cpu, &mut canvas);
        
    }


    // - initialise emulator App
    // - initialise boot buffer and game buffer
    // - initialise CPU: boot/game buffer
    // - initialise Window: dimentions, options, title, etc.
    // - run emulation cycle
}

fn run()
{}



fn read_rom_data(path: &str) -> Option<[u8; 0xFFFF]> {
    // Attempt to open the file
    match fs::File::open(path) {
        Ok(mut file) => {
            let mut rom_data = [0; 0xFFFF]; // Initialize an empty array

            // Read the file content into rom_data
            match file.read_exact(&mut rom_data) {
                Ok(_) => Some(rom_data), // Successfully loaded the data
                Err(_) => None, // Failed to read the full ROM data
            }
        }
        Err(_) => None, // Failed to open the file
    }
}


fn render_graphics(canvas: &mut Canvas<Window>) {
    // Clear the canvas
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    // Render graphics or display the emulator's state
    // Example: Draw a rectangle
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.fill_rect(Rect::new(100, 100, 200, 200)).unwrap();

    // Present the canvas
    canvas.present();
}

fn draw_screen(emu: &CPU, canvas: &mut Canvas<Window>) {
    // Clear canvas as black
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buf = emu.bus.gpu.get_screen_buffer();
    // Now set draw color to white, iterate through each point and see if it should be drawn
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (i, pixel) in screen_buf.iter().enumerate() {
        if *pixel {
            // Convert our 1D array's index into a 2D (x,y) position
            let x = (i % SCREEN_WIDTH) as u32;
            let y = (i / SCREEN_WIDTH) as u32;

            // Draw a rectangle at (x,y), scaled up by our SCALE value
            let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}

fn key2btn(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 =>    Some(0x1),
        Keycode::Num2 =>    Some(0x2),
        Keycode::Num3 =>    Some(0x3),
        Keycode::Num4 =>    Some(0xC),
        Keycode::Q =>       Some(0x4),
        Keycode::W =>       Some(0x5),
        Keycode::E =>       Some(0x6),
        Keycode::R =>       Some(0xD),
        Keycode::A =>       Some(0x7),
        Keycode::S =>       Some(0x8),
        Keycode::D =>       Some(0x9),
        Keycode::F =>       Some(0xE),
        Keycode::Z =>       Some(0xA),
        Keycode::X =>       Some(0x0),
        Keycode::C =>       Some(0xB),
        Keycode::V =>       Some(0xF),
        _ =>                None,
    }
}