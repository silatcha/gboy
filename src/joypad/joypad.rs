use crate::{device::device::Device, interrupt::interrupt::Flag};

const BTN_ROW_FLAG: u8 = 0x10;
const DIR_ROW_FLAG: u8 = 0x20;

/// Buttons.
#[derive(Debug, Clone, Copy)]
pub enum Key {
    Btn(Btn),
    Dir(Dir),
}

/// Action buttons.
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Btn {
    Start = 0x8,
    Select = 0x4,
    A = 0x2,
    B = 0x1,
}

/// Directional buttons.
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Dir {
    Down = 0x8,
    Up = 0x4,
    Left = 0x2,
    Right = 0x1,
}

/// Joypad emulation.
pub struct Joypad {
    int: Option<Flag>,
    joyp: u8,
    btn: u8,
    dir: u8,
}

impl Default for Joypad {
    fn default() -> Self {
        Self { int: None,
               joyp: 0x00,
               btn: 0xff,
               dir: 0xff }
    }
}

impl Joypad {
    /// Register new keypad input.
    pub fn press(&mut self, key: Key) {
        let (btn, dir) = match key {
            Key::Btn(btn) => (self.btn & !(btn as u8), self.dir),
            Key::Dir(dir) => (self.btn, self.dir & !(dir as u8)),
        };


        if self.btn != btn || self.dir != dir {
            self.int = Some(Flag::Joypad);
        }

        self.btn = btn;
        self.dir = dir;
    }


    pub fn release(&mut self, key: Key) {
        match key {
            Key::Btn(btn) => self.btn |= btn as u8,
            Key::Dir(dir) => self.dir |= dir as u8,
        }
    }

    pub(crate) fn take_int(&mut self) -> Option<Flag> {
        self.int.take()
    }
}

impl Device for Joypad {
    fn read(&self, addr: u16) -> u8 {
        assert_eq!(0xff00, addr);
        match self.joyp & 0x30 {
            BTN_ROW_FLAG => BTN_ROW_FLAG | (self.btn & 0xf),
            DIR_ROW_FLAG => DIR_ROW_FLAG | (self.dir & 0xf),
            0x30 => 0x3f,
            0x0 => 0xf,
            _ => unreachable!(),
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        assert_eq!(0xff00, addr);
        self.joyp = data;
    }
}

