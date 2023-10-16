const VRAM_BEGIN: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;
pub mod gpus;
use self::gpus::gpu::GPU;
use std::fs::File;
use std::io::Read;

pub struct MemoryBus 
{

    memory: [u8; 0xFFFF],
    boot_rom: Option<[u8; 0xFFFF]>,
    game_rom: [u8; 0xFFFF],
    gpu: GPU
    // TODO: review MMU logic
}

impl MemoryBus
{
    pub fn new(boot_rom_path: Option<&str>, game_rom_path: &str) -> MemoryBus
    {
        let mut memory_bus= MemoryBus
        {
            memory: [0; 0xFFFF],
            boot_rom: None,
            game_rom: [0; 0xFFFF],
            gpu: GPU::new(), // Initialize the GPU instance
        };

      
        
        // Load the boot ROM
        if let Some(boot_rom_path) = boot_rom_path {
            memory_bus.load_boot_rom(boot_rom_path);
        }
        
        // Load the game ROM
        memory_bus.load_game_rom(game_rom_path);

        memory_bus
    }

    fn load_boot_rom(&mut self, file_path: &str) {
        if let Ok(mut file) = File::open(file_path) {
            if let Ok(bytes_read) = file.read(&mut self.memory) {
                println!("Loaded {} bytes from boot ROM.", bytes_read);
                self.boot_rom = Some(self.memory);
            } else {
                eprintln!("Failed to read boot ROM.");
            }
        } else {
            eprintln!("Failed to open boot ROM file.");
        }
    }

    fn load_game_rom(&mut self, file_path: &str) {
        if let Ok(mut file) = File::open(file_path) {
            if let Ok(bytes_read) = file.read(&mut self.game_rom) {
                println!("Loaded {} bytes from game ROM.", bytes_read);
            } else {
                eprintln!("Failed to read game ROM.");
            }
        } else {
            eprintln!("Failed to open game ROM file.");
        }
    }

    // Read and Write to and from Memory
   pub fn read_byte(&self, address: u16) -> u8
    {
        println!("addresse {}", address as usize);
        let address = address as usize;

     
        

        self.memory[address as usize]
    }

   pub fn write_byte(&mut self, address: u16, byte: u8)
    {

        let address = address as usize;
      
        self.gpu.write_vram(address, byte)

    }
}