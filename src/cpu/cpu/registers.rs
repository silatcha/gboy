#[repr(u8)]
pub enum Flag {
    Z = 0x80,
    N = 0x40,
    H = 0x20,
    C = 0x10,
}

#[derive(Debug)]
#[rustfmt::skip]
pub struct Registers {
    pub a: u8, pub f: u8,
    pub b: u8, pub c: u8,
    pub d: u8, pub e: u8,
    pub h: u8, pub l: u8,
    /// Program counter
    pub pc: u16,
    /// Stack pointer
    pub sp: u16,
}

impl Default for Registers {
    #[rustfmt::skip]
    fn default() -> Self {
        Self {
            a: 0, f: 0,
            b: 0, c: 0,
            d: 0, e: 0,
            h: 0, l: 0,
            pc: 0,
            sp: 0,
        }
    }
}

impl Registers {
    pub fn af(&self) -> u16 {
        u16::from(self.a) << 8 | u16::from(self.f)
    }

    pub fn bc(&self) -> u16 {
        u16::from(self.b) << 8 | u16::from(self.c)
    }

    pub fn de(&self) -> u16 {
        u16::from(self.d) << 8 | u16::from(self.e)
    }

    pub fn hl(&self) -> u16 {
        u16::from(self.h) << 8 | u16::from(self.l)
    }

    pub fn set_af(&mut self, af: u16) {
        self.a = (af >> 8) as u8;
        // low nibble in F register should always be 0
        self.f = (af & 0xf0) as u8;
    }

    pub fn set_bc(&mut self, bc: u16) {
        self.b = (bc >> 8) as u8;
        self.c = (bc & 0xff) as u8;
    }

    pub fn set_de(&mut self, de: u16) {
        self.d = (de >> 8) as u8;
        self.e = (de & 0xff) as u8;
    }

    pub fn set_hl(&mut self, hl: u16) {
        self.h = (hl >> 8) as u8;
        self.l = (hl & 0xff) as u8;
    }

    pub fn is_flag(&self, flag: Flag) -> bool {
        self.f & (flag as u8) != 0
    }

    pub fn set_flag(&mut self, flag: Flag, b: bool) {
        if b {
            self.f |= flag as u8;
        } else {
            self.f &= !(flag as u8);
        }
    }
}

