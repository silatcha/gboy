use crate::{cartridge::cartridge::ram_banks, device::device::Device};

enum Mode {
    Rom,
    Ram,
}

/// MBC1 controller.
#[rustfmt::skip]
pub struct Mbc1 {
    rom: Box<[u8]>,
    ram: Vec<[u8; 0x2000]>,
    rom_bank: usize,
    ram_bank: usize,
    ram_enable: bool,
    mode: Mode,
}

impl Mbc1 {
    pub fn new(rom: Box<[u8]>) -> Self {
        let ram_banks = ram_banks(rom[0x149]);
        Self { rom,
               ram: vec![[0; 0x2000]; ram_banks],
               rom_bank: 0,
               ram_bank: 0,
               ram_enable: false,
               mode: Mode::Rom }
    }

    fn rom_addr(&self, addr: usize) -> usize {
        0x4000 * self.rom_bank.max(1) + addr - 0x4000
    }
}

impl Device for Mbc1 {
    fn read(&self, addr: u16) -> u8 {
        match addr as usize {
            addr @ 0x0000..=0x3fff => self.rom.get(addr).copied().unwrap_or(0xff),
            addr @ 0x4000..=0x7fff => {
                let addr = self.rom_addr(addr);
                self.rom.get(addr).copied().unwrap_or(0)
            }
            addr @ 0xa000..=0xbfff => {
                if self.ram_enable {
                    self.ram
                        .get(self.ram_bank)
                        .map(|bank| bank[addr - 0xa000])
                        .unwrap_or(0)
                } else {
                    0
                }
            }
            _ => panic!(),
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr as usize {
            
            0x0000..=0x1fff => self.ram_enable = data & 0xf == 0xa,
            
            0x2000..=0x3fff => {
                self.rom_bank &= 0x60;
                self.rom_bank |= data as usize & 0x1f;
            }
            
            0x4000..=0x5fff => match self.mode {
                Mode::Rom => {
                    self.rom_bank &= 0x1f;
                    self.rom_bank |= (data as usize & 0x3) << 5;
                }
                Mode::Ram => self.ram_bank = data as usize & 0x3,
            },
            0x6000..=0x7fff => {
                self.mode = match data {
                    0x00 => Mode::Rom,
                    0x01 => Mode::Ram,
                    _ => panic!(),
                }
            }
            addr @ 0xa000..=0xbfff => {
                if let Some(bank) = self.ram.get_mut(self.ram_bank) {
                    bank[addr as usize - 0xa000] = data
                }
            }
            _ => panic!(),
        }
    }
}
