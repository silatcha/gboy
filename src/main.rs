
use emulator::{
    apu::device::Audio,
    cartridge,
    cartridge::cartridge::Cartridge,
    joypad::joypad::{Btn, Dir, Key},
    ppu::ppu::{palette::*, Video},
    Builder, GameBoy,
    sdlvideo::SdlVideo,
};
use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Scancode,
    EventPump,
};
use std::{
    thread,
    io,
    time::{Duration, Instant},
};

use std::env;


const SCALE: u32 = 4;


fn main() {


    let mut roms: &[u8] = include_bytes!("../data/Aladdin.gb");

    
              loop {
                println!("Veuillez selectionner le jeu ( écrivez un numero ) :");
                println!("\n
                          1) aladdin 
                          2) Choplifter
                          3) Fun_Pak
                          4) Super Mario");
        
                let mut numero_jeu = String::new();
        
                io::stdin()
                    .read_line(&mut numero_jeu)
                    .expect("Échec de la lecture de l'entrée utilisateur");
        
                let numero_jeu: u32 = match numero_jeu.trim().parse() {
                    Ok(nombre) => nombre,
                    Err(_) => continue,
                };
        
        
                if numero_jeu == 1 {
                    roms = include_bytes!("../data/Aladdin.gb");
                    break;
                }else if  numero_jeu == 2 {
                    roms = include_bytes!("../data/Choplifter.gb");
                    break;
                }else if  numero_jeu == 3 {
                    roms = include_bytes!("../data/Fun_Pak.gb");
                    break;
                } else if  numero_jeu == 4 {
                    roms = include_bytes!("../data/Super_Mario_Land.gb");
                    break;
                }
            }
   

   

    env::set_var("RUST_BACKTRACE", "1");
    let sdl = sdl2::init().unwrap();
    let canvas = sdl.video()
                    .unwrap()
                    .window("DMG", 160 * SCALE, 144 * SCALE)
                    .position_centered()
                    .build()
                    .expect("Error creating SDL window")
                    .into_canvas()
                    .build()
                    .expect("Error creating SDL canvas");
    
   // let audio_subsystem = sdl.audio().unwrap();
    
    //let stereo_config: Stereo44100<f32> = Stereo44100(PhantomData);
    let mut emulator = Builder::default().video(SdlVideo::new(canvas))
                                         .cartridge(cartridge::from_bytes(roms).unwrap())
                                         .gb_mode()
                                         .skip_boot()
                                         .build();
                                        
     
    //let audio = <dyn Audio>::Sample;
   /* let samples_mutex = SamplesMutex::new(&Arc::new(Mutex::new(emulator.mmu_mut().apu().getinner())));                                      
    let audio_device =callback::create_device(&audio_subsystem, samples_mutex).unwrap();
    audio_device.resume();*/
   // std::thread::sleep(std::time::Duration::new(5, 0));
    // set-up custom 4 color palette
    emulator.mmu_mut().ppu_mut().pal_mut().set_color_pal(DMG);

    let mut pump = sdl.event_pump().unwrap();

    let mut carry = Duration::new(0, 0);




    loop {
        let time = Instant::now();

        if handle_input(&mut pump, &mut emulator) {
            break;
        }

        emulator.emulate_frame();
        emulator.mmu_mut()
                .ppu_mut()
                .video_mut()
                .canvas_mut()
                .present();

        let elapsed = time.elapsed() + carry;
        let sleep = Duration::new(0, 1_000_000_000 / 60);
        if elapsed < sleep {
            carry = Duration::new(0, 0);
            thread::sleep(sleep - elapsed);
        } else {
            carry = elapsed - sleep;
        }
    }
}

fn handle_input(pump: &mut EventPump,
                dmg: &mut GameBoy<impl Cartridge, impl Video, impl Audio>)
                -> bool {
    let joypad = dmg.mmu_mut().joypad_mut();
  
    for event in pump.poll_iter() {
        match event {
            Event::Window { win_event: WindowEvent::Close,
                            .. } => return true,
            Event::KeyDown { scancode: Some(Scancode::Escape), .. } => {
                return true
             }
            Event::KeyDown { scancode: Some(s), .. } => {
                if let Some(key) = map_scancode(s) {
                    joypad.press(key)
                }
            }
            Event::KeyUp { scancode: Some(s), .. } => {
                if let Some(key) = map_scancode(s) {
                    joypad.release(key)
                }
            }
            _ => {}
        }
   
    }
    
     
    false
}

fn map_scancode(scancode: Scancode) -> Option<Key> {
    match scancode {
        Scancode::J => Some(Key::Btn(Btn::A)),
        Scancode::K => Some(Key::Btn(Btn::B)),
        Scancode::RShift => Some(Key::Btn(Btn::Select)),
        Scancode::Return => Some(Key::Btn(Btn::Start)),
        Scancode::A => Some(Key::Dir(Dir::Left)),
        Scancode::D => Some(Key::Dir(Dir::Right)),
        Scancode::W => Some(Key::Dir(Dir::Up)),
        Scancode::S => Some(Key::Dir(Dir::Down)),
        _ => None,
    }
}