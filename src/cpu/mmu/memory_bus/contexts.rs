
use std::cell::Cell;

// Re-export the public interface defined in sub-modules
pub use super::display::Display;
pub use super::controller::Controller;
pub use super::audio::Audio;

pub struct Context {
    sdl2: ::sdl2::sdl::Sdl,
    controller: controller::Controller,
}

impl Context {
    pub fn new() -> Context {
        let sdl2 =
            ::sdl2::init().unwrap();

        Context {
            sdl2: sdl2,
            controller: controller::Controller::new(),
        }
    }

    pub fn new_display(&self, upscale: u8) -> display::Display {
        display::Display::new(&self.sdl2, upscale)
    }

    pub fn buttons(&self) -> &Cell<::ui::Buttons> {
        self.controller.buttons()
    }

    pub fn update_buttons(&self) -> ::ui::Event {
        self.controller.update(&self.sdl2)
    }
}