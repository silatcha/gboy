
use crate::device::device::Device;

mod mbc1;
mod mbc3;
mod mbc5;
mod rom;

pub use mbc1::Mbc1;
pub use mbc3::Mbc3;
pub use mbc5::Mbc5;
pub use rom::Rom;


pub trait Cartridge: Device {}

impl Cartridge for () {}
impl Cartridge for Rom {}
impl Cartridge for Mbc1 {}
impl Cartridge for Mbc3 {}
impl Cartridge for Mbc5 {}
impl Cartridge for Box<dyn Cartridge> {}

impl Device for Box<dyn Cartridge> {
    fn read(&self, addr: u16) -> u8 {
        self.as_ref().read(addr)
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.as_mut().write(addr, data)
    }
}



fn ram_banks(banks: u8) -> usize {
    match banks {
        0x00 => 0,
        0x01 | 0x02 => 1,
        0x03 => 4,
        0x04 => 16,
        _ => panic!(),
    }
}
