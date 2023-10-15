
use flags_register::FlagsRegister;

use super::flags_register;


// 8-bit registers
pub struct Registers
{
    pub a: u8,
    pub b: u8,
    pub  c: u8,
    pub  d: u8,
    pub e: u8,
    pub f: FlagsRegister,
    pub h: u8,
    pub  l: u8,
}

impl Registers 
{
    pub fn new() -> Registers
    {
        Registers
        {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister::new(),
            h: 0,
            l: 0,
            
        }
    }

    // Read/Write functions for virtual 16-bits registers
    // BC:
    pub  fn get_bc(&self) -> u16
    {
        (self.b as u16) << 8 | (self.c as u16)
    }

    pub fn set_bc(&mut self, value: u16)
    {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    // AF:
    pub fn get_af(&self) -> u16
    {
        
        (self.a as u16) << 8 | self.f.as_u16()
    }

    pub fn set_af(&mut self, value: u16)
    {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = FlagsRegister::from((value & 0xFF) as u8);
    }

    // DE:
    pub fn get_de(&self) -> u16
    {
        (self.d as u16) << 8 | (self.e as u16)
    }

    pub  fn set_de(&mut self, value: u16)
    {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    // HL:
    pub fn get_hl(&self) -> u16
    {
        (self.h as u16) << 8 | (self.l as u16)
    }

    pub  fn set_hl(&mut self, value: u16)
    {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}