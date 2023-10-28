pub mod memory_bus;
use crate::cpu::mmu::memory_bus::test::gpu;

pub trait Display {
    /// Clear the display
    fn clear(&mut self);
    /// Paint pixel at (x, y) using `color`. (0, 0) is top left.
    fn set_pixel(&mut self, x: u32, y: u32, color: gpu::Colors);
    /// Current frame is done and can be displayed.
    fn flip(&mut self);
}
pub enum Event {
    /// No event
    None,
    /// Shutdown the emulator
    PowerOff,
}



/// Description of a button's state
#[derive(Debug,Clone,Copy)]
pub enum ButtonState {
    /// Key is pushed down
    Down,
    /// Key is up
    Up,
}

impl ButtonState {
    pub fn is_down(self) -> bool {
        match self {
            ButtonState::Down => true,
            _                 => false,
        }
    }
}

/// State of all the GB buttons
#[derive(Debug,Clone,Copy)]
pub struct Buttons {
    pub up:        ButtonState,
    pub down:      ButtonState,
    pub left:      ButtonState,
    pub right:     ButtonState,
    pub a:         ButtonState,
    pub b:         ButtonState,
    pub start:     ButtonState,
    pub select:    ButtonState,
    /// State of the interrupt that occurs at the moment a button is
    /// pressed
    pub interrupt: bool,
}

impl Buttons {
    pub fn new(default_state: ButtonState) -> Buttons {
        Buttons {
            a:         default_state,
            b:         default_state,
            start:     default_state,
            select:    default_state,
            up:        default_state,
            down:      default_state,
            left:      default_state,
            right:     default_state,
            interrupt: false,
        }
    }
}
