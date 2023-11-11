use crate::{apu::samples::SamplesMutex
, device::device::Device, Audio};

use std::{
    marker::PhantomData,
    sync::{Arc, Mutex, MutexGuard},
};


//pub mod clock;

pub struct ApuInner<D: Audio> {
    _phantom: PhantomData<D>,

    sample: u64,

    pub(crate) ch0: Option<f64>,
    pub(crate) ch1: Option<f64>,
    pub(crate) ch2: Option<f64>,
    pub(crate) ch3: Option<f64>,

    // Sound Channel 1 - Tone & Sweep
    nr10: u8,
    nr11: u8,
    nr12: u8,
    nr13: u8,
    nr14: u8,
    // Sound Channel 2 - Tone
    nr21: u8,
    nr22: u8,
    nr23: u8,
    nr24: u8,
    // Sound Channel 3 - Wave Output
    nr30: u8,
    nr31: u8,
    nr32: u8,
    nr33: u8,
    nr34: u8,
    wave_ram: [u8; 0x10],
    // Sound Channel 4 - Noise
    nr41: u8,
    nr42: u8,
    nr43: u8,
    nr44: u8,

    // Sound Control Registers
    nr50: u8,
   
    pub nr51: u8,

    nr52: u8,
}


impl<D: Audio> ApuInner<D> {
    pub fn step(&mut self, cycles: u64) {}

  
    fn power_off(&mut self) {
        self.nr10 = 0;
        self.nr11 = 0;
        self.nr12 = 0;
        self.nr13 = 0;
        self.nr14 = 0;

        self.nr21 = 0;
        self.nr22 = 0;
        self.nr23 = 0;
        self.nr24 = 0;

        self.nr30 = 0;
        self.nr31 = 0;
        self.nr32 = 0;
        self.nr33 = 0;
        self.nr34 = 0;

        self.nr41 = 0;
        self.nr42 = 0;
        self.nr43 = 0;
        self.nr44 = 0;

        self.nr50 = 0;
        self.nr51 = 0;
        self.nr52 &= 0x80;
    }
}

pub struct Apu<D: Audio> {
   pub inner: Arc<Mutex<ApuInner<D>>>,
   pub apuinner: ApuInner<D>,
}

impl<D: Audio> Default for Apu<D> {
  
    fn default() -> Self {
        let inner = ApuInner { _phantom: PhantomData,
                               sample: 0,

                               ch0: None,
                               ch1: None,
                               ch2: None,
                               ch3: None,

                               nr10: 0,
                               nr11: 0,
                               nr12: 0,
                               nr13: 0,
                               nr14: 0,

                               nr21: 0,
                               nr22: 0,
                               nr23: 0,
                               nr24: 0,

                               nr30: 0,
                               nr31: 0,
                               nr32: 0,
                               nr33: 0,
                               nr34: 0,
                               wave_ram: [0; 0x10],

                               nr41: 0,
                               nr42: 0,
                               nr43: 0,
                               nr44: 0,

                               nr50: 0,
                               nr51: 0,
                               nr52: 0 };

                              let inn=inner.clone();
                              
        Self { inner: Arc::new(Mutex::new(inner)),
            apuinner: inn }
    }
}

impl<D: Audio> Apu<D> {
    /// Return audio samples iterator.
    pub fn samples(&self) -> SamplesMutex<D> {
        SamplesMutex::new(&self.inner)
    }

pub fn getinner(&self)->ApuInner<D>{
    self.apuinner.clone()
}
    pub fn lock(&self) -> MutexGuard<ApuInner<D>> {
     
        match self.inner.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }
}



impl<D: Audio> Clone for ApuInner<D> {
    fn clone(&self) -> Self {
        ApuInner {
            _phantom: PhantomData,
            sample: self.sample,
            ch0: self.ch0.clone(),
            ch1: self.ch1.clone(),
            ch2: self.ch2.clone(),
            ch3: self.ch3.clone(),
            nr10: self.nr10,
            nr11: self.nr11,
            nr12: self.nr12,
            nr13: self.nr13,
            nr14: self.nr14,
            nr21: self.nr21,
            nr22: self.nr22,
            nr23: self.nr23,
            nr24: self.nr24,
            nr30: self.nr30,
            nr31: self.nr31,
            nr32: self.nr32,
            nr33: self.nr33,
            nr34: self.nr34,
            wave_ram: self.wave_ram.clone(),
            nr41: self.nr41,
            nr42: self.nr42,
            nr43: self.nr43,
            nr44: self.nr44,
            nr50: self.nr50,
            nr51: self.nr51,
            nr52: self.nr52,
        }
    }
}




impl<D: Audio> Device for Apu<D> {
    fn read(&self, addr: u16) -> u8 {
        let apu = match self.inner.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        match addr {
            0xff10 => apu.nr10,
            0xff11 => apu.nr11,
            0xff12 => apu.nr12,
            0xff13 => apu.nr13,
            0xff14 => apu.nr14,

            0xff16 => apu.nr21,
            0xff17 => apu.nr22,
            0xff18 => apu.nr23,
            0xff19 => apu.nr24,

            0xff1a => apu.nr30,
            0xff1b => apu.nr31,
            0xff1c => apu.nr32,
            0xff1d => apu.nr33,
            0xff1e => apu.nr34,
            0xff30..=0xff3f => apu.wave_ram[addr as usize - 0xff30],

            0xff20 => apu.nr41,
            0xff21 => apu.nr42,
            0xff22 => apu.nr43,
            0xff23 => apu.nr44,

            0xff24 => apu.nr50,
            0xff25 => apu.nr51,

            // TODO
            0xff26 => apu.nr52 & 0x80,
            0xff27..=0xff2f => panic!(), // unused
            _ => panic!(),
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        let mut apu = match self.inner.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        if apu.nr52 & 0x80 != 0 {
            match addr {
                // Channel 1 sweep
                0xff10 => apu.nr10 = data,
                0xff11 => apu.nr11 = data,
                0xff12 => apu.nr12 = data,
                0xff13 => apu.nr13 = data,
                0xff14 => {
                    apu.nr14 = data & 0xc7;

                    if apu.nr14 & 0x80 != 0 {}
                }

                // Channel 2 - Tone
                0xff16 => apu.nr21 = data,
                0xff17 => apu.nr22 = data,
                0xff18 => apu.nr23 = data,
                0xff19 => {
                    apu.nr24 = data & 0xc7;

                    if apu.nr24 & 0x80 != 0 {}
                }

                // Channel 3 - Wave RAM
                0xff1a => apu.nr30 = data,
                0xff1b => apu.nr31 = data,
                0xff1c => apu.nr32 = data,
                0xff1d => apu.nr33 = data,
                0xff1e => {
                    apu.nr34 = data;

                    if apu.nr34 & 0x80 != 0 {}
                }
                0xff30..=0xff3f => { /* Handled below */ }

                // Channel 4 - Noise
                0xff20 => apu.nr41 = data,
                0xff21 => apu.nr42 = data,
                0xff22 => apu.nr43 = data,
                0xff23 => {
                    apu.nr44 = data;

                    if apu.nr44 & 0x80 != 0 {}
                }

                0xff24 => apu.nr50 = data,
                0xff25 => apu.nr51 = data,

                0xff26 => { /* Handled below */ }
                0xff27..=0xff2f => { /* Unused */ }
                _ => panic!(),
            }
        }

        // Wave RAM writes are unaffected by power status
        if let 0xff30..=0xff3f = addr {
            // let f = data >> 4;
            // let s = data & 0xf;
            // for _ in 0..f {
            //     print!("-");
            // }
            // println!("*");
            // for _ in 0..s {
            //     print!("-");
            // }
            // println!("*");
            apu.wave_ram[addr as usize - 0xff30] = data;
        }
        if addr == 0xff3f {
            // println!("===");
        }

        // Enable / Disable sound entirely
        if addr == 0xff26 {
            apu.nr52 &= 0x7f;
            apu.nr52 |= data & 0x80;

            if apu.nr52 & 0x80 == 0 {
                apu.power_off();
            }
        }
    }
}
