use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::render::RendererContext;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::Sdl;
use std::cell::Cell;
use sdl2;
use sdl2::GameControllerSubsystem;
// Re-export the public interface defined in sub-modules

pub use super::controller::Controller;



use crate::cpu::mmu::Event;
use crate::cpu::mmu::Buttons;
use crate::cpu::mmu;

use super::gpu::Colors;

pub struct Display {
    canvas : Canvas<Window>,
    upscale:  u8,
    sdl2: Sdl,
    controller: Controller,
  
}

impl Display {
    pub fn new(upscale: u8) -> Display {
        let up = 1 << (upscale as usize);
        let xres = 160 * up;
        let yres = 144 * up;
       
        let g:Sdl=None;
        let mut game= sdl2::init();

        if let Ok(game)= game {
            let game_controller_subsystem=game.game_controller();
           let g=game;

        let available = game_controller_subsystem
    .num_joysticks()
    .map_err(|e| format!("can't enumerate joysticks: {}", e));
println!("{:?} joysticks available", available);

let mut controller = (0..available)
.find_map(|id| {
    if !game_controller_subsystem.is_game_controller(id) {
        println!("{} is not a game controller", id);
        return None;
    }

    println!("Attempting to open controller {}", id);

    match game_controller_subsystem.open(id) {
        Ok(c) => {
            // We managed to find and open a game controller,
            // exit the loop
            println!("Success: opened \"{}\"", c.name());
            Some(c)
        }
        Err(e) => {
            println!("failed: {:?}", e);
            None
        }
    }
})
.expect("Couldn't open any controller");
println!("Controller mapping: {}", controller.mapping());


        }else {
            println!("Failed to initialize SDL2");
        }


       


        
     

        
      
        let video_subsystem = game.video().unwrap();
        let windows = video_subsystem
        .window("gameboy Emulator", xres, yres)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
        



   

        Display { 
           canvas: windows.into_canvas() .present_vsync() //< this means the screen cannot
           // render faster than your display rate (usually 60Hz or 144Hz)
           .build().unwrap(),
            upscale: upscale,
            
            sdl2: game,
            controller: Controller::new(),
        
    }
    }
    
/*
    pub fn new_display(&self, upscale: u8) -> Display {
        self.(upscale)
    }*/

    pub fn buttons(&self) -> &Cell<Buttons> {
        self.controller.buttons()
    }

    pub fn update_buttons(&self) -> Event {
        self.controller.update(&self.sdl2)
    }



  pub  fn clear(&mut self) {
        //let mut drawer = self.canvas.drawer();

        let _ = self.canvas.set_draw_color(Color::RGB(0xff, 0x00, 0x00));
        let _ = self.canvas.clear();
    }

 pub   fn set_pixel(&mut self, x: u32, y: u32, color: Colors) {
        let color = match color {
            Colors::Black     => Color::RGB(0x00, 0x00, 0x00),
            Colors::DarkGrey  => Color::RGB(0x55, 0x55, 0x55),
            Colors::LightGrey => Color::RGB(0xab, 0xab, 0xab),
            Colors::White     => Color::RGB(0xff, 0xff, 0xff),
        };

       

         self.canvas.set_draw_color(color);

        if self.upscale == 0 {
            let _ = self.canvas.draw_point(Point::new(x as i32, y as i32));
        } else {
            let up = 1 << (self.upscale as usize);

            // Translate coordinates
            let x = x as i32 * up;
            let y = y as i32 * up;

            let _ = self.canvas.fill_rect(Rect::new(x, y, up as u32, up as u32));
        }
    }


    

  pub  fn flip(&mut self) {
        self.canvas.present();
        self.clear();
    }


}

impl mmu::Display for Display {
    fn clear(&mut self) {
        //let mut drawer = self.canvas.drawer();

        let _ = self.canvas.set_draw_color(Color::RGB(0xff, 0x00, 0x00));
        let _ = self.canvas.clear();
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: Colors) {
        let color = match color {
            Colors::Black     => Color::RGB(0x00, 0x00, 0x00),
            Colors::DarkGrey  => Color::RGB(0x55, 0x55, 0x55),
            Colors::LightGrey => Color::RGB(0xab, 0xab, 0xab),
            Colors::White     => Color::RGB(0xff, 0xff, 0xff),
        };

       

         self.canvas.set_draw_color(color);

        if self.upscale == 0 {
            let _ = self.canvas.draw_point(Point::new(x as i32, y as i32));
        } else {
            let up = 1 << (self.upscale as usize);

            // Translate coordinates
            let x = x as i32 * up;
            let y = y as i32 * up;

            let _ = self.canvas.fill_rect(Rect::new(x, y, up as u32, up as u32));
        }
    }


    

    fn flip(&mut self) {
        self.canvas.present();
        self.clear();
    }


    
}


