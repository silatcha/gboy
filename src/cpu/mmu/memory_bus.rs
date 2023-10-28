const VRAM_BEGIN: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;
const RAM_SIZE: usize = 0x2000; // 8KB of RAM
const IO_PORT_SIZE: usize = 0xA0; // Size of I/O ports region

pub mod test;

pub mod inputs;
use inputs::input::EmulatorInput;
use inputs::keypad::Keypad;
use inputs::input::ButtonMode;
use self::test::gpu::GPU;
use std::fs::File;
use std::io::Read;
use self::test::display::Display;



pub struct MemoryBus 
{

    memory: [u8; 0xFFFF],
    boot_rom: Option<[u8; 0xFFFF]>,
    ram: [u8; 0xFFFF],
    pub gpu: GPU,
    io_ports: [u8; IO_PORT_SIZE], // I/O ports
    pub input: EmulatorInput,
    pub keypad: Keypad,
    // TODO: review MMU logic
}

impl MemoryBus
{
    pub fn new(rom_data: Option<[u8; 0xFFFF]>) -> MemoryBus
    {
        MemoryBus
        {
            memory: [0; 0xFFFF],
            boot_rom: rom_data,
            ram: [0; 0xFFFF],
            gpu: GPU::new(), // Initialize the GPU instance
            io_ports: [0; IO_PORT_SIZE],
            input: EmulatorInput::new(),
            keypad: Keypad::new()
        }


    }


    // Read and Write to and from Memory
   pub fn read_byte(&self, address: u16) -> u8
    {
        println!("addresse {}", address as usize);
       // let address = address as usize;

     
       let address = address as usize;

     

        
       match address {
        0x0000..=0x7FFF => {
            // Read from ROM (cartridge)
            if let Some(rom_data) = &self.boot_rom {
                rom_data[address as usize]
            } else {
                // Handle case when no ROM data is present
                println!("_ value error rom");
                0x00
            }
        }
        0x8000..=0x9FFF => {
            // Read from VRAM (Video RAM)
            self.gpu.read_vram(address - VRAM_BEGIN)
        },
        0xA000..=0xBFFF => {
            // Read from cartridge RAM (save data)
            self.ram[address as usize];
            self.memory[address as usize]
        }
        0xFF00 =>self.keypad.rb(),

        
           _ =>{
            println!("_ value error");
            println!("joypad {}",0xFF00);
            0x00
        }

    }
   
    }

   pub fn write_byte(&mut self, address: u16, byte: u8)
    {

        let address = address as usize;
        match address {
            VRAM_BEGIN ..= VRAM_END =>
           {
            self.gpu.write_vram(address, byte)
           },
           0xA000..=0xBFFF => {
            // Write to cartridge RAM (save data)
            self.ram[(address - 0xA000) as usize] = byte;
            self.memory[(address - 0xA000) as usize] = byte;
        }
           0xFF00 => self.keypad.wb(byte),
           // 0x8000..=0x9FFF => self.gpu.write_vram(address, byte), // Write to RAM
            //0xFE00..=0xFE9F => self.io_ports[(address - 0xFE00) as usize] = byte, // Write to I/O ports
            _ => {
                // Unmapped region, do nothing
            }
        };
        
      
       // self.gpu.write_vram(address, byte)

    }




     // Example Joypad I/O port reading method
     fn read_joypad(&self) -> u8 {
        // Implement Joypad read logic here (simplified example)
        // For instance, you can simulate button presses and read the state
        // This is just a placeholder and should be replaced with actual logic
        let mut joypad_state = 0x00;
        println!("READ NOT WORK");

        if self.input.is_a_button_pressed() {
            joypad_state |= 0x01; // Set Bit 0 for A button
        }
        // Repeat for other buttons and directional keys

        joypad_state
    }

     // Example Joypad I/O port writing method
     fn write_joypad(&mut self, value: u8) {
       // Bit 4 and Bit 5 of the written value determine the button mode
       let button_mode = value & 0x30;

       // Update the EmulatorInput's button mode
       match button_mode {
           0x10 => self.input.set_button_mode(ButtonMode::ButtonKeys), // Button keys selected
           0x20 => self.input.set_button_mode(ButtonMode::DirectionalKeys), // Directional keys selected
           _ => {} // Other modes are not supported (set to "None")
       }
    }
}




