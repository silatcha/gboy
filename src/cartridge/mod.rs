use self::cartridge::{Cartridge, Rom, Mbc1, Mbc3, Mbc5};

pub mod cartridge;

pub fn from_bytes(bytes: &[u8]) -> Result<Box<dyn Cartridge>, ()> {
    let bytes = bytes.to_vec().into_boxed_slice();
    match *bytes.get(0x147).ok_or(())? {
        0x00 => Ok(Box::new(Rom::new(bytes))),
        0x01..=0x03 => Ok(Box::new(Mbc1::new(bytes))),
        0x0f..=0x13 => Ok(Box::new(Mbc3::new(bytes))),
        0x19..=0x1e => Ok(Box::new(Mbc5::new(bytes))),
        _ => Err(()),
    }
}