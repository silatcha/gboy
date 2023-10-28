// Define a trait for display objects
pub trait Display {
    fn set_pixel(&mut self, x: usize, y: usize, color: u32);
    fn update(&mut self);
}

// Example: Implement the trait for a basic console display
pub struct ConsoleDisplay {
    width: usize,
    height: usize,
    pixels: Vec<u32>,
}

impl ConsoleDisplay {
   pub  fn new(width: usize, height: usize) -> Self {
        let pixels = vec![0; width * height]; // Initialize with black pixels
        ConsoleDisplay {
            width,
            height,
            pixels,
        }
    }

   pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.pixels[index] = color;
        }
    }


  pub  fn update(&self)->usize {
        // For a console display, printing the pixels is a simple way to update
        let mut ind: usize=0;
        for y in 0..self.height {
            for x in 0..self.width {
                ind = y * self.width + x;
                print!("{:06X} ", self.pixels[ind]);
            }
            println!(); // Start a new line for the next row
        }
        ind
    }
}



