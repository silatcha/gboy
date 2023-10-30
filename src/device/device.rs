pub trait Device {
    /// Read a single byte from memory.
    fn read(&self, addr: u16) -> u8;

    /// Write a single byte to memory.
    fn write(&mut self, addr: u16, data: u8);

    /// Read a signed byte from memory.
    fn read_signed(&self, addr: u16) -> i8 {
        unsafe { std::mem::transmute(self.read(addr)) }
    }

    /// Read a 2 byte word to memory.
    fn read_word(&self, addr: u16) -> u16 {
        let lo = self.read(addr);
        let hi = self.read(addr + 1);
        (u16::from(hi) << 8) | u16::from(lo)
    }

    /// Write a 2 byte word to memory.
    fn write_word(&mut self, addr: u16, data: u16) {
        let lo = data & 0xff;
        let hi = data >> 8;
        self.write(addr, lo as u8);
        self.write(addr + 1, hi as u8);
    }
}

// Base implementation to emulate the lack of a device.
// For example, () represents the lack of a connected cartridge.
impl Device for () {
    fn read(&self, _: u16) -> u8 {
        0xff
    }

    fn write(&mut self, _: u16, _: u8) {}
}

#[cfg(test)]
mod tests {
    use crate::device::device::Device;

    #[test]
    fn words() {
        impl Device for [u8; 4] {
            fn read(&self, addr: u16) -> u8 {
                self[addr as usize]
            }

            fn write(&mut self, addr: u16, data: u8) {
                self[addr as usize] = data;
            }
        }

        let mut dev = [0u8; 4];

        dev.write_word(0, 0x1234);
        dev.write_word(2, 0xabcd);
        assert_eq!(0x1234, dev.read_word(0));
        assert_eq!(0xabcd, dev.read_word(2));
    }
}
