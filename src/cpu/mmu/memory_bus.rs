const VRAM_BEGIN: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;
pub mod gpus;
use self::gpus::gpu::GPU;

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
    pub fn new(boot_rom: Option<[u8; 0xFFFF]>, game_rom: [u8; 0xFFFF]) -> MemoryBus
    {
        MemoryBus
        {
            memory: [0; 0xFFFF], // Initialize the memory array with zeros
            boot_rom,
            game_rom,
            gpu: GPU::new(), // Initialize the GPU instance
        }
    }

    // Read and Write to and from Memory
   pub fn read_byte(&self, address: u16) -> u8
    {
        let address = address as usize;

     
        

        self.memory[address as usize]
    }

   pub fn write_byte(&mut self, address: u16, byte: u8)
    {

        let address = address as usize;
      
        self.gpu.write_vram(address, byte)

    }
}