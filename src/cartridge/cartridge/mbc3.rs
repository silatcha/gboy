use crate::{cartridge::cartridge::ram_banks, device::device::Device};

enum Mode {
    Ram,
    Rtc,
}

/// MBC3 controller.
pub struct Mbc3 {
    rom: Box<[u8]>,
    ram: Vec<[u8; 0x2000]>,
    rtc: [u8; 5],
    rtc_select: usize,
    rom_bank: usize,
    ram_bank: usize,
    ram_timer_enabled: bool,
    mode: Mode,
}

impl Mbc3 {
    pub fn new(rom: Box<[u8]>) -> Self {
        let ram_banks = ram_banks(rom[0x149]);
        Self { rom,
               ram: vec![[0; 0x2000]; ram_banks],
               rtc: [0; 5],
               rtc_select: 0,
               rom_bank: 0,
               ram_bank: 0,
               ram_timer_enabled: false,
               mode: Mode::Ram }
    }

    fn rom_addr(&self, addr: usize) -> usize {
        0x4000 * self.rom_bank.max(1) + addr - 0x4000
    }
}

impl Device for Mbc3 {
    fn read(&self, addr: u16) -> u8 {
        match addr as usize {
            addr @ 0x0000..=0x3fff => self.rom[addr],
            addr @ 0x4000..=0x7fff => {
                let addr = self.rom_addr(addr);
                self.rom.get(addr).copied().unwrap_or(0)
            }
            addr @ 0xa000..=0xbfff => {
                if self.ram_timer_enabled {
                    match self.mode {
                        Mode::Ram => self.ram
                                         .get(self.ram_bank)
                                         .map(|bank| bank[addr - 0xa000])
                                         .unwrap_or(0),
                        Mode::Rtc => self.rtc[self.rtc_select],
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
            0x0000..=0x1fff => self.ram_timer_enabled = data & 0xf == 0xa,
            0x2000..=0x3fff => self.rom_bank = data as usize,
            0x4000..=0x5fff => match data {
                0x00..=0x03 => {
                    self.mode = Mode::Ram;
                    self.ram_bank = data as usize
                }
                0x08..=0x0c => {
                    self.mode = Mode::Rtc;
                    self.rtc_select = data as usize - 0x08
                }
                _ => panic!(),
            },

            0x6000..=0x7fff => {}
            addr @ 0xa000..=0xbfff => {
                if self.ram_timer_enabled {
                    match self.mode {
                        Mode::Ram => {
                            if let Some(bank) = self.ram.get_mut(self.ram_bank) {
                                bank[addr - 0xa000] = data
                            }
                        }
                        Mode::Rtc => self.rtc[self.rtc_select] = data,
                    }
                }
            }
            _ => panic!(),
        }
    }
}
