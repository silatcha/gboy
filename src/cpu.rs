


mod register;
pub mod instruction;
mod mmu;



use self::instruction::targets::{
    ArithmeticTarget,
    ADDHLTarget,
    JumpTest,
    BitPosition,
    LoadByteSource,
    LoadByteTarget,
    LoadType,
    StackTarget, PrefixTarget, RSTPosition, IncDecTarget,
    LoadWordTarget, targetA
};

use self::register::registers::Registers;
use self::instruction::instructions::Instruction;
use self::mmu::memory_bus::MemoryBus;



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
    pub fn new(boot_rom: Option<[u8; 0xFFFF]>, game_rom: [u8; 0xFFFF]) -> CPU
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
    pub fn step(&mut self) 
    {
        // Read the instruction byte from memory using Program Counter register
        let mut instruction_byte = self.bus.read_byte(self.program_counter);

        // Check if the byte we read from memory is 0xCB, if it is, we read one
        // more byte and interpret the current as a "prefix instruction"
        let prefixed = instruction_byte == 0xCB;

        if prefixed
        {
            instruction_byte = self.bus.read_byte(self.program_counter + 1);
        }

        // Translate the byte to one of the instancse of the Instruction enum
        let next_program_counter = if let Some(instruction) = Instruction::from_byte(instruction_byte,prefixed)
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
    pub fn execute(&mut self, instruction: Instruction)-> u16
    {
        match instruction
        {
            // ADD
            Instruction::ADD(target) =>
            {
               match target
                {
                    ArithmeticTarget::C =>
                    {
                        let value = self.registers.C;
                        let new_value = self.add(value);
                        self.registers.A = new_value;
                       
                    },
                    ArithmeticTarget::B =>
                    {
                        let value = self.registers.B;
                        let new_value = self.add(value);
                        self.registers.A = new_value;
                       
                    },
                    ArithmeticTarget::A =>
                    {
                        let value = self.registers.A;
                        let new_value = self.add(value);
                        self.registers.A = new_value;
                       
                    },
                    ArithmeticTarget::D =>
                    {
                        let value = self.registers.D;
                        let new_value = self.add(value);
                        self.registers.A = new_value;
                       
                    },
                    ArithmeticTarget::E =>
                    {
                        let value = self.registers.E;
                        let new_value = self.add(value);
                        self.registers.A = new_value;
                       
                    },
                    ArithmeticTarget::H =>
                    {
                        let value = self.registers.H;
                        let new_value = self.add(value);
                        self.registers.A = new_value;
                        
                    },
                    ArithmeticTarget::L =>
                    {
                        let value = self.registers.L;
                        let new_value = self.add(value);
                        self.registers.A = new_value;
                       
                    }
                    ArithmeticTarget::HL =>
                    {
                        let value = self.bus.read_byte( self.registers.get_HL());
                        let new_value = self.add(value);
                        self.registers.A = new_value;
                        
                    },
                    ArithmeticTarget::D8 => {

                        let new_value = self.add(self.read_next_byte());
                        self.registers.A = new_value;
                    },

                }
                
                self.program_counter.wrapping_add(1)
                
            }



            Instruction::ADDHL(target) =>
            {
                match target
                {
                    ADDHLTarget::BC =>
                    {
                        let value = self.registers.get_BC();
                        let new_value = self.addhl(value);
                        self.registers.set_HL(new_value);
                        
                    },
                    ADDHLTarget::DE =>
                    {
                        let value = self.registers.get_DE();
                        let new_value = self.addhl(value);
                        self.registers.set_HL(new_value);
                       
                    },
                   
                    ADDHLTarget::HL =>
                    {
                        let value = self.registers.get_HL();
                        let new_value = self.addhl(value);
                        self.registers.set_HL(new_value);
                        
                    },
                    ADDHLTarget::SP =>
                    {
                        let value = self.stack_pointer;
                        let new_value = self.addhl(value);
                        self.registers.set_HL(new_value);
                        
                    },
                    
                  
                }
                
                self.program_counter.wrapping_add(1)
                
            }

            // RES
            Instruction::RES(target, bitPosition) =>
            {
                
                 match target
                {
                    
                    PrefixTarget::A => {
                        let value =self.registers.A;
                        self.registers.A=  self.res((bitPosition as BitPosition).into(),value);
                        
                    },
                    PrefixTarget::B => {
                        let value =self.registers.B;
                       self.registers.B=  self.res((bitPosition as BitPosition).into(),value);
                       
                    },
                    PrefixTarget::C => {
                        let value =self.registers.C;
                       self.registers.C=  self.res((bitPosition as BitPosition).into(),value);
                        },
                    PrefixTarget::D => {
                        let value =self.registers.D;
                        self.registers.D=  self.res((bitPosition as BitPosition).into(),value);
                       },
                    PrefixTarget::E => {
                        let value =self.registers.E;
                       self.registers.E=  self.res((bitPosition as BitPosition).into(),value);
                        },
                    PrefixTarget::H => {
                        let value =self.registers.H;
                        self.registers.H=  self.res((bitPosition as BitPosition).into(),value);
                       },
                    PrefixTarget::L => {
                        let value =self.registers.L;
                        self.registers.L=  self.res((bitPosition as BitPosition).into(),value);
                       },
                    PrefixTarget::HLI => {
                        let value =self.bus.read_byte(self.registers.get_HL());
                        let val =self.res((bitPosition as BitPosition).into(),value);
                        self.bus.write_byte( self.registers.get_HL(), val);
                        
                        },
                };
                
               
                
                self.program_counter.wrapping_add(2)
            }
             // RL
             Instruction::RL(target) =>
             {
                 
                 match target
                 {
                    PrefixTarget::A => {
                        let value =self.registers.A;
                        self.registers.A=  self.rl(value);
                        
                    },
                    PrefixTarget::B => {
                        let value =self.registers.B;
                        self.registers.B=  self.rl(value);
                        },
                    PrefixTarget::C => {
                        let value =self.registers.C;
                        self.registers.C=  self.rl(value);
                        },
                    PrefixTarget::D => {
                        let value =self.registers.D;
                        self.registers.D=  self.rl(value);
                        },
                    PrefixTarget::E => {
                        let value =self.registers.E;
                        self.registers.E=  self.rl(value);
                       },
                    PrefixTarget::H => {
                        let value =self.registers.H;
                        self.registers.H=  self.rl(value);
                       },
                    PrefixTarget::L => {
                        let value =self.registers.L;
                        self.registers.L=  self.rl(value);
                        },
                    PrefixTarget::HLI => {
                        
                        let value =self.bus.read_byte( self.registers.get_HL());
                        let val= self.rl(value);
                        self.bus.write_byte(self.registers.get_HL(), val);
                        
                        },
                 };
              
                 self.program_counter.wrapping_add(2)
               
             }

             // RLC
             Instruction::RLC(target) =>
             {
                 
                 let prefix = match target
                 {
                    PrefixTarget::A => {
                        let value =self.registers.A;
                        self.registers.A=  self.rlc(value);
                       
                    },
                    PrefixTarget::B => {
                        let value =self.registers.B;
                        self.registers.B=  self.rlc(value);
                        },
                    PrefixTarget::C => {
                        let value =self.registers.C;
                        self.registers.C=  self.rlc(value);
                       },
                    PrefixTarget::D => {
                        let value =self.registers.D;
                        self.registers.D=  self.rlc(value);
                        },
                    PrefixTarget::E => {
                        let value =self.registers.E;
                        self.registers.E=  self.rlc(value);
                        },
                    PrefixTarget::H => {
                        let value =self.registers.H;
                        self.registers.H=  self.rlc(value);
                        },
                    PrefixTarget::L => {
                        let value =self.registers.L;
                        self.registers.L=  self.rlc(value);
                       },
                    PrefixTarget::HLI => {
                        let value =self.bus.read_byte(self.registers.get_HL());
                        let val=self.rlc(value);
                        self.bus.write_byte(self.registers.get_HL(), val);
                        },
                 };
                
                 self.program_counter.wrapping_add(2)
             }

             // RR
             Instruction::RR(target) =>
             {
                 
                match target
                 {
                    PrefixTarget::A => {
                        let value =self.registers.A;
                        self.registers.A=  self.rr(value);
                        
                    },
                    PrefixTarget::B => {
                        let value =self.registers.B;
                        self.registers.B=  self.rr(value);
                        },
                    PrefixTarget::C => {
                        let value =self.registers.C;
                        self.registers.C=  self.rr(value);
                        },
                    PrefixTarget::D => {
                        let value =self.registers.D;
                        self.registers.D=  self.rr(value);
                        },
                    PrefixTarget::E => {
                        let value =self.registers.E;
                        self.registers.E=  self.rr(value);
                        },
                    PrefixTarget::H => {
                        let value =self.registers.H;
                        self.registers.H=  self.rr(value);
                        },
                    PrefixTarget::L => {
                        let value =self.registers.L;
                        self.registers.L=  self.rr(value);
                        },
                    PrefixTarget::HLI => {
                        let value =self.bus.read_byte(self.registers.get_HL());
                        let val=self.rr(value);
                        self.bus.write_byte(self.registers.get_HL(), val);
                    },
                 };
                
                 self.program_counter.wrapping_add(2)
             }


             // RRc
             Instruction::RRC(target) =>
             {
                 
                match target
                 {
                    PrefixTarget::A => {
                        let value =self.registers.A;
                        self.registers.A=  self.rrc(value);
                        
                    },
                    PrefixTarget::B => {
                        let value =self.registers.B;
                        self.registers.B=  self.rrc(value);
                       },
                    PrefixTarget::C => {
                        let value =self.registers.C;
                        self.registers.C=  self.rrc(value);
                        },
                    PrefixTarget::D => {
                        let value =self.registers.D;
                        self.registers.D=  self.rrc(value);
                        },
                    PrefixTarget::E => {
                        let value =self.registers.E;
                        self.registers.E=  self.rrc(value);
                        },
                    PrefixTarget::H => {
                        let value =self.registers.H;
                        self.registers.H=  self.rrc(value);
                        },
                    PrefixTarget::L => {
                        let value =self.registers.L;
                        self.registers.L=  self.rrc(value);
                       },
                    PrefixTarget::HLI => {
                        let value =self.bus.read_byte(self.registers.get_HL());
                        let val=self.rrc(value);
                        self.bus.write_byte(self.registers.get_HL(), val);
                        },
                 };
                
                 self.program_counter.wrapping_add(2)
             }


              // RST
              Instruction::RST(target) =>
              {
                  
                self.rst((target as RSTPosition).into());
                self.program_counter.wrapping_add(1)
                  
              }


              // SCF
              Instruction::SCF =>
              {
                  
                self.scf();
                self.program_counter.wrapping_add(1)
                 
              }

              // SET
              Instruction::SET(target,bitPosition) =>
              {
                  
                  
               match target
                {
                    PrefixTarget::A => {
                        let value =self.registers.A;
                        self.registers.A=  self.set((bitPosition as BitPosition).into(),value);
                       
                    },
                    PrefixTarget::B => {
                        let value =self.registers.B;
                        self.registers.B=  self.set((bitPosition as BitPosition).into(),value);
                       },
                    PrefixTarget::C => {
                        let value =self.registers.C;
                        self.registers.C=  self.set((bitPosition as BitPosition).into(),value);
                        },
                    PrefixTarget::D => {
                        let value =self.registers.D;
                        self.registers.D=  self.set((bitPosition as BitPosition).into(),value);
                        },
                    PrefixTarget::E => {
                        let value =self.registers.E;
                        self.registers.E=  self.set((bitPosition as BitPosition).into(),value);
                        },
                    PrefixTarget::H => {
                        let value =self.registers.H;
                        self.registers.H=  self.set((bitPosition as BitPosition).into(),value);
                        },
                    PrefixTarget::L => {
                        let value =self.registers.L;
                        self.registers.L=  self.set((bitPosition as BitPosition).into(),value);
                        },
                    PrefixTarget::HLI => {
                        let value =self.bus.read_byte(self.registers.get_HL());
                        let val=self.set((bitPosition as BitPosition).into(),value);
                        self.bus.write_byte(self.registers.get_HL(), val);
                        },
                };
               
                self.program_counter.wrapping_add(2)
              }

              // SLA
              Instruction::SLA(target) =>
              {
                  
                  
                match target
                {
                    PrefixTarget::A => {
                        let value =self.registers.A;
                        self.registers.A=  self.sla(value);
                       
                    },
                    PrefixTarget::B => {
                        let value =self.registers.B;
                        self.registers.B=  self.sla(value);
                        },
                    PrefixTarget::C => {
                        let value =self.registers.C;
                        self.registers.C=  self.sla(value);
                      },
                    PrefixTarget::D => {
                        let value =self.registers.D;
                        self.registers.D=  self.sla(value);
                       },
                    PrefixTarget::E => {
                        let value =self.registers.E;
                        self.registers.E=  self.sla(value);
                       },
                    PrefixTarget::H => {
                        let value =self.registers.H;
                        self.registers.H=  self.sla(value);
                       },
                    PrefixTarget::L => {
                        let value =self.registers.L;
                        self.registers.L=  self.sla(value);
                       },
                    PrefixTarget::HLI => {
                        let value =self.bus.read_byte(self.registers.get_HL());
                        let val=self.sla(value);
                        self.bus.write_byte(self.registers.get_HL(), val);
                       },
                };
               
                self.program_counter.wrapping_add(2)
              }

              // SRA
              Instruction::SRA(target) =>
              {
                  
                  
                match target
                {
                    PrefixTarget::A => {
                        let value =self.registers.A;
                        self.registers.A=  self.sra(value);
                        
                    },
                    PrefixTarget::B => {
                        let value =self.registers.B;
                        self.registers.B=  self.sra(value);
                       },
                    PrefixTarget::C => {
                        let value =self.registers.C;
                        self.registers.C=  self.sra(value);
                       },
                    PrefixTarget::D => {
                        let value =self.registers.D;
                        self.registers.D=  self.sra(value);
                        },
                    PrefixTarget::E => {
                        let value =self.registers.E;
                        self.registers.E=  self.sra(value);
                       },
                    PrefixTarget::H => {
                        let value =self.registers.H;
                        self.registers.H=  self.sra(value);
                        },
                    PrefixTarget::L => {
                        let value =self.registers.L;
                        self.registers.L=  self.sra(value);
                       },
                    PrefixTarget::HLI => {
                        let value =self.bus.read_byte(self.registers.get_HL());
                        let val=self.sra(value);
                        self.bus.write_byte(self.registers.get_HL(), val);
                       },
                };
               
                self.program_counter.wrapping_add(2)
              }

              // SRL
              Instruction::SRL(target) =>
              {
                  
                  
                match target
                {
                    PrefixTarget::A => {
                        let value =self.registers.A;
                        self.registers.A=  self.srl(value);
                        
                    },
                    PrefixTarget::B => {
                        let value =self.registers.B;
                        self.registers.B=  self.srl(value);
                      },
                    PrefixTarget::C => {
                        let value =self.registers.C;
                        self.registers.C=  self.srl(value);
                       },
                    PrefixTarget::D => {
                        let value =self.registers.D;
                        self.registers.D=  self.srl(value);
                        },
                    PrefixTarget::E => {
                        let value =self.registers.E;
                        self.registers.E=  self.srl(value);
                        },
                    PrefixTarget::H => {
                        let value =self.registers.H;
                        self.registers.H=  self.srl(value);
                       },
                    PrefixTarget::L => {
                        let value =self.registers.L;
                        self.registers.L=  self.srl(value);
                        },
                    PrefixTarget::HLI => {
                        let value =self.bus.read_byte(self.registers.get_HL());
                        let val=self.srl(value);
                        self.bus.write_byte(self.registers.get_HL(), val);
                        },
                };
               
                self.program_counter.wrapping_add(2)
              }

              // SRL
              Instruction::SWAP(target) =>
              {
                  
                  
                let prefix = match target
                {
                    PrefixTarget::A => {
                        let value =self.registers.A;
                        self.registers.A=  self.swap(value);
                        
                    },
                    PrefixTarget::B => {
                        let value =self.registers.B;
                        self.registers.B=  self.swap(value);
                        },
                    PrefixTarget::C => {
                        let value =self.registers.C;
                        self.registers.C=  self.swap(value);
                        },
                    PrefixTarget::D => {
                        let value =self.registers.D;
                        self.registers.D=  self.swap(value);
                     },
                    PrefixTarget::E => {
                        let value =self.registers.E;
                        self.registers.E=  self.swap(value);
                       },
                    PrefixTarget::H => {
                        let value =self.registers.H;
                        self.registers.H=  self.swap(value);
                        },
                    PrefixTarget::L => {
                        let value =self.registers.L;
                        self.registers.L=  self.swap(value);
                        },
                    PrefixTarget::HLI => {
                        let value =self.bus.read_byte(self.registers.get_HL());
                        let val=self.swap(value);
                        self.bus.write_byte(self.registers.get_HL(), val);
                        },
                };
               
                self.program_counter.wrapping_add(2)
              }


              // EI
              Instruction::EI =>
              {
                self.ei();
                self.program_counter.wrapping_add(1)
               
              }

              // EI
              Instruction::DI =>
              {
                self.di();
                self.program_counter.wrapping_add(1)
                
              }

            

            Instruction::ADDSP =>
            {
               
                
                    self.addsp();
                    self.program_counter.wrapping_add(1)
                
            }
            
            Instruction::CCF =>
            {
               
                    self.ccf();
                    self.program_counter.wrapping_add(1)
                
            }
            Instruction::CPL =>
            {
                        self.cpl();
                        self.program_counter.wrapping_add(1)
                 
                
            }
            Instruction::DAA =>
            {
                        self.daa();
                        self.program_counter.wrapping_add(4)
               
                
            }
            Instruction::ADC(target) =>
            {
               match target
                {
                    ArithmeticTarget::C =>
                    {
                        let value = self.registers.C;
                        let new_value = self.adc(value);
                        self.registers.A = new_value;
                       
                    },
                    ArithmeticTarget::B =>
                    {
                        let value = self.registers.B;
                        let new_value = self.adc(value);
                        self.registers.A = new_value;
                       
                    },
                    ArithmeticTarget::A =>
                    {
                        let value = self.registers.A;
                        let new_value = self.adc(value);
                        self.registers.A = new_value;
                       
                    },
                    ArithmeticTarget::D =>
                    {
                        let value = self.registers.D;
                        let new_value = self.adc(value);
                        self.registers.A = new_value;
                        
                    },
                    ArithmeticTarget::E =>
                    {
                        let value = self.registers.E;
                        let new_value = self.adc(value);
                        self.registers.A = new_value;
                        
                    },
                    ArithmeticTarget::H =>
                    {
                        let value = self.registers.H;
                        let new_value = self.adc(value);
                        self.registers.A = new_value;
                        
                    },
                    ArithmeticTarget::L =>
                    {
                        let value = self.registers.L;
                        let new_value = self.adc(value);
                        self.registers.A = new_value;
                        
                    },
                    ArithmeticTarget::HL =>
                    {
                        let value = self.bus.read_byte(self.registers.get_HL());
                        let new_value = self.adc(value);
                        self.registers.A = new_value;
                        
                    }
                    ArithmeticTarget::D8 => {
                        
                        let new_value = self.adc(self.read_next_byte());
                        self.registers.A = new_value;
                    },

                };
                
                self.program_counter.wrapping_add(4)
                
            }
            // SUB
            Instruction::SUB(target) =>
            {
              match target
                {
                    ArithmeticTarget::C =>
                    {
                        let value = self.registers.C;
                        let new_value = self.sub(value);
                        self.registers.A = new_value;
                       
                    },
                    ArithmeticTarget::B =>
                    {
                        let value = self.registers.B;
                        let new_value = self.sub(value);
                        self.registers.A = new_value;
                       
                    },
                    ArithmeticTarget::A =>
                    {
                        let value = self.registers.A;
                        let new_value = self.sub(value);
                        self.registers.A = new_value;
                        
                    },
                    ArithmeticTarget::D =>
                    {
                        let value = self.registers.D;
                        let new_value = self.sub(value);
                        self.registers.A = new_value;
                       
                    },
                    ArithmeticTarget::E =>
                    {
                        let value = self.registers.E;
                        let new_value = self.sub(value);
                        self.registers.A = new_value;
                        
                    },
                    ArithmeticTarget::H =>
                    {
                        let value = self.registers.H;
                        let new_value = self.sub(value);
                        self.registers.A = new_value;
                        
                    },
                    ArithmeticTarget::L =>
                    {
                        let value = self.registers.L;
                        let new_value = self.sub(value);
                        self.registers.A = new_value;
                        
                    },
                    ArithmeticTarget::HL =>
                    {
                        let value = self.bus.read_byte( self.registers.get_HL());
                        let new_value = self.sub(value);
                        self.registers.A = new_value;
                        
                    }
                    ArithmeticTarget::D8 => {
                        let new_value = self.sub(self.read_next_byte());
                        self.registers.A = new_value;
                    },

                };
                
                self.program_counter.wrapping_add(4)
                
            }   
            // SBC
            Instruction::SBC(target) =>
            {
               match target
                {
                    ArithmeticTarget::C =>
                    {
                        let value = self.registers.C;
                        let new_value = self.sbc(value);
                        self.registers.A = new_value;
                        
                    },
                    ArithmeticTarget::B =>
                    {
                        let value = self.registers.B;
                        let new_value = self.sbc(value);
                        self.registers.A = new_value;
                        
                    },
                    ArithmeticTarget::A =>
                    {
                        let value = self.registers.A;
                        let new_value = self.sbc(value);
                        self.registers.A = new_value;
                        
                    },
                    ArithmeticTarget::D =>
                    {
                        let value = self.registers.D;
                        let new_value = self.sbc(value);
                        self.registers.A = new_value;
                        
                    },
                    ArithmeticTarget::E =>
                    {
                        let value = self.registers.E;
                        let new_value = self.sbc(value);
                        self.registers.A = new_value;
                       
                    },
                    ArithmeticTarget::H =>
                    {
                        let value = self.registers.H;
                        let new_value = self.sbc(value);
                        self.registers.A = new_value;
                        
                    },
                    ArithmeticTarget::L =>
                    {
                        let value = self.registers.L;
                        let new_value = self.sbc(value);
                        self.registers.A = new_value;
                        
                    },
                    ArithmeticTarget::HL =>
                    {
                        let value = self.bus.read_byte(self.registers.get_HL());
                        let new_value = self.sbc(value);
                        self.registers.A = new_value;
                       
                    }
                    ArithmeticTarget::D8 => {
                        let new_value = self.sbc(self.read_next_byte());
                        self.registers.A = new_value;
                    },
                };
                
                self.program_counter.wrapping_add(1)
                
            } 
            // AND
            Instruction::AND(target) =>
            {
              match target
                {
                    ArithmeticTarget::C =>
                    {
                        let value = self.registers.C;
                        let new_value = self.and(value);
                        self.registers.A = new_value;
                       
                    },
                    ArithmeticTarget::B =>
                    {
                        let value = self.registers.B;
                        let new_value = self.and(value);
                        self.registers.A = new_value;
                      
                    },
                    ArithmeticTarget::A =>
                    {
                        let value = self.registers.A;
                        let new_value = self.and(value);
                        self.registers.A = new_value;
                       
                    },
                    ArithmeticTarget::D =>
                    {
                        let value = self.registers.D;
                        let new_value = self.and(value);
                        self.registers.A = new_value;
                      
                    },
                    ArithmeticTarget::E =>
                    {
                        let value = self.registers.E;
                        let new_value = self.and(value);
                        self.registers.A = new_value;
                     
                    },
                    ArithmeticTarget::H =>
                    {
                        let value = self.registers.H;
                        let new_value = self.and(value);
                        self.registers.A = new_value;
                       
                    },
                    ArithmeticTarget::L =>
                    {
                        let value = self.registers.L;
                        let new_value = self.and(value);
                        self.registers.A = new_value;
                       
                    }
                    ArithmeticTarget::HL =>
                    {
                        let value = self.bus.read_byte(self.registers.get_HL());
                        let new_value = self.and(value);
                        self.registers.A = new_value;
                      
                    }
                    ArithmeticTarget::D8 => {
                        let new_value = self.and(self.read_next_byte());
                        self.registers.A = new_value;
                    },

                };
                
                self.program_counter.wrapping_add(1)
                
            }
            // OR
            Instruction::OR(target) =>
            {
              match target
                {
                    ArithmeticTarget::C =>
                    {
                        let value = self.registers.C;
                        let new_value = self.or(value);
                        self.registers.A = new_value;
                       
                    },
                    ArithmeticTarget::B =>
                    {
                        let value = self.registers.B;
                        let new_value = self.or(value);
                        self.registers.A = new_value;
                       
                    },
                    ArithmeticTarget::A =>
                    {
                        let value = self.registers.A;
                        let new_value = self.or(value);
                        self.registers.A = new_value;
                       
                    },
                    ArithmeticTarget::D =>
                    {
                        let value = self.registers.D;
                        let new_value = self.or(value);
                        self.registers.A = new_value;
                        
                    },
                    ArithmeticTarget::E =>
                    {
                        let value = self.registers.E;
                        let new_value = self.or(value);
                        self.registers.A = new_value;
                        
                    },
                    ArithmeticTarget::H =>
                    {
                        let value = self.registers.H;
                        let new_value = self.or(value);
                        self.registers.A = new_value;
                       
                    },
                    ArithmeticTarget::L =>
                    {
                        let value = self.registers.L;
                        let new_value = self.or(value);
                        self.registers.A = new_value;
                       
                    }
                    ArithmeticTarget::HL =>
                    {
                        let value = self.bus.read_byte(self.registers.get_HL());
                        let new_value = self.or(value);
                        self.registers.A = new_value;
                       
                    }
                    ArithmeticTarget::D8 => {
                        let new_value = self.or(self.read_next_byte());
                        self.registers.A = new_value;
                    },

                };
                
                self.program_counter.wrapping_add(4)
                
            }
            // XOR
            Instruction::XOR(target) =>
            {
                match target
                {
                    ArithmeticTarget::C =>
                    {
                        let value = self.registers.C;
                        let new_value = self.xor(value);
                        self.registers.A = new_value;
                        
                    },
                    ArithmeticTarget::B =>
                    {
                        let value = self.registers.B;
                        let new_value = self.xor(value);
                        self.registers.A = new_value;
                        
                    },
                    ArithmeticTarget::A =>
                    {
                        let value = self.registers.A;
                        let new_value = self.xor(value);
                        self.registers.A = new_value;
                       
                    },
                    ArithmeticTarget::D =>
                    {
                        let value = self.registers.D;
                        let new_value = self.xor(value);
                        self.registers.A = new_value;
                       
                    },
                    ArithmeticTarget::E =>
                    {
                        let value = self.registers.E;
                        let new_value = self.xor(value);
                        self.registers.A = new_value;
                       
                    },
                    ArithmeticTarget::H =>
                    {
                        let value = self.registers.H;
                        let new_value = self.xor(value);
                        self.registers.A = new_value;
                       
                    },
                    ArithmeticTarget::L =>
                    {
                        let value = self.registers.L;
                        let new_value = self.xor(value);
                        self.registers.A = new_value;
                      
                    }
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_HL());
                        let new_value = self.xor(value);
                        self.registers.A = new_value;
                    }
                    ArithmeticTarget::D8 => {
                        let new_value = self.xor(self.read_next_byte());
                        self.registers.A = new_value;
                    },

                };
                
                self.program_counter.wrapping_add(4)
                
            }
            // CP 
            Instruction::CP(target) =>
            {
               match target
                {
                    ArithmeticTarget::C =>
                    {
                       
                         self.cp(self.registers.C);
                        
                        
                    },
                    ArithmeticTarget::B =>
                    {
                        self.cp(self.registers.B);
                        
                        
                    },
                    ArithmeticTarget::A =>
                    {
                        self.cp(self.registers.A);
                    },
                    ArithmeticTarget::D =>
                    {
                        self.cp(self.registers.D);
                    },
                    ArithmeticTarget::E =>
                    {
                        self.cp(self.registers.E);
                    },
                    ArithmeticTarget::H =>
                    {
                        self.cp(self.registers.H);
                    },
                    ArithmeticTarget::L =>
                    {
                        self.cp(self.registers.L);
                    }
                    ArithmeticTarget::HL =>
                    {
                        self.cp(self.bus.read_byte(self.registers.get_HL()));
                    },
                    ArithmeticTarget::D8 => {
                        self.read_next_byte();
                        
                    },

                };
                
                self.program_counter.wrapping_add(4)
                
            }
            // INCREMENT 
            Instruction::INC(target) =>
            {
               match target
                {
                    IncDecTarget::C =>
                    {
                        self.registers.C.wrapping_add(1);
                      
                        
                    },
                    IncDecTarget::B =>
                    {
                        self.registers.B.wrapping_add(1);
                        
                       
                    },
                    IncDecTarget::A =>
                    {
                     self.registers.A.wrapping_add(1);
                        
                       
                    },
                    IncDecTarget::D =>
                    {
                      self.registers.D.wrapping_add(1);
                       
                       
                    },
                    IncDecTarget::E =>
                    {
                       self.registers.E.wrapping_add(1);
                      
                       
                    },
                    IncDecTarget::H =>
                    {
                        self.registers.H.wrapping_add(1);
                        
                       
                    },
                    IncDecTarget::L =>
                    {
                        self.registers.L.wrapping_add(1);
                       
                       
                    },
                    IncDecTarget::HLI =>
                    {
                         self.bus.write_byte(self.registers.get_HL(),self.bus.read_byte(self.registers.get_HL().wrapping_add(1)));
                       
                       
                    },
                    IncDecTarget::BC =>
                    {
                        self.registers.set_BC(self.registers.get_BC().wrapping_add(1));
                        
                       
                    },
                    IncDecTarget::DE =>
                    {
                        self.registers.set_DE(self.registers.get_DE().wrapping_add(1));
                        
                    },
                    
                    IncDecTarget::HL =>
                    {
                        self.registers.set_HL(self.registers.get_HL().wrapping_add(1));
                      
                    },
                    IncDecTarget::SP =>
                    {
                        self.stack_pointer.wrapping_add(1);
                        
                        
                    }

                };
                
                self.program_counter.wrapping_add(1)
                
            }
            // DECREMENT 
            Instruction::DEC(target) =>
            {
                match target
                {
                    IncDecTarget::C =>
                    {
                        let value = self.registers.C;
                        let new_value = self.dec(value);
                        self.registers.C = new_value;
                       
                    },
                    IncDecTarget::B =>
                    {
                        let value = self.registers.B;
                        let new_value = self.dec(value);
                        self.registers.B = new_value;
                       
                    },
                    IncDecTarget::A =>
                    {
                        let value = self.registers.A;
                        let new_value = self.dec(value);
                        self.registers.A = new_value;
                        
                    },
                    IncDecTarget::D =>
                    {
                        let value = self.registers.D;
                        let new_value = self.dec(value);
                        self.registers.D = new_value;
                        
                    },
                    IncDecTarget::E =>
                    {
                        let value = self.registers.E;
                        let new_value = self.dec(value);
                        self.registers.E = new_value;
                        
                    },
                    IncDecTarget::H =>
                    {
                        let value = self.registers.H;
                        let new_value = self.dec(value);
                        self.registers.H = new_value;
                       
                    },
                    IncDecTarget::L =>
                    {
                        let value = self.registers.L;
                        let new_value = self.dec(value);
                        self.registers.L = new_value;
                       
                    },
                    IncDecTarget::HLI =>
                    {
                        self.bus.write_byte(self.registers.get_HL(),self.bus.read_byte(self.registers.get_HL().wrapping_add(1)));
                        let value = self.bus.read_byte(self.registers.get_HL());
                        self.dec(value);
                        self.bus.write_byte(self.registers.get_HL(),value);
                        
                    },
                    IncDecTarget::BC =>
                    {
                        let value = self.registers.get_BC();
                        let new_value = self.dechl(value);
                        self.registers.set_BC( new_value);
                        
                    },
                    IncDecTarget::DE =>
                    {
                        let value = self.registers.get_DE();
                        let new_value = self.dechl(value);
                        self.registers.set_DE(new_value);
                       
                    },
                   
                    IncDecTarget::HL =>
                    {
                        let value = self.registers.get_HL();
                        let new_value = self.dechl(value);
                        self.registers.set_HL(new_value);
                        
                    },
                    IncDecTarget::SP =>
                    {
                        let value = self.stack_pointer;
                        let new_value = self.dechl(value);
                        self.stack_pointer = new_value;
                        
                    }

                };
                
                self.program_counter.wrapping_add(1)
                
            }
            // BIT OPERATION 
            Instruction::BIT(target1, target2) =>
            {
                match target1
                {
                    PrefixTarget::C =>
                    {                       
                        self.bit(self.registers.C, (target2 as BitPosition).into());                        
                       
                    },
                    PrefixTarget::B =>
                    {
                        self.bit(self.registers.B, (target2 as BitPosition).into());                        
                     
                    },
                    PrefixTarget::A =>
                    {
                        self.bit(self.registers.A, (target2 as BitPosition).into());                        
                       
                    },
                    PrefixTarget::D=>
                    {
                       self.bit(self.registers.D, (target2 as BitPosition).into());                        
                        
                    },
                    PrefixTarget::E=>
                    {
                       self.bit(self.registers.E, (target2 as BitPosition).into());                        
                        
                    },
                    PrefixTarget::H=>
                    {
                       self.bit(self.registers.H, (target2 as BitPosition).into());                        
                        
                    },
                    PrefixTarget::L =>
                    {
                        self.bit(self.registers.L, (target2 as BitPosition).into());                        
                        
                    },
                    PrefixTarget::HLI =>
                    {
                       self.bit(self.bus.read_byte(self.registers.get_HL()), (target2 as BitPosition).into());                        
                        
                    },
                    

                };
                
                self.program_counter.wrapping_add(2)
                
            }
            // JP
            Instruction::JP(target) =>
            {
                let jump_condition = match target
                {
                  
                        JumpTest::NZ => !self.registers.F.zero,
                        JumpTest::Z => self.registers.F.zero,
                        JumpTest::C => self.registers.F.carry,
                        JumpTest::A => true,
                        JumpTest::NC => !self.registers.F.carry,
                };
                    self.jump(jump_condition)
            }

            // JPHL
            Instruction::JPHL =>
            {
               
                    self.jp_nz_nn();
                    self.program_counter.wrapping_add(1)
            }
            // JR
            Instruction::JR(target) =>
            {
                let jump_condition = match target
                {
                  
                        JumpTest::NZ => !self.registers.F.zero,
                        JumpTest::Z => self.registers.F.zero,
                        JumpTest::C => self.registers.F.carry,
                        JumpTest::A => true,
                        JumpTest::NC => !self.registers.F.carry,
                };

                self.jr_nc_sn(jump_condition)
            }


            // LD
            Instruction::LD(load_type) =>
            {
                match load_type   {
                    LoadType::Byte(target, source) =>
                    {
                        let source_value = match source
                        {
                            LoadByteSource::A => self.registers.A,
                            LoadByteSource::B => self.registers.B,
                            LoadByteSource::C => self.registers.C,
                            LoadByteSource::D => self.registers.D,
                            LoadByteSource::E => self.registers.E,
                            LoadByteSource::H => self.registers.H,
                            LoadByteSource::L => self.registers.L,
                            LoadByteSource::BC => self.bus.read_byte(self.registers.get_BC()),
                            LoadByteSource::DE => self.bus.read_byte(self.registers.get_DE()),
                            LoadByteSource::HL => self.bus.read_byte(self.registers.get_HL()),
                            LoadByteSource::D8 => self.read_next_byte(),
                            LoadByteSource::HLD => self.bus.read_byte(self.registers.get_HL()),
                            LoadByteSource::HLI => self.bus.read_byte(self.registers.get_HL()),
                            LoadByteSource::CC => self.bus.read_byte(self.registers.C as u16),                       
                        };
                        match target
                        {
                            LoadByteTarget::A => self.registers.A = source_value,
                            LoadByteTarget::B => self.registers.B = source_value,
                            LoadByteTarget::C => self.registers.C = source_value,
                            LoadByteTarget::D => self.registers.D = source_value,
                            LoadByteTarget::E => self.registers.E = source_value,
                            LoadByteTarget::H => self.registers.H = source_value,
                            LoadByteTarget::L => self.registers.L = source_value,
                            LoadByteTarget::HLI => self.bus.write_byte(self.registers.get_HL(), source_value),
                            LoadByteTarget::BC => self.bus.write_byte(self.registers.get_BC(), source_value),
                            LoadByteTarget::DE => self.bus.write_byte(self.registers.get_DE(), source_value),
                            LoadByteTarget::HL => self.bus.write_byte(self.registers.get_HL(), source_value),
                            LoadByteTarget::HLD => self.bus.write_byte(self.registers.get_HL(), source_value),
                            LoadByteTarget::CC => self.bus.write_byte(self.registers.C as u16, source_value),           
                            
                        };
                        match source
                        {
                            LoadByteSource::D8  => self.program_counter.wrapping_add(2),
                            _                   => self.program_counter.wrapping_add(1),
                            LoadByteSource::A => self.program_counter.wrapping_add(1),
                            LoadByteSource::B => self.program_counter.wrapping_add(1),
                            LoadByteSource::C => self.program_counter.wrapping_add(1),
                            LoadByteSource::D => self.program_counter.wrapping_add(1),
                            LoadByteSource::E => self.program_counter.wrapping_add(1),
                            LoadByteSource::H => self.program_counter.wrapping_add(1),
                            LoadByteSource::L => self.program_counter.wrapping_add(1),
                            LoadByteSource::BC => self.program_counter.wrapping_add(1),
                            LoadByteSource::DE => self.program_counter.wrapping_add(1),
                            LoadByteSource::HL => self.program_counter.wrapping_add(1),
                            
                            LoadByteSource::HLD => self.program_counter.wrapping_add(1),
                            LoadByteSource::HLI => self.program_counter.wrapping_add(1),
                            LoadByteSource::CC => {
                                self.program_counter.wrapping_add(2)
                            }  
                        }
                        
                    }
                    LoadType::Word(target) => {
                        match target
                        {
                            LoadWordTarget::BC => self.bus.write_byte(self.registers.C as u16,self.bus.read_byte(self.registers.get_BC())),  
                            LoadWordTarget::DE => self.bus.write_byte(self.read_next_word(), self.bus.read_byte(self.registers.get_DE())) ,
                            LoadWordTarget::HL => self.bus.write_byte(self.read_next_word(), self.bus.read_byte(self.registers.get_HL())) ,
                           LoadWordTarget::SP => self.bus.write_byte(self.read_next_word(), self.bus.read_byte(self.stack_pointer)) ,
                        }
                        self.program_counter.wrapping_add(3)
                    },
                    LoadType::AFromIndirect(_) => todo!(),
                    LoadType::IndirectFromA(_) => todo!(),
                    LoadType::AFromByteAddress => {
                        self.bus.write_byte(self.registers.A as u16, self.registers.A);
                        self.program_counter.wrapping_add(3)
                    },
                    LoadType::ByteAddressFromA => {
                        self.registers.A= self.bus.read_byte(self.registers.A as u16);
                        self.program_counter.wrapping_add(3)
                    },
                    LoadType::SPFromHL => todo!(),
                    LoadType::HLFromSPN => todo!(),
                    LoadType::IndirectFromSP => todo!(),
                }
               
            }
            // CALL
            Instruction::CALL(test) => {
                let jump_condition = match test {
                    JumpTest::NZ => !self.registers.F.zero,
                    _ => { panic!("TODO: support more conditions") }
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

            Instruction::RETI =>
            {
                self.reti()
            }
            // PUSH
            Instruction::PUSH(target) =>
            {
                let value = match target
                {
                    StackTarget::BC => {
                        self.registers.get_BC()
                      
                    },
                    StackTarget::DE => {
                        self.registers.get_DE()
                       
                    },
                    StackTarget::HL => {
                        self.registers.get_HL()
                       
                    },
                    StackTarget::AF => {
                        self.registers.get_AF()
                       
                    },
                    //self.program_counter.wrapping_add(1);
                    
                };
                self.push(value);
                self.program_counter.wrapping_add(1)
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
            },

            // DI
            Instruction::DI =>
            {
                self.di();
                self.program_counter.wrapping_add(1)
            },
            // EI
            Instruction::DI =>
            {
                self.ei();
                self.program_counter.wrapping_add(1)
            },

            // halte
            Instruction::HALT =>
            {
                self.halte();
                self.program_counter.wrapping_add(1)
            },

            Instruction::STOP =>
            {
                
                self.program_counter.wrapping_add(2)
            },

            Instruction::NOP =>
            {
                
                self.program_counter.wrapping_add(1)
            },

            Instruction::RRA =>
            {
             self.rra();   
                self.program_counter.wrapping_add(1)
            },

            Instruction::RRCA =>
            {
             self.rrca();   
                self.program_counter.wrapping_add(1)
            },

            Instruction::RLA =>
            {
             self.rla();   
                self.program_counter.wrapping_add(1)
            },

            Instruction::RLCA =>
            {
             self.rlca();   
                self.program_counter.wrapping_add(1)
            },
            
            Instruction::LDC(_) => todo!(),
            Instruction::LDHLI(_) => todo!(),
            Instruction::LDHLD(_) => todo!(),
            Instruction::LDR(_) => todo!(),
            Instruction::LDH(targetA) => {
                match targetA{
                    targetA::A => {
                        self.registers.A=self.bus.read_byte(self.registers.A as u16);
                    },
                }
                self.program_counter.wrapping_add(2)
            },
            Instruction::LDHS(targetA) => {
                match targetA{
                    instruction::targets::sourceA::A => {
                        self.bus.write_byte(self.registers.A as u16, self.registers.A);
                    },
                }
                self.program_counter.wrapping_add(2)
            },
            
           

            
           
        }
    }
    
    
   


    fn rlca(&mut self)  {
        let bit7 = (self.registers.A & 0x01) != 0; // Capture le 7e bit du registre A (drapeau de retenue)
        self.registers.A <<= 1; // Effectue la rotation vers la gauche de 1 bit
    if self.registers.F.carry {
        self.registers.A |= 0x0001; // Met la valeur du drapeau de retenue dans le bit 0 du registre A
    }
    self.registers.F.carry = bit7; // Met à jour le drapeau de retenue avec l'ancienne valeur du 7e bit
    }

        fn rla(&mut self)  {
            let bit7 = (self.registers.A & 0x01) != 0; // Capture le 7e bit du registre A (drapeau de retenue)
            self.registers.A <<= 1; // Effectue la rotation vers la gauche de 1 bit
        if self.registers.F.carry {
            self.registers.A |= 0x0001; // Met la valeur du drapeau de retenue dans le bit 0 du registre A
        }
        self.registers.F.carry = bit7; // Met à jour le drapeau de retenue avec l'ancienne valeur du 7e bit
        }

fn rrca(&mut self)  {
    let bit0 = self.registers.A & 0x01; // Capture le bit 0 du registre A
    self.registers.A >>= 1; // Effectue la rotation vers la droite de 1 bit
    self.registers.A |= if self.registers.F.carry { 0x80 } else { 0 }; // Copie la valeur du drapeau de retenue dans le bit 7 du registre A
    self.registers.F.carry = bit0 != 0; // Met à jour le drapeau de retenue avec l'ancienne valeur du bit 0
}

    fn rra(&mut self){
        let bit0 = self.registers.A & 0x01; // Capture le bit 0 du registre A
        self.registers.A >>= 1; // Effectue la rotation vers la droite de 1 bit
        if self.registers.F.carry {
            self.registers.A |= 0x80; // Met la valeur du drapeau de retenue dans le bit 7 du registre A
        }
        self.registers.F.carry = bit0 != 0; 
    }
   
   
    //ADC
    fn adc(&mut self,value:u8) -> u8
    {
       
        let mut n_adjusted = value;
    if self.registers.F.carry {
        n_adjusted = n_adjusted.wrapping_add(1);
    }

    // Perform the subtraction.
    let mut result = self.registers.A.wrapping_sub(n_adjusted);

   
    self.registers.F.substract= true;
    self.registers.F.half_carry = (self.registers.A & 0x0F) < (n_adjusted & 0x0F);
    self.registers.F.carry= self.registers.A < n_adjusted;
   

    // If there was a carry in the subtraction, set the carry flag.
    if self.registers.F.carry {
        self.registers.F.carry = self.registers.F.carry || self.registers.A .wrapping_sub(1) < n_adjusted;
    }

    result = result.wrapping_sub(if self.registers.F.carry{ 1 } else { 0 });
    self.registers.F.zero= result == 0;

    result

    }
   
    // Substract instruction
    fn sub(&mut self, value: u8) -> u8
    {
        let (new_value, did_overflow) = self.registers.A.overflowing_sub(value);

        // Set the flags
        self.registers.F.zero = new_value == 0;
        self.registers.F.substract = true;
        self.registers.F.carry = did_overflow;

        // Half Carry is set if adding the lower nibbles of the value and
        // register A together results in a value bigger than 0xF.
        self.registers.F.half_carry = (self.registers.A & 0x0F) < (value & 0x0F);

        new_value
    }
    // AND instruction
    fn and(&mut self, value: u8) -> u8
    {
        let new_value = self.registers.A & value;

        // Set the flags
        self.registers.F.zero = new_value == 0;
        self.registers.F.substract = false;
        self.registers.F.carry = false;

        // Half Carry is set if adding the lower nibbles of the value and
        // register A together results in a value bigger than 0xF.
        self.registers.F.half_carry = true;

        new_value
    }
    fn or(&mut self, value: u8) -> u8
    {
        let new_value = self.registers.A | value;

        // Set the flags
        self.registers.F.zero = new_value == 0;
        self.registers.F.substract = false;
        self.registers.F.carry = false;

        // Half Carry is set if adding the lower nibbles of the value and
        // register A together results in a value bigger than 0xF.
        self.registers.F.half_carry = false;

        new_value
    }
    // XOR
    fn xor(&mut self, value: u8) -> u8
    {
        let new_value = self.registers.A ^ value;

        // Set the flags
        self.registers.F.zero = new_value == 0;
        self.registers.F.substract = false;
        self.registers.F.carry = false;

        // Half Carry is set if adding the lower nibbles of the value and
        // register A together results in a value bigger than 0xF.
        self.registers.F.half_carry = false;

        new_value
    }
    // CP instr
    fn cp(&mut self, value: u8)
    {
        let _result = self.registers.A.wrapping_sub(value);

    // Calculate flags based on the comparison.
    
    self.registers.F.zero= self.registers.A == value;
    self.registers.F.substract= true;
    self.registers.F.half_carry= (self.registers.A & 0x0F) < (value & 0x0F);
    self.registers.F.carry= self.registers.A < value;
    

    }
    
     fn inchl(&mut self, value: u16) -> u16
    {
        let (new_value, did_overflow) = value.overflowing_add(1);
        
        // Set the flags
        //self.registers.F.zero = (new_value == 0);
        self.registers.F.substract = false;
        self.registers.F.carry = did_overflow;

        // Half Carry is set if adding the lower nibbles of the value and
        // register A together results in a value bigger than 0xF.
        self.registers.F.half_carry = (value & 0x0FFF + (value + 0x0FFF)) > 0x0FFF;

        new_value
    }
    // Decrement instruction
    fn dec(&mut self, value: u8) -> u8
    {
        let (new_value, did_overflow) = value.overflowing_sub(1);
       

        // Set the flags
        self.registers.F.zero = new_value == 0;
        self.registers.F.substract = true;
        self.registers.F.half_carry = new_value & 0x0F  == 0x0F;
        

        // Half Carry is set if adding the lower nibbles of the value and
        // register A together results in a value bigger than 0xF.
        

        new_value
    }
    // Decrement 2 bytes instruction
    fn dechl(&mut self, value: u16) -> u16
    {
        let (new_value, did_overflow) = value.overflowing_sub(1);
       

        // Set the flags
        self.registers.F.zero = new_value == 0;
        self.registers.F.substract = true;
        self.registers.F.half_carry = new_value & 0x0F  == 0x0F;
        

        // Half Carry is set if adding the lower nibbles of the value and
        // register A together results in a value bigger than 0xF.
        

        new_value
    }
    // addsp instr
    fn addsp(&mut self)
    {

        let result = self.stack_pointer.wrapping_add(1);
    
        // Determine if there's a carry from bit 11 (bit 15 is the sign bit)
        let carry = (result & 0x07FF) & 0x0800 != 0;
    
        // Determine if there's a half-carry from bit 3
        let half_carry = (result & 0x000F)  & 0x0010 != 0;
    
        
        self.registers.F.zero = false; // Reset Z flag
        self.registers.F.substract = false; // Reset N flag
        self.registers.F.half_carry= half_carry; // Set or reset H flag according to operation
        self.registers.F.carry = carry; // Set or reset C flag according to operation
        
    }

    // Accumulate
    fn addhl(&mut self, value: u16) -> u16
    {
        let (new_value, did_overflow) = self.registers.get_HL().overflowing_add(value);

        // Set the flags
        self.registers.F.zero = new_value == 0;

        self.registers.F.substract = false;
        self.registers.F.carry = did_overflow;

        // Half Carry is set if adding the lower nibbles of the value and
        // register A together results in a value bigger than 0xF.

        self.registers.F.half_carry = ((self.stack_pointer & 0xF) + (value + 0xF)) > 0xF;

        new_value

    }

    
    fn bit(&mut self, r: u8, bit: u8) {
 
        let bit_mask = 1 << bit;
    let result = (r & bit_mask) == 0;

   
    self.registers.F.zero= result;
    self.registers.F.substract = false;
    self.registers.F.half_carry = true;
    
   
    }

    fn ccf(&mut self){
        self.registers.F.carry = !self.registers.F.carry;
        self.registers.F.substract = false;
        self.registers.F.half_carry = false;        
    }
    fn cpl(&mut self){
        self.registers.A = !self.registers.A;
        self.registers.F.substract = true;
        self.registers.F.half_carry = true;
        
    }
        
    fn daa(&mut self){
        self.registers.A ;  
        let a = self.registers.A;
        let mut adjust = 0;

        if self.registers.F.half_carry {
            adjust |= 0x06;
        }

        if self.registers.F.carry {
            adjust |= 0x60;
        }

        let res = if self.registers.F.substract {
            a.wrapping_sub(adjust)
        } else {
            if a & 0x0f > 0x09 {
                adjust |= 0x06;
            }

            if a > 0x99 {
                adjust |= 0x60;
            }

            a.wrapping_add(adjust)
        };

        self.registers.A = res;

        self.registers.F.zero = res == 0;
        self.registers.F.half_carry = false;
        self.registers.F.carry = adjust & 0x60 == 0x60;
     
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

            (most_significant_byte << 8) | least_significant_byte
        }
        else
        {
            // Jump instruction is 3 bytes wide, we still need to move the PC if we don't jump
            self.program_counter.wrapping_add(3)
        }
    }


    /// Unconditional jump to absolute address
    fn jp_nn(&mut self) {
        let addr = self.read_next_word();
        self.program_counter = addr;
    }

    /// Unconditional jump to address in `HL`
    fn jp_hl(&mut self) {
        let hl = self.registers.get_HL();

        // Moving from HL to PC does not require additional cycles
        // apparently.
        self.program_counter = hl;
    }

    /// Jump to absolute address if `!Z`
    fn jp_nz_nn(&mut self) {

        
       self.bus.write_byte(self.registers.get_HL(), self.bus.read_byte(self.registers.get_HL()));

    
    }

    /// Jump to absolute address if `Z`
    fn jp_z_nn(&mut self) {
        let addr = self.read_next_word();

        if self.registers.F.zero {
            self.program_counter = addr ;
        }
    }

    /// Jump to absolute address if `!C`
    fn jp_nc_nn(&mut self) {
        let addr = self.read_next_word();

        if !self.registers.F.carry {
            self.program_counter = addr ;
        }
    }

    /// Jump to absolute address if `C`
    fn jp_c_nn(&mut self) {
        let addr = self.read_next_word();

        if self.registers.F.carry {
            self.program_counter = addr ;
        }
    }

    /// Unconditional jump to relative address
    fn jr_sn(&mut self) {
        let off = self.read_next_byte() ;

        let mut pc = self.program_counter ;

        pc = pc.wrapping_add(off as u16);

        self.program_counter = pc ;
    }

    /// Jump to relative address if `!Z`
    fn jr_nz_sn(&mut self) {
        let off = self.read_next_byte() as i8;

        if !self.registers.F.zero {
            let mut pc = self.program_counter as i16;

            pc = pc.wrapping_add(off as i16);

            self.program_counter = pc as u16;
        }
    }

    /// Jump to relative address if `Z`
    fn jr_z_sn(&mut self) {
        let off = self.read_next_byte() as u8;

        if self.registers.F.zero {
            let mut pc = self.program_counter ;

            pc = pc.wrapping_add(off as u16);

            self.program_counter = pc as u16;
        }
    }

    /// Jump to relative address if `!C`
    fn jr_nc_sn(&mut self, should_jump: bool)->u16 {
     
            
                // Add the signed immediate value to the current address and jump to it
                self.program_counter.wrapping_add(2)
            
        }
    

    /// Jump to relative address if `C`
    fn jr_c_sn(&mut self) {
        let off = self.read_next_byte() as i8;

        if self.registers.F.carry {
            let mut pc = self.program_counter as i16;

            pc = pc.wrapping_add(off as i16);

            self.program_counter = pc as u16;
        }
    }


    // Accumulate
    fn add(&mut self, value: u8) -> u8
    {
        let (new_value, did_overflow) = self.registers.A.overflowing_add(value);

        // Set the flags
        self.registers.F.zero = new_value == 0;
        self.registers.F.substract = false;
        self.registers.F.carry = did_overflow;

        // Half Carry is set if adding the lower nibbles of the value and
        // register A together results in a value bigger than 0xF.
        self.registers.F.half_carry = ((self.registers.A & 0xF) + (value + 0xF)) > 0xF;

        new_value
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

    // Return
    fn return_i(&mut self, should_jump: bool) -> u16
    {
        if should_jump
        {
            self.is_interrupted=true;
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
        
        //self.bus.abort();
    }

    //RET
    fn reti(&mut self)-> u16
    {
        let return_address = self.pop();
        self.is_interrupted=true;
        return_address
    }


    //RET
    fn ret(&mut self) ->u16
    {
        let return_address = self.pop();
    return_address
 
        
    }

    //RL
    fn rl(&mut self,n:u8) -> u8
    {
        let did_overflow = n & 0x80 == 0x80;
        let new_value =
            n << 1 |
            (if self.registers.F.carry {0x01} else {0x00})
        ;

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
        let new_value =
            value << 1 |
            (if did_overflow {0x01} else {0x00}) // on change avec la valeur la plus à droit
        ;

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
        let new_value =
            value >> 1 |
            (if self.registers.F.carry {0x80} else {0x00}) // on change avec la valeur la plus à gauche
        ;

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
        let new_value =
            value >> 1 |
            (if did_overflow {0x80} else {0x00}) // on change avec la valeur la plus à gauche
        ;

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
    fn rst(&mut self,value:u16) -> u16
    {
        self.push(value);
        
        self.program_counter.wrapping_add(1)

    }

    //SBC
    fn sbc(&mut self,value:u8) -> u8
    {
        let mut n_adjusted = value;
        if self.registers.F.carry {
            n_adjusted = n_adjusted.wrapping_sub(1);
        }

        // Perform the subtraction.
        let result = self.registers.A.wrapping_sub(n_adjusted);

        self.registers.F.zero= result==0;

        self.registers.F.substract=false;

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

      fn sra(&mut self, value:u8) -> u8
      {
        let did_overflow = value & 0x80 == 0x80;
        let new_value = (value & 0xfe) >> 1;

        self.registers.F.zero = new_value == 0;
        self.registers.F.half_carry=false;
        self.registers.F.substract=false;
        self.registers.F.carry=did_overflow;

        new_value
       
      }

      fn srl(&mut self, value:u8) -> u8
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
