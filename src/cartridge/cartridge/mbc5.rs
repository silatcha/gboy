use crate::{cartridge::cartridge::ram_banks, device::device::Device};

/// MBC5 controller.
pub struct Mbc5 {
    rom: Box<[u8]>,
    ram: Vec<[u8; 0x2000]>,
    rom_bank: usize,
    ram_bank: usize,
    ram_enabled: bool,
}

impl Mbc5 {
    pub fn new(rom: Box<[u8]>) -> Self {
        let ram_banks = ram_banks(rom[0x149]);
        Self { rom,
               ram: vec![[0; 0x2000]; ram_banks],
               rom_bank: 0,
               ram_bank: 0,
               ram_enabled: true }
    }

    fn rom_addr(&self, addr: usize) -> usize {
        0x4000 * self.rom_bank + addr - 0x4000
    }
}

impl Device for Mbc5 {
    fn read(&self, addr: u16) -> u8 {
        match addr as usize {
            addr @ 0x0000..=0x3fff => self.rom[addr],
            addr @ 0x4000..=0x7fff => {
                let addr = self.rom_addr(addr);
                self.rom.get(addr).copied().unwrap_or(0)
            }
            addr @ 0xa000..=0xbfff => {
                if self.ram_enabled {
                    if let Some(bank) = self.ram.get(self.ram_bank) {
                        bank[addr - 0xa000]
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
            _ => panic!(),
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr as usize {

            0x0000..=0x1fff => self.ram_enabled = data & 0xf == 0xa,

            0x2000..=0x2fff => {
                self.rom_bank &= !0xff;
                self.rom_bank |= data as usize;
            }
            0x3000..=0x3fff => {
                self.rom_bank &= 0xff;
                self.rom_bank |= (data as usize & 0x1) << 8;
            }

            0x4000..=0x5fff => self.ram_bank = (data & 0xf) as usize,
            0x6000..=0x7fff => { /* read-only */ }
            addr @ 0xa000..=0xbfff => {
                if self.ram_enabled {
                    if let Some(bank) = self.ram.get_mut(self.ram_bank) {
                        bank[addr - 0xa000] = data;
                    }
                }
            }
            _ => panic!("{:x}", addr),
        }
    }
}
