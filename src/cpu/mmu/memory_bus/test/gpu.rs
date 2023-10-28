/* Tiles are stored between 0x8000 and 0x97FF (6144 bytes)
 * 
 * 8000 - 87FF
 *      Tile Set #1 - PART I
 * 8800 - 8FFF
 *      Tile Set #1 - PART II
 *      Tile Set #2 - PART I
 * 9000 - 97FF
 *      Tile Set #2 - PART II
 */ 

const TILES_SIZE: usize = 384;

const VRAM_BEGIN: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = (VRAM_END - VRAM_BEGIN) + 1;
const OAM_SIZE: usize = 160;
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;


/* The bit value to colour mapping is as follows:
 *
 * 0b11 : white
 * 0b10 : dark-grey
 * 0b01 : light-grey
 * 0b00 : black
 */

 mod frame_buffer;

 use frame_buffer::Framebuffer;
 use super::display::Display;
 




/// All possible color values on the original game boy
#[derive(Clone,Copy,PartialEq,Eq)]
pub enum Colors {
    White     = 0,
    LightGrey = 1,
    DarkGrey  = 2,
    Black     = 3,
}

impl Colors {
    /// Create a color from a u8 in the range 0...3
    fn from_u8(c: u8) -> Colors {
        match c {
            0 => Colors::White,
            1 => Colors::LightGrey,
            2 => Colors::DarkGrey,
            3 => Colors::Black,
            _ => panic!("Invalid color: 0x{:02x}", c),
        }
    }
}


 #[derive(Copy,Clone)]
enum TilePixelValue 
{
    WHITE,
    DarkGrey,
    LightGrey,
    BLACK,
}


// Each Tile is encoded in 16 bytes.
type Tile = [[TilePixelValue; 8]; 8];

fn empty_tile() -> Tile 
{
    [[TilePixelValue::WHITE; 8]; 8]
}

pub struct GPU
{
    vram: [u8; VRAM_SIZE],
    tile_set: [Tile; TILES_SIZE],
    oam: [u8; OAM_SIZE],   // Object Attribute Memory (Sprite data)
    lcdc: u8,              // LCD Control register
    scanline: u8,          // Current scanline (0-153)
    screen_buffer: [bool; SCREEN_WIDTH * SCREEN_HEIGHT], // Pixel buffer for the screen
    frame_buffer: Framebuffer,
    display: Display,
    line_cache: [[Option<u8>; 10]; 144],
    window_enabled:         bool,

}

impl GPU
{

    pub fn new() -> GPU {
       
        GPU {
            vram: [0; VRAM_SIZE],      // Initialize vram with zeros
            tile_set: [empty_tile(); TILES_SIZE],  // Initialize tile_set with Tile instances
            oam: [0; OAM_SIZE],
            lcdc: 0,
            scanline: 0,
            screen_buffer: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            frame_buffer: Framebuffer::new(),
            display:Display::new(1),
            line_cache:             [[None; 10]; 144],
            window_enabled:         false,
        }
    }

    // Read and Write to and from VRAM array
   pub fn write_vram(&mut self, address: usize, value: u8)
    {
        // If the address is greater than 0x1800, abort
        if address >= 0x1800
        {
            return
        }

        self.vram[address] = value;

        // Tiles rows are encoded in 16-bit, the first bit is always on an even address.
        // Using AND between tis value and 0xFFFE gives the address of the first 8-bits.
        let normalised_address = address & 0xFFFE;

        // Retrieve 16 bits that encodes the tile row.
        let tile_byte_1 = self.vram[normalised_address];
        let tile_byte_2 = self.vram[normalised_address + 1];

        // A tile is 8 rows tall, each encoded in 16 bits.
        let tile_index = address / 16;

        // Every 16 bits is a row
        let row_index = (address % 16) / 2;

        // Loop through the row of tiles (8 times)
        for pixel_index in 0..8
        {
            // Shift the pixels to read the bits left to right.
            let mask = 1 << (7 - pixel_index);

            let lsb = tile_byte_1 & mask;
            let msb = tile_byte_2 & mask;

            // Assess the pixel values
            let value = match( lsb != 0, msb != 0)
            {
                (true,false) => TilePixelValue::WHITE,
                (true,true) => TilePixelValue::LightGrey,
                (false,true) => TilePixelValue::DarkGrey,
                (false,false) => TilePixelValue::BLACK,
            };

            // Set the pixel's value to render
            self.tile_set[tile_index][row_index][pixel_index] = value;
        }
    }

  pub  fn read_vram(&self, address: usize) -> u8
    {
        self.vram[address]
    }




    pub fn render_scanline(&mut self,display_buffer: &mut [u32; SCREEN_WIDTH]) {
        let lcdc = self.lcdc;

        // Check if the LCD is enabled
        if lcdc & 0x80 == 0 {
            // LCD is disabled, do nothing for this scanline
            return;
        }

        // Implement scanline rendering based on the current display mode
        let current_mode = self.scanline_mode();

        match current_mode {
            0 => {
                self.increment_scanline();
              let pixel=  Framebuffer::copy_framebuffer_to_display(&mut self.frame_buffer);
              self.screen_buffer[pixel ]^=true;
               // self.copy_framebuffer_to_display(display_buffer);
                // Mode 0 - Horizontal Blank (HBlank)
                // In HBlank mode, the GPU does nothing, just increments the scanline counter.
                // You may need to update scanline-related registers.
            }
            1 => {
                self.increment_scanline(); // Increment the scanline counter

                // Copy pixel data from framebuffer to display buffer.
                let pixel=  Framebuffer::copy_framebuffer_to_display(&mut self.frame_buffer);
                self.screen_buffer[pixel]^=true;
                // Mode 1 - Vertical Blank (VBlank)
                // In VBlank mode, the GPU enters the VBlank period and can draw to the screen buffer.
                // You should implement screen drawing logic here and update the screen buffer.
            }
            2 => {
                self.increment_scanline();

                let pixel=  Framebuffer::copy_framebuffer_to_display(&mut self.frame_buffer);
              self.screen_buffer[pixel]^=true;
                // Mode 2 - Searching OAM (Object Attribute Memory)
                // In this mode, the GPU searches the OAM for sprite information.
                // You might handle sprite rendering and priority here.
            }
            3 => {
                self.increment_scanline();
                let pixel=  Framebuffer::copy_framebuffer_to_display(&mut self.frame_buffer);
                self.screen_buffer[pixel]^=true;
                //self.copy_framebuffer_to_display(display_buffer);
                // Mode 3 - Transfer Data to LCD Driver (Drawing)
                // In this mode, the GPU transfers data from VRAM to the LCD driver.
                // Implement background rendering here.
            }
            _ => {
                // Unknown mode (should not occur)
            }
        }
    }

    fn scanline_mode(&self) -> u8 {
        // Implement your logic to determine the current mode based on scanline position
        // You'll need to consider timing and scanline-related registers.
        const HBLANK_START: u8 = 0;
        const HBLANK_END: u8 = 51;
        const VBLANK_START: u8 = 144;
        const VBLANK_END: u8 = 153;
        const OAM_START: u8 = 20;
        const OAM_END: u8 = 252;
        const DRAWING_START: u8 = 77;
        const DRAWING_END: u8 = 239;

        let scanline = self.scanline;

        if scanline < HBLANK_START || scanline > HBLANK_END {
            return 0; // HBlank
        } else if scanline >= VBLANK_START && scanline <= VBLANK_END {
            return 1; // VBlank
        } else if scanline >= OAM_START && scanline <= OAM_END {
            return 2; // OAM
        } else if scanline >= DRAWING_START && scanline <= DRAWING_END {
            return 3; // Drawing
        } else {
            return 0; // Default to HBlank
        }
    
    }


    fn increment_scanline(&mut self) {
        // Increment the scanline counter and reset if it reaches the end.
        self.scanline += 1;

        if self.scanline > 153 {
            self.scanline = 0;
        }
        
    }

    pub fn render_frame(&mut self) {
        // Render a complete frame (all scanlines)
        for _ in 0..SCREEN_HEIGHT {
            self.render_scanline(&mut [0; 64]);
        }
    }

    pub fn read_oam(&self, address: u16) -> u8 {
        self.oam[address as usize]
    }

    pub fn write_oam(&mut self, address: u16, value: u8) {
        self.oam[address as usize] = value;
    }

    pub fn read_lcdc(&self) -> u8 {
        self.lcdc
    }

    pub fn write_lcdc(&mut self, value: u8) {
        self.lcdc = value;
    }

    pub fn get_screen_buffer(&self) -> &[bool] {
        &self.screen_buffer
    }







    /// Render a single pixel from the display
    fn render_pixel(&mut self, x: u8, y: u8) {
        let bg_col =
            // Window is always on top of background
            if self.window_enabled && self.in_window(x, y) {
                self.window_color(x, y)
            } else if self.bg_enabled {
                self.background_color(x, y)
            } else {
                // No background or window
                AlphaColor { color: Colors::White, opaque: false }
            };

        let col = if self.sprites_enabled {
            self.render_sprite(x, y, bg_col)
        } else {
            bg_col.color
        };

        self.display.set_pixel(x as u32, y as u32, col);
    }

    fn render_sprite(&self, x: u8, y: u8, bg_col: AlphaColor) -> Colors {

        for &entry in self.line_cache[y as usize].iter() {
            match entry {
                None        => break, // Nothing left in cache
                Some(index) => {
                    let sprite = &self.oam[index as usize];

                    let sprite_x = (x as i32) - sprite.left_column();

                    if sprite_x >= 8 {
                        // Sprite was earlier on the line
                        continue
                    }

                    if sprite_x < 0 {
                        // It's too early to draw that sprite. Since
                        // sprites are in order on the line we know
                        // there's no sprite remaining to be drawn
                        break;
                    }

                    if sprite.background() && bg_col.opaque {
                        // Sprite is covered by the background
                        continue;
                    }

                    let sprite_y = (y as i32) - sprite.top_line();

                    let (height, tile) = match self.sprite_size {
                        SpriteSize::Sz8x8  => (7, sprite.tile()),
                        // For 16pix tiles the LSB is ignored
                        SpriteSize::Sz8x16 => (15, sprite.tile() & 0xfe),
                    };

                    let sprite_y = match sprite.y_flip() {
                        true  => height - sprite_y,
                        false => sprite_y,
                    };

                    let sprite_x = match sprite.x_flip() {
                        true  => 7 - sprite_x,
                        false => sprite_x,
                    };

                    // Sprites always use TileSet 1
                    let pix = self.pix_color(tile,
                                             sprite_x as u8,
                                             sprite_y as u8,
                                             TileSet::Set1);

                    // White color (0) pre-palette denotes a
                    // transparent pixel
                    if pix != Colors::White {
                        // Pixel is not transparent, compute the color
                        // and return that

                        let palette = match sprite.palette() {
                            sprite::Palette::Obp0 => self.obp0,
                            sprite::Palette::Obp1 => self.obp1,
                        };


                        return palette.transform(pix);
                    }
                }
            }
        }

        bg_col.color
    }


    }

    





  
