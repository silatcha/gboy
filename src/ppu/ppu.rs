use crate::{
    device::device::Device,
    interrupt::interrupt::Flag,
    ppu::ppu::{
        oam::{Entry, Oam},
        palette::Color,
        reg::{
            LcdcStat, StatMode, TileDataAddr, TileMapAddr, STAT_HBLANK_FLAG, STAT_LYC_LY_FLAG,
            STAT_SEARCH_FLAG, STAT_VBLANK_FLAG,
        },
    },
    vram::vram::VRam,
    Mode,
};
use reg::{ColorPal, Line, Pal, Scroll, Window};
use std::mem;

pub mod oam;
pub mod palette;
pub mod reg;

pub const LCD_WIDTH: usize = 160;
pub const LCD_HEIGHT: usize = 144;

pub(crate) const SEARCH: u64 = 80; //  80 dots (19 us)
pub(crate) const PIXELS: u64 = 230; // 168 to 291 cycles (40 to 60 us) depending on sprite count
pub(crate) const HBLANK: u64 = 147; // 85 to 208 dots (20 to 49 us) depending on previous mode 3 duration
pub(crate) const VBLANK: u64 = 4560; // 4560 dots (1087 us, 10 scanlines)

/// Display scanline renderer.
pub trait Video {
    fn draw_video(&mut self, pixels: &[[Color; LCD_WIDTH]; LCD_HEIGHT]);
}

impl Video for () {
    fn draw_video(&mut self, _: &[[Color; LCD_WIDTH]; LCD_HEIGHT]) {}
}

pub struct Ppu<V: Video> {
    video: V,
    mode: Mode,
    // Same as cycles, but documentation often refers to it as "dots" instead of cycles.
    dots: u64,
    buffer: Box<[[Color; LCD_WIDTH]; LCD_HEIGHT]>,
    // Color index within color palette.
    // Used to tell when a sprite is behind a BG tile.
    color_index: Box<[[u8; LCD_WIDTH]; LCD_HEIGHT]>,
    stat_mode: StatMode,
    vram: VRam,
    oam: Oam,
    lcdc_stat: LcdcStat,
    scroll: Scroll,
    line: Line,
    win: Window,
    pal: Pal,
    color_pal: ColorPal,
    vblank_int: Option<Flag>,
    lcdc_int: Option<Flag>,
}

impl<V: Video> Ppu<V> {
    pub(crate) fn new(mode: Mode, output: V) -> Self {
        Self { dots: 0,
               video: output,
               mode,
               buffer: Box::new([[[0xff, 0xff, 0xff]; LCD_WIDTH]; LCD_HEIGHT]),
               color_index: Box::new([[0; LCD_WIDTH]; LCD_HEIGHT]),
               vram: VRam::default(),
               oam: Oam::default(),
               stat_mode: StatMode::HBlank,
               lcdc_stat: LcdcStat::default(),
               scroll: Scroll::default(),
               line: Line::default(),
               win: Window::default(),
               pal: Pal::default(),
               color_pal: ColorPal::default(),
               vblank_int: None,
               lcdc_int: None }
    }

    pub fn lcdc_stat(&self) -> &LcdcStat {
        &self.lcdc_stat
    }

    pub fn win(&self) -> &Window {
        &self.win
    }

    pub fn vram_mut(&mut self) -> &mut VRam {
        &mut self.vram
    }

    /// Return the color palette register
    pub fn color_pal(&self) -> &ColorPal {
        &self.color_pal
    }

    /// Return GB mode color palette registers.
    pub fn pal(&self) -> &Pal {
        &self.pal
    }

    /// Return GB mode color palette registers as mutable.
    pub fn pal_mut(&mut self) -> &mut Pal {
        &mut self.pal
    }

    pub fn video(&self) -> &V {
        &self.video
    }

    pub fn video_mut(&mut self) -> &mut V {
        &mut self.video
    }

    /// Decodes tile data (RGB8 pixel format).
    pub fn tile_data(&self, data: TileDataAddr, bank: usize, out: &mut Vec<Color>) {
        for y in 0..(16 * 8) {
            for x in 0..(16 * 8) {
                let tile = 16 * (y / 8) + (x / 8);
                let col = 7 - (x & 7);
                let row = y & 7;
                let offset = match data {
                    TileDataAddr::X8000 => 16 * (tile as usize) + row * 2,
                    TileDataAddr::X8800 => 0x800 + 16 * (tile as usize) + row * 2,
                };
                // decode color index from tile data
                let lo = self.vram.bank(bank)[offset] >> (col as u8) & 0x1;
                let hi = self.vram.bank(bank)[offset + 1] >> (col as u8) & 0x1;
                let color_index = lo | (hi << 1);
                out.push(palette::GRAYSCALE[color_index as usize]);
            }
        }
    }

    /// Decode tile map pixel data (RGB8 pixel format).
    pub fn tile_map(&self, map: TileMapAddr, data: TileDataAddr, out: &mut Vec<Color>) {
        for y in 0..(8 * 32) {
            for x in 0..(8 * 32) {
                out.push(self.point_color(y, x, map, data).0);
            }
        }
    }

    pub fn step(&mut self, cycles: u64) {
        if self.lcdc_stat.lcdc & 0x80 == 0 {
            return;
        }

        self.dots += cycles;

        let mut line = self.line.ly;

        let next_state = match self.stat_mode {
            StatMode::Search if self.dots >= SEARCH => StatMode::Pixels,
            StatMode::Pixels if self.dots >= PIXELS => StatMode::HBlank,
            StatMode::HBlank if self.dots >= HBLANK => StatMode::Search,
            StatMode::VBlank if self.dots >= VBLANK => StatMode::Search,
            _ => self.stat_mode,
        };

        // handle state transitions.
        match (self.stat_mode, next_state) {
            (StatMode::Search, StatMode::Search) => {}
            (StatMode::Search, StatMode::Pixels) => {
                self.dots %= SEARCH;
                self.stat_mode = StatMode::Pixels;
            }

            (StatMode::Pixels, StatMode::Pixels) => { /* TODO dot by dot */ }
            (StatMode::Pixels, StatMode::HBlank) => {
                self.draw_line(line, 0, LCD_WIDTH);
                self.dots %= PIXELS;
                self.stat_mode = StatMode::HBlank;
                if self.lcdc_stat.stat & STAT_HBLANK_FLAG != 0 {
                    self.request_lcdc();
                }
            }

            (StatMode::HBlank, StatMode::HBlank) => {}
            (StatMode::HBlank, StatMode::Search) if line == 143 => {
                self.dots %= HBLANK;
                self.stat_mode = StatMode::VBlank;
                self.request_vblank();
                if self.lcdc_stat.stat & STAT_VBLANK_FLAG != 0 {
                    self.request_lcdc();
                }
                self.video.draw_video(&self.buffer);
                line = 144;
            }
            (StatMode::HBlank, StatMode::Search) => {
                self.dots %= HBLANK;
                self.stat_mode = StatMode::Search;
                if self.lcdc_stat.stat & STAT_SEARCH_FLAG != 0 {
                    self.request_lcdc();
                }
                line += 1;

                // search 10 visible sprites in new line
                let h = self.lcdc_stat.lcdc_ob_size();
                self.oam.search(line, h);
            }

            (StatMode::VBlank, StatMode::Search) => {
                self.dots %= VBLANK;
                self.stat_mode = StatMode::Search;
                if self.lcdc_stat.stat & STAT_SEARCH_FLAG != 0 {
                    self.request_lcdc();
                }
                line = 0;
            }
            (StatMode::VBlank, StatMode::VBlank) => {
                const DOTS_PER_LINE: u64 = 456;

                if line != 0 {
                    line = 144 + (self.dots / DOTS_PER_LINE) as u8;
                }

                // some documents claim that the LY counter is reset in the middle of the last
                // VBLANK line.
                if line == 153 && self.dots % DOTS_PER_LINE > DOTS_PER_LINE / 2 {
                    line = 0;
                }
            }

            _ => panic!(),
        }

        // LY=LYC
        if line == self.line.lyc {
            if line != self.line.ly && self.lcdc_stat.stat & STAT_LYC_LY_FLAG != 0 {
                self.request_lcdc();
            }
            self.lcdc_stat.stat |= 0b0000_0100;
        } else {
            self.lcdc_stat.stat &= 0b1111_1011;
        }

        self.line.ly = line;
        self.lcdc_stat.stat_set_mode(self.stat_mode);
    }

    fn request_vblank(&mut self) {
        self.vblank_int = Some(Flag::VBlank);
    }

    fn request_lcdc(&mut self) {
        self.lcdc_int = Some(Flag::LCDCStat);
    }

    // Must be called by the MMU after an update
    pub(crate) fn take_vblank_int(&mut self) -> Option<Flag> {
        self.vblank_int.take()
    }

    // Must be called by the MMU after an update
    pub(crate) fn take_lcdc_int(&mut self) -> Option<Flag> {
        self.lcdc_int.take()
    }

    fn clear_video(&mut self) {
        let color = match self.mode {
            Mode::GB => self.pal.clear_color(),
            Mode::CGB => self.color_pal.clear_color(),
        };
        mem::replace(self.buffer.as_mut(), [[color; LCD_WIDTH]; LCD_HEIGHT]);
        mem::replace(self.color_index.as_mut(), [[0; LCD_WIDTH]; LCD_HEIGHT]);
        self.video.draw_video(&self.buffer);
    }

    fn draw_line(&mut self, ly: u8, offset: usize, dots: usize) {
        let lcdc_0 = self.lcdc_stat.lcdc & 0x1 != 0;

        // Behavior of the LCD registers is different on each game boy model.
        // In CGB mode (game boy color), the BG is always visible.
        let display_bg = match self.mode {
            Mode::GB => lcdc_0,
            Mode::CGB => true,
        };

        if display_bg {
            self.draw_bg(ly, offset, dots);
        }
        let display_win = self.lcdc_stat.lcdc & 0x20 != 0;
        if display_win {
            self.draw_win(ly, offset, dots);
        }
        let display_ob = self.lcdc_stat.lcdc & 0x2 != 0;
        if display_ob {
            self.draw_ob(ly, offset, dots);
        }
    }

    // fetch pixel color from a given coordinate
    // coordinate is relative to the tilemap origin
    #[rustfmt::skip]
    fn point_color(&self, y: usize, x: usize, map: TileMapAddr, data: TileDataAddr) -> (Color, u8) {
        // look up tile index
        let map = map as u16;
        let tile_map_idx = 32 * (y / 8) + (x / 8);
        let tile_map_offset = map as usize - 0x8000;
        let tile = self.vram.bank(0)[tile_map_offset + tile_map_idx];
        let flags = self.vram.bank(1)[tile_map_offset + tile_map_idx];
        // look up tile pixel
        let mut col = 7 - (x & 7);
        let mut row = y & 7;
        // flip tiles in CGB mode
        if let Mode::CGB = self.mode {
            if flags & 0x20 != 0 { col = 7 - col }
            if flags & 0x40 != 0 { row = 7 - row }
        }
        let offset = match data {
            TileDataAddr::X8000 => 16 * (tile as usize) + row * 2,
            TileDataAddr::X8800 => {
                let tile: i8 = unsafe { mem::transmute(tile) };
                let tile = (tile as isize + 128) as usize;
                0x800 + 16 * tile + row * 2
            }
        };
        let bank = match self.mode {
            Mode::GB => 0,
            Mode::CGB => (flags >> 3) & 0x1,
        } as usize;
        // decode color index from tile data
        let lo = self.vram.bank(bank)[offset] >> (col as u8) & 0x1;
        let hi = self.vram.bank(bank)[offset + 1] >> (col as u8) & 0x1;
        let mut color_index = lo | (hi << 1);
        // return pixel color
        match self.mode {
            Mode::GB => (self.pal.bg_color(color_index as usize), color_index),
            Mode::CGB => {
                let palette = (flags & 0x7) as usize;
                let color = self.color_pal.bg_pal_color(palette, color_index as usize);

                // tile priority over all OAM
                // in CGB mode, this tile voerrides any priorities.
                // TODO don't use magic value (4)
                if flags & 0x80 != 0 && color_index != 0 {
                    color_index = 4;
                }

                (color, color_index)
            }
        }
    }

    fn draw_bg(&mut self, ly: u8, offset: usize, dots: usize) {
        let Scroll { scx, scy } = self.scroll;

        for lcd_x in offset..(offset + dots) {
            let y = ly.wrapping_add(scy) as usize;
            let x = (lcd_x as u8).wrapping_add(scx) as usize;
            let map = self.lcdc_stat.bg_tile_map();
            let data = self.lcdc_stat.bg_win_tile_data();
            let (color, index) = self.point_color(y, x, map, data);
            self.buffer[ly as usize][lcd_x] = color;
            self.color_index[ly as usize][lcd_x] = index;
        }
    }

    fn draw_win(&mut self, ly: u8, offset: usize, dots: usize) {
        assert_eq!(0, offset);
        assert_eq!(LCD_WIDTH, dots);

        let Window { wy, wx } = self.win;

        if ly < wy {
            return;
        }

        // The window becomes visible (if enabled) when positions are set in
        // range WX=0..166, WY=0..143. A position of WX=7, WY=0 locates the
        // window at upper left, it is then completely covering normal
        // background.
        let wx = (wx as i16 - 7).max(0);

        for lcd_x in wx..LCD_WIDTH as i16 {
            let y = (ly as i16 - wy as i16) as usize;
            let x = (lcd_x - wx) as usize;
            let map = self.lcdc_stat.win_tile_map();
            let data = self.lcdc_stat.bg_win_tile_data();
            let (color, index) = self.point_color(y, x, map, data);
            self.buffer[ly as usize][lcd_x as usize] = color;
            self.color_index[ly as usize][lcd_x as usize] = index;
        }
    }

    #[rustfmt::skip]
    fn draw_ob(&mut self, ly: u8, offset: usize, dots: usize) {
        assert_eq!(0, offset);
        assert_eq!(LCD_WIDTH, dots);

        let ly = ly as i16;
        let h = self.lcdc_stat.lcdc_ob_size() as i16;

        // TODO fix OAM search
        //  I should be checking all 40 sprites since only 10 are visible per line.
        //  The timming of the PPU is somewhat broken so it crashes often.
        for entry in 0..40 {
            let Entry { ypos, xpos, mut tile, flags } = *self.oam.get(39 - entry);

            // position of the top-left corner of the sprite within the lcd display.
            // This value is signed because when xpos=0 the sprite is offscreen (same for ypos).
            let x = xpos as i16 - 8;
            let y = ypos as i16 - 16;

            // skip entry if it doesn't overlap with the current line
            // TODO Again, this line shouldn't be necessary if the above TODO comment was resolved.
            //  This check should be performed by the OAM search (see method `oam::search`)
            if ly < y || ly >= y + h {
                continue;
            }
            // In 16-pixel mode, the top sprite low bit is always 0 and in the bottom sprite it's 1
            // Initially I thought this should be handled by game code but later I found that some games
            // rely on the PPU performing this AND explicitly.
            //
            // The games that I found rely on this:
            //  - Shantae (sprites break otherwise)
            if h == 16 {
                tile &= 0xfe;
            }

            for lcd_x in x.max(0)..(x + 8).min(LCD_WIDTH as _) {
                // pixel position within the tile
                let mut row = (ly - y) as usize;
                let mut col = 7 - (lcd_x - x) as usize;
                // flip/mirror sprite
                if flags & 0x40 != 0 { row = (h as usize - 1) - row }
                if flags & 0x20 != 0 { col = 7 - col }
                // pretty much the same as in Self::point_color
                let offset = 16 * (tile as usize) + row * 2;
                let bank = match self.mode {
                    Mode::GB => 0,
                    Mode::CGB => (flags >> 3) & 0x1,
                } as usize;
                let lo = self.vram.bank(bank)[offset] >> (col as u8) & 0x1;
                let hi = self.vram.bank(bank)[offset + 1] >> (col as u8) & 0x1;
                let color_index = lo | (hi << 1);

                // discards transparent pixels (color_index = 0)
                // works out tile priority and decide if the sprite pixel is discarded (begind BG)
                // there are a few cases:
                //  - color index 0 is transparent (always discarded)
                //  - If one of the LCDC bits is set and the color index underneath is not 0.
                //  - (CGB only) The BG tile underneath can override any priorities (always discard).
                if color_index == 0 || // transparent pixel
                    flags & 0x80 != 0 && self.color_index[ly as usize][lcd_x as usize] != 0 || // behind colors 1-3
                    self.color_index[ly as usize][lcd_x as usize] == 4 // bg tile OAM priority bits (CGB only)
                {
                    continue;
                }

                // draw pixel color
                self.buffer[ly as usize][lcd_x as usize] = match self.mode {
                    Mode::GB => {
                        let pal = (flags >> 4 & 0x1) as usize;
                        self.pal.obp_color(pal, color_index as usize)
                    }
                    Mode::CGB => {
                        let palette = (flags & 0x7) as usize;
                        self.color_pal.ob_pal_color(palette, color_index as usize)
                    },
                };
            }
        }
    }
}

// FIXME parts of the PPU are not accessible deppending on the current state,
//  but these gates are commented out due to timming bugs.
impl<V: Video> Device for Ppu<V> {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9fff => self.vram.read(addr),
            0xfe00..=0xfe9f => {
                // if matches!(self.stat_mode, StatMode::HBlank | StatMode::VBlank) {
                self.oam.read(addr)
                // } else {
                //     0
                // }
            }
            0xff40 | 0xff41 => self.lcdc_stat.read(addr),
            0xff42 | 0xff43 => self.scroll.read(addr),
            0xff44 => self.line.ly,
            0xff45 => self.line.lyc,
            0xff4a | 0xff4b => self.win.read(addr),
            0xff47..=0xff49 => self.pal.read(addr),
            0xff4f => {
                // if matches!(
                //     self.stat_mode,
                //     StatMode::Search | StatMode::HBlank | StatMode::VBlank
                // ) {
                self.vram.read(addr)
                // } else {
                //     0
                // }
            }
            0xff68..=0xff6b => {
                // if matches!(
                //     self.stat_mode,
                //     StatMode::Search | StatMode::HBlank | StatMode::VBlank
                // ) {
                self.color_pal.read(addr)
                // } else {
                //     0
                // }
            }
            _ => panic!(),
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x8000..=0x9fff => self.vram.write(addr, data),
            0xfe00..=0xfe9f => {
                // if matches!(self.stat_mode, StatMode::HBlank | StatMode::VBlank) {
                self.oam.write(addr, data)
                // }
            }
            0xff40 | 0xff41 => {
                let lcdc = self.lcdc_stat.lcdc;
                self.lcdc_stat.write(addr, data);

                // LCD display disabled
                if lcdc & 0x80 != 0 && self.lcdc_stat.lcdc & 0x80 == 0 {
                    self.dots = 0;
                    self.line.ly = 0;
                    self.stat_mode = StatMode::HBlank;
                    self.lcdc_stat.stat_set_mode(self.stat_mode);

                    // clear video output
                    self.clear_video();
                }
            }
            0xff42 | 0xff43 => self.scroll.write(addr, data),
            0xff44 => {
                // to The LY indicates the vertical line to which the present
                // data is transferred to the LCD Driver. The LY can take on any
                // value between 0 through 153. The values between 144 and 153
                // indicate the V-Blank period. Writing will reset the counter.

                self.dots = 0;
                self.line.ly = 0;
                self.stat_mode = StatMode::Search;
                self.lcdc_stat.stat_set_mode(self.stat_mode);
            }
            0xff45 => self.line.lyc = data,
            0xff4a | 0xff4b => {
                if addr == 0xff4a && data != 0 {
                    //eprintln!("WY = {}, LY = {}", data, self.line.ly);
                }
                self.win.write(addr, data)
            }
            0xff47..=0xff49 => self.pal.write(addr, data),
            0xff4f => {
                // if matches!(
                //     self.stat_mode,
                //     StatMode::Search | StatMode::HBlank | StatMode::VBlank
                // ) {
                self.vram.write(addr, data)
                // }
            }
            0xff68..=0xff6b => {
                // if matches!(
                //     self.stat_mode,
                //     StatMode::Search | StatMode::HBlank | StatMode::VBlank
                // ) {
                self.color_pal.write(addr, data)
                // }
            }
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{device::device::Device, mmu::mmu::Mmu, Mode};

    #[test]
    fn vram() {
        let mut mmu = Mmu::<_, _, ()>::new(Mode::GB, (), ());

        mmu.write(0x8000, 1);
        mmu.write(0x9fff, 2);

        assert_eq!(1, mmu.read(0x8000));
        assert_eq!(2, mmu.read(0x9fff));
    }

    #[test]
    fn oam() {
        let mut mmu = Mmu::<_, _, ()>::new(Mode::GB, (), ());

        mmu.write(0xfe00, 1);
        mmu.write(0xfe9f, 2);

        assert_eq!(1, mmu.read(0xfe00));
        assert_eq!(2, mmu.read(0xfe9f));
    }

    #[test]
    fn registers() {
        let mut mmu = Mmu::<_, _, ()>::new(Mode::GB, (), ());

        mmu.write(0xff42, 1);
        mmu.write(0xff43, 2);
        mmu.write(0xff44, 3);
        mmu.write(0xff45, 4);
        mmu.write(0xff4a, 5);
        mmu.write(0xff4b, 6);
        mmu.write(0xff47, 7);
        mmu.write(0xff48, 8);
        mmu.write(0xff49, 9);

        assert_eq!(1, mmu.read(0xff42));
        assert_eq!(2, mmu.read(0xff43));
        // The LY indicates the vertical line to which the present data
        // is transferred to the LCD Driver. The LY can take on any
        // value between 0 through 153. The values between 144 and 153
        // indicate the V-Blank period. Writing will reset the counter.
        assert_eq!(0, mmu.read(0xff44));
        assert_eq!(4, mmu.read(0xff45));
        assert_eq!(5, mmu.read(0xff4a));
        assert_eq!(6, mmu.read(0xff4b));
        assert_eq!(7, mmu.read(0xff47));
        assert_eq!(8, mmu.read(0xff48));
        assert_eq!(9, mmu.read(0xff49));
    }
}
