use std::cell::RefCell;
#[derive(Copy,Clone)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

mod display;
use display::Display;
use display::ConsoleDisplay;
const WIDTH: usize = 64;  // Adjust to your display dimensions
const HEIGHT: usize = 32;

pub struct Framebuffer {
    data: Vec<Pixel>,
    display: ConsoleDisplay

}

impl Framebuffer {
   pub fn new() -> Self {
        let data = vec![Pixel { r: 0, g: 0, b: 0 }; WIDTH * HEIGHT]; // Initialize with black pixels
        Framebuffer { 
            data,
            display:ConsoleDisplay::new(64,32)
         }
    }

    pub fn copy_framebuffer_to_display(&mut self)->usize {
        let mut color_value:u32;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let pixel = self.data[y * WIDTH + x];
    
                // Convert the Pixel format to your display format
                 color_value = convert_pixel_to_display_format(&pixel);
    
                // Copy the color value to the display
                self.display.set_pixel(x, y, color_value);
            }
        }
    
        // Update the display to reflect the copied data
        self.display.update()
        
    }

}

pub fn convert_pixel_to_display_format(pixel: &Pixel) -> u32 {
    // Assuming a simple 24-bit RGB format (8 bits per channel)
    let red = (pixel.r as u32) << 16;
    let green = (pixel.g as u32) << 8;
    let blue = pixel.b as u32;

    // Combine the color channels to create the display format color value
    let color_value = red | green | blue;

    color_value
}


