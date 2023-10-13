

//mod gpu;
use gpus::gpu;
use register::flags_register;
pub mod instructions;
mod mmu;

use self::instruction::{
    ArithmeticTarget,
    ADDHLTarget,
    JumpTest,
    LoadByteSource,
    LoadByteTarget,
    LoadType,
    StackTarget,
};

use self::registers::Registers;

use super::gpus;

pub struct CPU
{
    pub registers: Registers,
    pub bus: MemoryBus,

    // 16-bit registers
    program_counter: u16,
    stack_pointer: u16,

    is_halted: bool,
    is_interrupted: bool,
}

impl CPU
{
    // Constructor
    pub fn new(boot_rom: Option<Vec<u8>>, game_rom: Vec<u8>) -> CPU
    {
        CPU
        {
            registers: Registers::new(),
            program_counter: 0x0,
            stack_pointer: 0x0,
            bus: MemoryBus::new(boot_rom, game_rom),
            is_halted: false,
            is_interrupted: true,
        }
    }

    // Program Counter's step to next OpCode
    pub fn step(&mut self) -> u8
    {
        // Read the instruction byte from memory using Program Counter register
        let mut instruction_byte = self.bus.read_byte(self.program_counter);

        // Check if the byte we read from memory is 0xCB, if it is, we read one
        // more byte and interpret the current as a "prefix instruction"
        let prefixed = (instruction_byte == 0xCB);

        if prefixed
        {
            instruction_byte = self.bus.read_byte(self.program_counter + 1);
        }

        // Translate the byte to one of the instancse of the Instruction enum
        let next_program_counter = if let Some(instruction) = Instruction::from_byte(instruction_byte)
        {
            self.execute(instruction)
        }
        else
        {
            let error_description = format!("0x{}{:x}", if prefixed { "cb" } else { "" }, instruction_byte);
            panic!("Unknown instruction found for: {}", error_description);
        };

        self.program_counter = next_program_counter;
    }


    // Executes OpCodes
    pub fn execute(&mut self, instruction: Instruction)
    {
        match instruction
        {
            // ADD
            Instruction::ADD(target) =>
            {
               let addA= match target
                {
                    ArithmeticTarget::C =>
                    {
                        let value = self.registers.C;
                        let new_value = self.add(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::B =>
                    {
                        let value = self.registers.B;
                        let new_value = self.add(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::A =>
                    {
                        let value = self.registers.A;
                        let new_value = self.add(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::D =>
                    {
                        let value = self.registers.D;
                        let new_value = self.add(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::E =>
                    {
                        let value = self.registers.E;
                        let new_value = self.add(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::H =>
                    {
                        let value = self.registers.H;
                        let new_value = self.add(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::L =>
                    {
                        let value = self.registers.L;
                        let new_value = self.add(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    }

                };
                
                    self.add(addA)
                
            }



            Instruction::ADDHL(target) =>
            {
               let addA= match target
                {
                    ADDHLTarget::BC =>
                    {
                        let value = self.registers.get_BC();
                        let new_value = self.add(value);
                        self.registers.set_HL(new_value);
                        self.program_counter.wrapping_add(1)
                    },
                    ADDHLTarget::DE =>
                    {
                        let value = self.registers.get_DE();
                        let new_value = self.add(value);
                        self.registers.set_HL(new_value);
                        self.program_counter.wrapping_add(1)
                    },
                   
                    ADDHLTarget::HL =>
                    {
                        let value = self.registers.get_HL();
                        let new_value = self.add(value);
                        self.registers.set_HL(new_value);
                        self.program_counter.wrapping_add(1)
                    },
                    ADDHLTarget::SP =>
                    {
                        let value = self.registers.SP;
                        let new_value = self.add(value);
                        self.registers.set_HL(new_value);
                        self.program_counter.wrapping_add(1)
                    },
                    

                };
                
                    self.addhl(addA)
                
            }

            // RES
            Instruction::RES(target, bitPosition) =>
            {
                
                let prefix = match target
                {
                    PrefixTarget::A => {
                        let value =self.registers.A;
                        self.registers.A=  self.res(bitPosition,value);
                        self.program_counter.wrapping_add(2);
                    },
                    PrefixTarget::B => {
                        let value =self.registers.B;
                        self.registers.B=  self.res(bitPosition,value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::C => {
                        let value =self.registers.C;
                        self.registers.C=  self.res(bitPosition,value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::D => {
                        let value =self.registers.D;
                        self.registers.D=  self.res(bitPosition,value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::E => {
                        let value =self.registers.E;
                        self.registers.E=  self.res(bitPosition,value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::H => {
                        let value =self.registers.H;
                        self.registers.H=  self.res(bitPosition,value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::L => {
                        let value =self.registers.L;
                        self.registers.L=  self.res(bitPosition,value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::HLI => {
                        let value =self.registers.get_HL();
                        self.registers.set_HL( self.res(bitPosition,value));
                        self.program_counter.wrapping_add(2);},
                };
               
                self.res(bitPosition,prefix)
            }
             // RL
             Instruction::RL(target) =>
             {
                 
                 let prefix = match target
                 {
                    PrefixTarget::A => {
                        let value =self.registers.A;
                        self.registers.A=  self.rl(value);
                        self.program_counter.wrapping_add(2);
                    },
                    PrefixTarget::B => {
                        let value =self.registers.B;
                        self.registers.B=  self.rl(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::C => {
                        let value =self.registers.C;
                        self.registers.C=  self.rl(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::D => {
                        let value =self.registers.D;
                        self.registers.D=  self.rl(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::E => {
                        let value =self.registers.E;
                        self.registers.E=  self.rl(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::H => {
                        let value =self.registers.H;
                        self.registers.H=  self.rl(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::L => {
                        let value =self.registers.L;
                        self.registers.L=  self.rl(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::HLI => {
                        let value =self.registers.get_HL();
                        self.registers.set_HL( self.rl(value));
                        self.program_counter.wrapping_add(2);},
                 };
              
                 self.rl(prefix)
             }

             // RLC
             Instruction::RLC(target) =>
             {
                 
                 let prefix = match target
                 {
                    PrefixTarget::A => {
                        let value =self.registers.A;
                        self.registers.A=  self.rlc(value);
                        self.program_counter.wrapping_add(2);
                    },
                    PrefixTarget::B => {
                        let value =self.registers.B;
                        self.registers.B=  self.rlc(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::C => {
                        let value =self.registers.C;
                        self.registers.C=  self.rlc(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::D => {
                        let value =self.registers.D;
                        self.registers.D=  self.rlc(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::E => {
                        let value =self.registers.E;
                        self.registers.E=  self.rlc(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::H => {
                        let value =self.registers.H;
                        self.registers.H=  self.rlc(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::L => {
                        let value =self.registers.L;
                        self.registers.L=  self.rlc(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::HLI => {
                        let value =self.registers.get_HL();
                        self.registers.set_HL( self.rlc(value));
                        self.program_counter.wrapping_add(2);},
                 };
                
                 self.rlc(prefix)
             }

             // RR
             Instruction::RR(target) =>
             {
                 
                 let prefix = match target
                 {
                    PrefixTarget::A => {
                        let value =self.registers.A;
                        self.registers.A=  self.rr(value);
                        self.program_counter.wrapping_add(2);
                    },
                    PrefixTarget::B => {
                        let value =self.registers.B;
                        self.registers.B=  self.rr(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::C => {
                        let value =self.registers.C;
                        self.registers.C=  self.rr(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::D => {
                        let value =self.registers.D;
                        self.registers.D=  self.rr(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::E => {
                        let value =self.registers.E;
                        self.registers.E=  self.rr(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::H => {
                        let value =self.registers.H;
                        self.registers.H=  self.rr(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::L => {
                        let value =self.registers.L;
                        self.registers.L=  self.rr(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::HLI => {
                        let value =self.registers.get_HL();
                        self.registers.set_HL( self.rr(value));
                        self.program_counter.wrapping_add(2);},
                 };
                
                 self.rr(prefix)
             }


             // RRc
             Instruction::RRC(target) =>
             {
                 
                 let prefix = match target
                 {
                    PrefixTarget::A => {
                        let value =self.registers.A;
                        self.registers.A=  self.rrc(value);
                        self.program_counter.wrapping_add(2);
                    },
                    PrefixTarget::B => {
                        let value =self.registers.B;
                        self.registers.B=  self.rrc(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::C => {
                        let value =self.registers.C;
                        self.registers.C=  self.rrc(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::D => {
                        let value =self.registers.D;
                        self.registers.D=  self.rrc(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::E => {
                        let value =self.registers.E;
                        self.registers.E=  self.rrc(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::H => {
                        let value =self.registers.H;
                        self.registers.H=  self.rrc(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::L => {
                        let value =self.registers.L;
                        self.registers.L=  self.rrc(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::HLI => {
                        let value =self.registers.get_HL();
                        self.registers.set_HL( self.rrc(value));
                        self.program_counter.wrapping_add(2);},
                 };
                
                 self.rrc(prefix)
             }


              // RST
              Instruction::RST(target) =>
              {
                  
                  
                self.program_counter.wrapping_add(1);
                  self.rst(target);
              }


              // SCF
              Instruction::SCF(target) =>
              {
                  
                  
                self.program_counter.wrapping_add(1);
                  self.SCF(target);
              }

              // SET
              Instruction::SET(target,bitPosition) =>
              {
                  
                  
                let prefix = match target
                {
                    PrefixTarget::A => {
                        let value =self.registers.A;
                        self.registers.A=  self.set(bitPosition,value);
                        self.program_counter.wrapping_add(2);
                    },
                    PrefixTarget::B => {
                        let value =self.registers.B;
                        self.registers.B=  self.set(bitPosition,value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::C => {
                        let value =self.registers.C;
                        self.registers.C=  self.set(bitPosition,value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::D => {
                        let value =self.registers.D;
                        self.registers.D=  self.set(bitPosition,value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::E => {
                        let value =self.registers.E;
                        self.registers.E=  self.set(bitPosition,value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::H => {
                        let value =self.registers.H;
                        self.registers.H=  self.set(bitPosition,value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::L => {
                        let value =self.registers.L;
                        self.registers.L=  self.set(bitPosition,value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::HLI => {
                        let value =self.registers.get_HL();
                        self.registers.set_HL( self.set(bitPosition,value));
                        self.program_counter.wrapping_add(2);},
                };
               
                self.set(bitPosition,prefix)
              }

              // SLA
              Instruction::SLA(target) =>
              {
                  
                  
                let prefix = match target
                {
                    PrefixTarget::A => {
                        let value =self.registers.A;
                        self.registers.A=  self.sla(value);
                        self.program_counter.wrapping_add(2);
                    },
                    PrefixTarget::B => {
                        let value =self.registers.B;
                        self.registers.B=  self.sla(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::C => {
                        let value =self.registers.C;
                        self.registers.C=  self.sla(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::D => {
                        let value =self.registers.D;
                        self.registers.D=  self.sla(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::E => {
                        let value =self.registers.E;
                        self.registers.E=  self.sla(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::H => {
                        let value =self.registers.H;
                        self.registers.H=  self.sla(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::L => {
                        let value =self.registers.L;
                        self.registers.L=  self.sla(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::HLI => {
                        let value =self.registers.get_HL();
                        self.registers.set_HL( self.sla(value));
                        self.program_counter.wrapping_add(2);},
                };
               
                self.sla(prefix)
              }

              // SRA
              Instruction::SRA(target) =>
              {
                  
                  
                let prefix = match target
                {
                    PrefixTarget::A => {
                        let value =self.registers.A;
                        self.registers.A=  self.sra(value);
                        self.program_counter.wrapping_add(2);
                    },
                    PrefixTarget::B => {
                        let value =self.registers.B;
                        self.registers.B=  self.sra(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::C => {
                        let value =self.registers.C;
                        self.registers.C=  self.sra(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::D => {
                        let value =self.registers.D;
                        self.registers.D=  self.sra(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::E => {
                        let value =self.registers.E;
                        self.registers.E=  self.sra(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::H => {
                        let value =self.registers.H;
                        self.registers.H=  self.sra(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::L => {
                        let value =self.registers.L;
                        self.registers.L=  self.sra(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::HLI => {
                        let value =self.registers.get_HL();
                        self.registers.set_HL( self.sra(value));
                        self.program_counter.wrapping_add(2);},
                };
               
                self.sra(prefix)
              }

              // SRL
              Instruction::SRL(target) =>
              {
                  
                  
                let prefix = match target
                {
                    PrefixTarget::A => {
                        let value =self.registers.A;
                        self.registers.A=  self.srl(value);
                        self.program_counter.wrapping_add(2);
                    },
                    PrefixTarget::B => {
                        let value =self.registers.B;
                        self.registers.B=  self.srl(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::C => {
                        let value =self.registers.C;
                        self.registers.C=  self.srl(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::D => {
                        let value =self.registers.D;
                        self.registers.D=  self.srl(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::E => {
                        let value =self.registers.E;
                        self.registers.E=  self.srl(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::H => {
                        let value =self.registers.H;
                        self.registers.H=  self.srl(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::L => {
                        let value =self.registers.L;
                        self.registers.L=  self.srl(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::HLI => {
                        let value =self.registers.get_HL();
                        self.registers.set_HL( self.srl(value));
                        self.program_counter.wrapping_add(2);},
                };
               
                self.srl(prefix)
              }

              // SRL
              Instruction::SWAP(target) =>
              {
                  
                  
                let prefix = match target
                {
                    PrefixTarget::A => {
                        let value =self.registers.A;
                        self.registers.A=  self.swap(value);
                        self.program_counter.wrapping_add(2);
                    },
                    PrefixTarget::B => {
                        let value =self.registers.B;
                        self.registers.B=  self.swap(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::C => {
                        let value =self.registers.C;
                        self.registers.C=  self.swap(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::D => {
                        let value =self.registers.D;
                        self.registers.D=  self.swap(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::E => {
                        let value =self.registers.E;
                        self.registers.E=  self.swap(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::H => {
                        let value =self.registers.H;
                        self.registers.H=  self.swap(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::L => {
                        let value =self.registers.L;
                        self.registers.L=  self.swap(value);
                        self.program_counter.wrapping_add(2);},
                    PrefixTarget::HLI => {
                        let value =self.registers.get_HL();
                        self.registers.set_HL( self.swap(value));
                        self.program_counter.wrapping_add(2);},
                };
               
                self.swap(prefix)
              }


              // EI
              Instruction::EI =>
              {
                self.program_counter.wrapping_add(1);
                self.ei()
              }

              // EI
              Instruction::DI =>
              {
                self.program_counter.wrapping_add(1);
                self.Di()
              }

            // LD
            Instruction::LD(load_type)     => 
            {
                LoadType::Byte(target, source)  =>
                {
                    let source_value = match source
                    {
                        LoadByteSource::A => self.registers.A,
                        LoadByteSource::D8 => self.read_next_byte(),
                        LoadByteSource::HLI => self.bus.read_byte(self.registers.get_HL()),
                        LoadByteSource::HL => self.bus.read_byte(self.registers.get_HL()),
                        LoadByteSource::B => self.registers.B,
                        LoadByteSource::C => self.registers.C,
                        LoadByteSource::D => self.registers.D,
                        LoadByteSource::E => self.registers.E,
                        LoadByteSource::H => self.registers.H,
                        LoadByteSource::L => self.registers.L,
                        LoadByteSource::BC => self.bus.read_byte(self.registers.get_BC()),
                        LoadByteSource::DE => self.bus.read_byte(self.registers.get_DE())
                    };
                    match target
                    {
                        LoadByteTarget::A => self.registers.A = source_value,
                        LoadByteTarget::HLI => self.bus.write_byte(self.registers.get_HL(), source_value),
                        LoadByteSource::B => self.registers.B = source_value,
                        LoadByteSource::C => self.registers.C = source_value,
                        LoadByteSource::D => self.registers.D = source_value,
                        LoadByteSource::E => self.registers.E = source_value,
                        LoadByteSource::H => self.registers.H = source_value,
                        LoadByteSource::L => self.registers.L = source_value,
                        LoadByteTarget::BC => self.bus.write_byte(self.registers.get_BC(), source_value),
                        LoadByteTarget::DE => self.bus.write_byte(self.registers.get_DE(), source_value),
                        LoadByteTarget::HL => self.bus.write_byte(self.registers.get_HL(), source_value),
                      
                    };
                    match source
                    {
                        LoadByteSource::D8  => self.program_counter.wrapping_add(2),
                        _                   => self.program_counter.wrapping_add(1),
                        LoadByteSource::A => self.program_counter.wrapping_add(1),
                        
                        LoadByteSource::HLI => self.program_counter.wrapping_add(1),
                        LoadByteSource::HL => self.program_counter.wrapping_add(1),
                        LoadByteSource::B => self.program_counter.wrapping_add(1),
                        LoadByteSource::C => self.program_counter.wrapping_add(1),
                        LoadByteSource::D => self.program_counter.wrapping_add(1),
                        LoadByteSource::E => self.program_counter.wrapping_add(1),
                        LoadByteSource::H => self.program_counter.wrapping_add(1),
                        LoadByteSource::L => self.program_counter.wrapping_add(1),
                        LoadByteSource::BC => self.program_counter.wrapping_add(1),
                        LoadByteSource::DE => self.program_counter.wrapping_add(1),
                    }
                    
                }
                LoadType::Word(target) =>
                {


                    match target
                    {
                       
                        LoadByteSource::SP => self.bus.read_next_word(self.registers.SP),
                        LoadByteSource::HL => self.bus.read_next_word(self.registers.get_HL()),
                        LoadByteSource::BC => self.bus.read_next_word(self.registers.get_BC()),
                        LoadByteSource::DE => self.bus.read_next_word(self.registers.get_DE())
                      
                    };
                    match target
                    {
                        LoadByteSource::BC => self.program_counter.wrapping_add(3),
                        LoadByteSource::DE => self.program_counter.wrapping_add(3),
                        LoadByteSource::HL => self.program_counter.wrapping_add(3),
                        LoadByteSource::SP => self.program_counter.wrapping_add(3),
                    }

                }

           }

            
            // CALL
            Instruction::CALL(target) =>
            {
                let jump_condition = match target
                {
                   
                    JumpTest::NZ => !self.registers.F.zero,
                    JumpTest::NC => !self.registers.F.carry,
                    JumpTest::Z  => !self.registers.F.zero,
                    JumpTest::C  => !self.registers.F.carry,
                    JumpTest::A  => true
                    

                    
                };
                self.call(jump_condition)
            }
            // RET
            Instruction::RET(target) =>
            {
                let jump_condition = match target
                {
                   
                    JumpTest::NZ => !self.registers.F.zero,
                    JumpTest::NC => !self.registers.F.carry,
                    JumpTest::Z  => !self.registers.F.zero,
                    JumpTest::C  => !self.registers.F.carry,
                    JumpTest::A  => true
            
                };

                self.return_(jump_condition)
            }

            Instruction::RETI(target) =>
            {
                let jump_condition = match target
                {
                   
                    JumpTest::NZ => !self.registers.F.zero,
                    JumpTest::NC => !self.registers.F.carry,
                    JumpTest::Z  => !self.registers.F.zero,
                    JumpTest::C  => !self.registers.F.carry,
                    JumpTest::A  => true
            
                };

                self.reti(jump_condition)
            }
            // PUSH
            Instruction::PUSH(target) =>
            {
                let value = match target
                {
                    StackTarget::BC => {
                        self.registers.get_BC();
                        self.program_counter.wrapping_add(1);
                    },
                    StackTarget::DE => {
                        self.registers.get_DE();
                        self.program_counter.wrapping_add(1);
                    },
                    StackTarget::HL => {
                        self.registers.get_HL();
                        self.program_counter.wrapping_add(1)
                    },
                    StackTarget::AF => {
                        self.registers.get_AF();
                        self.program_counter.wrapping_add(1);
                    },
                    //self.program_counter.wrapping_add(1);
                    
                };
                self.push(value);
            }
            // POP
            Instruction::POP(target) =>
            {
                let result = self.pop();
                match target
                {
                    StackTarget::BC => self.registers.set_BC(result),
                    StackTarget::DE => self.registers.set_DE(result),
                    StackTarget::HL => self.registers.set_HL(result),
                    StackTarget::AF => self.registers.set_AF(result)
                }
                {
                
                    self.program_counter.wrapping_sub(1)
                }
            }

            // DI
            Instruction::DI =>
            {
                self.di();
            }
            // EI
            Instruction::DI =>
            {
                self.ei();
            }

            // halte
            Instruction::DI =>
            {
                self.halte();
            }
        }
    }
    

    // Accumulate
    fn add(&mut self, value: u8) -> u8
    {
        let (new_value, did_overflow) = self.registers.A.overflowing_add(value);

        // Set the flags
        self.registers.F.zero = (new_value == 0);
        self.registers.F.substract = false;
        self.registers.F.carry = did_overflow;

        // Half Carry is set if adding the lower nibbles of the value and
        // register A together results in a value bigger than 0xF.
        self.registers.F.half_carry = ((self.registers.A & 0xF) + (value + 0xF)) > 0xF;

        new_value
    }

    // Accumulate
    fn addhl(&mut self, value: u8) -> u8
    {
        let (new_value, did_overflow) = self.registers.get_HL().overflowing_add(value);

        // Set the flags
        self.registers.F.zero = (new_value == 0);
        self.registers.F.substract = false;
        self.registers.F.carry = did_overflow;

        // Half Carry is set if adding the lower nibbles of the value and
        // register A together results in a value bigger than 0xF.
        self.registers.F.half_carry = ((self.registers.get_HL() & 0xF) + (value + 0xF)) > 0xF;

        new_value
    }

    // Jump
    fn jump(&self, should_jump: bool) -> u16
    {
        if should_jump
        {
            // GB is Little Endian, ie:
            // PC+2 is MSB and PC+1 is LSB
            let least_significant_byte = self.bus.read_byte(self.program_counter + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.program_counter + 2) as u16;
        }
        else
        {
            // Jump instruction is 3 bytes wide, we still need to move the PC if we don't jump
            self.program_counter.wrapping_add(3)
        }
    }

    // Push
    fn push(&mut self, value: u16)
    {
        // Decrease SP by 1: the Stack grows downward in memory.
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);

        // Write the MSB of the 16-bit value into memory at the location SP is pointing
        self.bus.write_byte(self.stack_pointer,((value & 0xFF00) >> 8) as u8);

        // Decrease SP by 1
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);

        // Write the LSB of the 16-bit value into memory at the location SP is pointing
        self.bus.write_byte(self.stack_pointer,(value & 0x00FF) as u8);
    }

    // Pop
    fn pop(&mut self) -> u16
    {
        // Read the LSB of the 16-bit value which is pointed by the SP
        let least_significant_byte = self.bus.read_byte(self.stack_pointer) as u16;

        // Increase SP by 1
        self.stack_pointer = self.stack_pointer.wrapping_add(1);

        // Read the MSB of the 16-bit value which is pointed by the SP
        let most_significant_byte = self.bus.read_byte(self.stack_pointer) as u16;

        // Increase SP by 1
        self.stack_pointer = self.stack_pointer.wrapping_add(1);

        // Pop the 16-bit value
        (most_significant_byte << 8) | least_significant_byte
    }

    // Call
    fn call(&mut self, should_jump: bool) -> u16
    {
        // Push the next PC on to the stack
        let next_program_counter = self.program_counter.wrapping_add(3);

        // Jump to the address specified in the next 2 bytes of the memory
        if should_jump
        {
            self.push(next_program_counter);
            self.read_next_word()
        }
        else
        {
            next_program_counter
        }
    }

    // Return
    fn return_(&mut self, should_jump: bool) -> u16
    {
        if should_jump
        {
            self.pop()
        }
        else
        {
            self.program_counter.wrapping_add(1)
        }
    }

    // Read and Write functions
    fn read_next_byte(&self) -> u8
    {
        self.bus.read_byte(self.program_counter + 1)
    }

    fn read_next_word(&self) -> u16
    {
        ((self.bus.read_byte(self.program_counter + 2) as u16) << 8) | (self.bus.read_byte(self.program_counter +1) as u16)
    }



    fn ei(&mut self)
    {
        
        self.is_interrupted=true;

        
    }

    fn di(&mut self)
    {
        
        self.is_interrupted=false;  
    }

    fn nope(&mut self)
    {
        
       
    }

    fn halte(&mut self)
    { 
        self.is_halted=true;
    }

    // TODO
    fn stop(&mut self)
    {
        
        self.bus.abort();
    }

    //RET
    fn reti(&mut self)
    {
        self.pop();
        let addr= self.pop();
        self.jump(addr);
        self.is_interrupted=true;
    }


    //RET
    fn ret(&mut self)
    {
        self.pop();
        let addr= self.pop();
        self.jump(addr);
    }

    //RL
    fn rl(&mut self,n:u8) -> u8
    {
        let did_overflow = n & 0x80 == 0x80;
        let new_value =(
            n << 1 |
            (if self.registers.F.carry {0x01} else {0x00})
        );

        self.registers.F.zero = new_value == 0;
        self.registers.F.half_carry=false;
        self.registers.F.substract=false;
        self.registers.F.carry=did_overflow;

        new_value
       
    }



    //RLC
    fn rlc(&mut self,value:u8) -> u8
    {
        // la valeur la plus à gauche
        let did_overflow = value & 0x80 == 0x80;
        let new_value =(
            n << 1 |
            (if did_overflow {0x01} else {0x00}) // on change avec la valeur la plus à droit
        );

        self.registers.F.zero = new_value == 0;
        self.registers.F.half_carry=false;
        self.registers.F.substract=false;
        self.registers.F.carry=did_overflow;

        new_value
       
    }


    //RR
    fn rr(&mut self,value:u8) -> u8
    {
        let did_overflow = value & 0x01 == 0x01;
        let new_value =(
            n >> 1 |
            (if self.registers.F.carry {0x80} else {0x00}) // on change avec la valeur la plus à gauche
        );

        self.registers.F.zero = new_value == 0;
        self.registers.F.half_carry=false;
        self.registers.F.substract=false;
        self.registers.F.carry=did_overflow;

        new_value
       
    }

    //RRC
    fn rrc(&mut self,value:u8) -> u8
    {
        // la valeur la plus a droit
        let did_overflow = value & 0x01 == 0x01;
        let new_value =(
            n >> 1 |
            (if did_overflow {0x80} else {0x00}) // on change avec la valeur la plus à gauche
        );

        self.registers.F.zero = new_value == 0;
        self.registers.F.half_carry=false;
        self.registers.F.substract=false;
        self.registers.F.carry=did_overflow;

        new_value
       
    }

    

    // RES
    fn res(&mut self,b: u8,value: u8) -> u8
    {
        value & !(1 << b)
    }


    //RST
    fn rst(&mut self,value:u8) -> u16
    {
        self.push(value);
        
        self.jump(value as u16)

    }

    //SBC
    fn sbc(&mut self,value:u8) -> u8
    {
        let mut n_adjusted = value;
        if self.registers.F.carry {
            n_adjusted = value.wrapping_sub(1);
        }

        // Perform the subtraction.
        let result = self.registers.A.wrapping_sub(n_adjusted);

        self.registers.F.zero= result==0;

        if self.registers.F.substract {
            self.registers.F.substract=false;
        }else {
            self.registers.F.substract=true;
        }

        self.registers.F.half_carry=(self.registers.A & 0x0F) < (n_adjusted & 0x0F) + if self.registers.F.carry {1} else {0};
        self.registers.F.carry=self.registers.A < n_adjusted + if self.registers.F.carry {1} else {0};

        result

    }


     //SCF
     fn scf(&mut self)
     {
        self.registers.F.carry=true;
        self.registers.F.substract=false;
        self.registers.F.half_carry=false;
 
     }



      //SET
      fn set(&mut self,b: u8, value:u8) -> u8
      {
        value | (1 << b)
      }


      //SLA
      fn sla(&mut self, value:u8) -> u8
      {
        let did_overflow = value & 0x80 == 0x80;
        let new_value = (value & 0x7f) << 1 & 0xfe;

        self.registers.F.zero = new_value == 0;
        self.registers.F.half_carry=false;
        self.registers.F.substract=false;
        self.registers.F.carry=did_overflow;

        new_value
       
      }

      fn SRA(&mut self, value:u8) -> u8
      {
        let did_overflow = value & 0x80 == 0x80;
        let new_value = (value & 0xfe) >> 1;

        self.registers.F.zero = new_value == 0;
        self.registers.F.half_carry=false;
        self.registers.F.substract=false;
        self.registers.F.carry=did_overflow;

        new_value
       
      }

      fn SRL(&mut self, value:u8) -> u8
      {
        let did_overflow = value & 0x80 == 0x80;
        let new_value = (value & 0x7f) >> 1;

        self.registers.F.zero = new_value == 0;
        self.registers.F.half_carry=false;
        self.registers.F.substract=false;
        self.registers.F.carry=did_overflow;

        new_value
       
      }

      fn swap(&mut self, value:u8) -> u8
      {
       
       let new_value= value >> 4 | (value & 0x0F)<<4;

        self.registers.F.zero = new_value == 0;
        self.registers.F.half_carry=false;
        self.registers.F.substract=false;
        self.registers.F.carry=false;

        new_value
       
      }

}