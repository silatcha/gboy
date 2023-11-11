/// To generate ticks at specific given rate, given a base clock rate.
pub struct Clock {
    base: u64,
    freq: u64,
    tick: u64,
}

impl Clock {
    pub fn new(base: u64, freq: u64) -> Self {
        assert!(freq <= base);
        Self { base,
               freq,
               tick: 0 }
    }


    pub fn step(&mut self, cycles: u64) -> u64 {
        let cycles_tick = self.base / self.freq;
        self.tick += cycles;
        let clocks = self.tick / cycles_tick;
        self.tick %= cycles_tick;
        clocks
    }
}

