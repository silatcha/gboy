use sdl2::keyboard::Keycode;
pub struct Keypad {
    row0: u8,
    row1: u8,
    data: u8,
    pub interrupt: u8,
}

#[derive(Copy, Clone)]
pub enum KeypadKey {
    Right,
    Left,
    Up,
    Down,
    A,
    B,
    Select,
    Start,
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            row0: 0x0F,
            row1: 0x0F,
            data: 0xFF,
            interrupt: 0,
        }
    }

    pub fn rb(&self) -> u8 {
        self.data
    }

    pub fn wb(&mut self, value: u8) {
        self.data = (self.data & 0xCF) | (value & 0x30);
        self.update();
    }

    fn update(&mut self) {
        let old_values = self.data & 0xF;
        let mut new_values = 0xF;

        if self.data & 0x10 == 0x00 {
            new_values &= self.row0;
        }
        if self.data & 0x20 == 0x00 {
            new_values &= self.row1;
        }

        if old_values == 0xF && new_values != 0xF {
            self.interrupt |= 0x10;
        }

        self.data = (self.data & 0xF0) | new_values;
    }

    pub fn keydown(&mut self, key: Keycode) {
        match key {
            Keycode::D =>      self.row0 &= 1 << 0,
            Keycode::Q =>      self.row0 &= 1 << 1,
            Keycode::Z =>      self.row0 &= 1 << 2,
            Keycode::S =>      self.row0 &= 1 << 3,
            Keycode::A =>      self.row1 &= 1 << 0,
            Keycode::B =>      self.row1 &= 1 << 1,
            Keycode::F =>      self.row1 &= 1 << 2,
            Keycode::Space =>  self.row1 &= 1 << 3,
            _ =>                {}
        }
        self.update();
    }

    pub fn keyup(&mut self, key: Keycode) {
        match key {
            Keycode::D =>  self.row0 |= 1 << 0,
            Keycode::Q =>   self.row0 |= 1 << 1,
            Keycode::Z =>     self.row0 |= 1 << 2,
            Keycode::S =>   self.row0 |= 1 << 3,
            Keycode::A =>      self.row1 |= 1 << 0,
            Keycode::B =>      self.row1 |= 1 << 1,
            Keycode::F => self.row1 |= 1 << 2,
            Keycode::Space =>  self.row1 |= 1 << 3,
            _ =>                {},
        }
        self.update();
    }
}
