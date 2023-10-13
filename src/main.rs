

extern crate sdl2;
use std::path::Path;

                    

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
    let rompath = Path::new(&argv[1]);
    //let mut b=Option<Vec<u8>>::new();
   // let mut rom = Vec::new();

    
    //let cpu=CPU::new(rom, &rompath);

   
    // - initialise emulator App
    // - initialise boot buffer and game buffer
    // - initialise CPU: boot/game buffer
    // - initialise Window: dimentions, options, title, etc.
    // - run emulation cycle
}

fn run()
{}