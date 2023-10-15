
use flags_register::FlagsRegister;

use super::flags_register;


// 8-bit registers
pub struct Registers
{
    pub A: u8,
    pub B: u8,
    pub  C: u8,
    pub  D: u8,
    pub E: u8,
    pub F: FlagsRegister,
    pub H: u8,
    pub  L: u8,
}

impl Registers 
{
    pub fn new() -> Registers
    {
        Registers
        {
            A: 0,
            B: 0,
            C: 0,
            D: 0,
            E: 0,
            F: FlagsRegister::new(),
            H: 0,
            L: 0,
            
        }
    }

    // Read/Write functions for virtual 16-bits registers
    // BC:
    pub  fn get_BC(&self) -> u16
    {
        (self.B as u16) << 8 | (self.C as u16)
    }

    pub fn set_BC(&mut self, value: u16)
    {
        self.B = ((value & 0xFF00) >> 8) as u8;
        self.C = (value & 0xFF) as u8;
    }

    // AF:
    pub fn get_AF(&self) -> u16
    {
        
        (self.A as u16) << 8 | self.F.as_u16()
    }

    pub fn set_AF(&mut self, value: u16)
    {
        self.A = ((value & 0xFF00) >> 8) as u8;
        self.F = FlagsRegister::from((value & 0xFF) as u8);
    }

    // DE:
    pub fn get_DE(&self) -> u16
    {
        (self.D as u16) << 8 | (self.E as u16)
    }

    pub  fn set_DE(&mut self, value: u16)
    {
        self.D = ((value & 0xFF00) >> 8) as u8;
        self.E = (value & 0xFF) as u8;
    }

    // HL:
    pub fn get_HL(&self) -> u16
    {
        (self.H as u16) << 8 | (self.L as u16)
    }

    pub  fn set_HL(&mut self, value: u16)
    {
        self.H = ((value & 0xFF00) >> 8) as u8;
        self.L = (value & 0xFF) as u8;
    }
}