
use emulator::{
    apu::device::{Audio, Stereo44100},
    cartridge,
    apu::samples::SamplesMutex,
    cartridge::cartridge::{Cartridge, Mbc1, Mbc3, Mbc5},
    joypad::joypad::{Btn, Dir, Joypad, Key},
    ppu::ppu::{palette::*, Video},
    Builder, GameBoy,
    sdlvideo::SdlVideo,
    callback,
    callback::Callback
};
use sdl2::{
    event::{Event, WindowEvent, EventWatchCallback},
    keyboard::Scancode,
    EventPump,
};
use std::{
    thread,
    time::{Duration, Instant},
    sync::{Arc, Mutex, MutexGuard},
};
use std::marker::PhantomData;

const SCALE: u32 = 4;

static ROM: &[u8] = include_bytes!("../data/Super_Mario_Land.gb");

fn main() {
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
    
    let audio_subsystem = sdl.audio().unwrap();
    
    let stereo_config: Stereo44100<f32> = Stereo44100(PhantomData);
    let mut emulator = Builder::default().video(SdlVideo::new(canvas))
                                         .cartridge(cartridge::from_bytes(ROM).unwrap())
                                         .gb_mode()
                                         .build();
                                        
     
    //let audio = <dyn Audio>::Sample;
    let samples_mutex = SamplesMutex::new(&Arc::new(Mutex::new(emulator.mmu_mut().apu().apuinner)));                                      
    let audio_device =callback::create_device(&audio_subsystem, samples_mutex);
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
            Event::KeyDown { scancode: Some(Scancode::S),
                             .. } => unimplemented!("screenshot"),
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
        Scancode::Z => Some(Key::Btn(Btn::A)),
        Scancode::X => Some(Key::Btn(Btn::B)),
        Scancode::RShift => Some(Key::Btn(Btn::Select)),
        Scancode::Return => Some(Key::Btn(Btn::Start)),
        Scancode::Left => Some(Key::Dir(Dir::Left)),
        Scancode::Right => Some(Key::Dir(Dir::Right)),
        Scancode::Up => Some(Key::Dir(Dir::Up)),
        Scancode::Down => Some(Key::Dir(Dir::Down)),
        _ => None,
    }
}