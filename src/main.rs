

extern crate sdl2;
use std::path::Path;
mod cpu;
use std::env;

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
const ENLARGEMENT_FACTOR: usize = 1;
const WINDOW: [usize; 2] = [(160 * ENLARGEMENT_FACTOR), (144 * ENLARGEMENT_FACTOR)];
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
   if args.len() != 3 {
       eprintln!("Usage: {} <boot_rom_path> <game_rom_path>", args[0]);
       return;
   }

   // Extract the file paths from command line arguments
   let boot_rom_path = &args[1];
   let game_rom_path = &args[2];

   let mut cpu = CPU::new(Some(boot_rom_path), game_rom_path);

   
     // Initialize SDL2
     let sdl_context = sdl2::init().unwrap();
     let video_subsystem = sdl_context.video().unwrap();
 
     // Set window dimensions and options
     let width = 800;
     let height = 600;
     let title = "Emulator Window";
     let window = video_subsystem
         .window(title, width, height)
         .position_centered()
         .build()
         .unwrap();
 
     // Create a Canvas for rendering
     let mut canvas = window.into_canvas().build().unwrap();
 
     // Create an event pump for handling events
     let mut event_pump = sdl_context.event_pump().unwrap();
 
     

       // Main emulation loop
    'emulation: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'emulation,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'emulation,
                _ => {}
            }
        }

        // Emulate one instruction (replace with your emulator logic)
        cpu.step();

        // Update your display (e.g., render graphics)
        render_graphics(&mut canvas);

       // Introduce a small delay (optional)
       std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
   }



    // - initialise emulator App
    // - initialise boot buffer and game buffer
    // - initialise CPU: boot/game buffer
    // - initialise Window: dimentions, options, title, etc.
    // - run emulation cycle
}

fn run()
{}



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