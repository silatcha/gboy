


mod register;
use register::flags_register;
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
    StackTarget, PrefixTarget, RSTPosition,
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
        let prefixed = (instruction_byte == 0xCB);

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
                    ArithmeticTarget::HL =>
                    {
                        let value = self.bus.read_byte( self.registers.get_HL());
                        let new_value = self.add(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },

                }
                
                   // self.add(addA);
                
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
                        self.program_counter.wrapping_add(1)
                    },
                    ADDHLTarget::DE =>
                    {
                        let value = self.registers.get_DE();
                        let new_value = self.addhl(value);
                        self.registers.set_HL(new_value);
                        self.program_counter.wrapping_add(1)
                    },
                   
                    ADDHLTarget::HL =>
                    {
                        let value = self.registers.get_HL();
                        let new_value = self.addhl(value);
                        self.registers.set_HL(new_value);
                        self.program_counter.wrapping_add(1)
                    },
                    ADDHLTarget::SP =>
                    {
                        let value = self.stack_pointer;
                        let new_value = self.addhl(value);
                        self.registers.set_HL(new_value);
                        self.program_counter.wrapping_add(1)
                    },
                    

                }
                
                   
                
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
                        self.bus.write_byte(self.registers.get_HL(), self.res((bitPosition as BitPosition).into(),value));
                        
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
                        self.bus.write_byte(self.registers.get_HL(), self.rl(value));
                        
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
                        self.bus.write_byte(self.registers.get_HL(), self.rlc(value));
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
                        
                        self.bus.write_byte(self.registers.get_HL(), self.rr(value));
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
                        
                        self.bus.write_byte(self.registers.get_HL(), self.rrc(value));
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
                        
                        self.bus.write_byte(self.registers.get_HL(), self.set((bitPosition as BitPosition).into(),value));
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
                        
                        self.bus.write_byte(self.registers.get_HL(), self.sla(value));
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
                        
                        self.bus.write_byte(self.registers.get_HL(), self.sra(value));
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
                        
                        self.bus.write_byte(self.registers.get_HL(), self.srl(value));
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
                        
                        self.bus.write_byte(self.registers.get_HL(), self.swap(value));
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
                        self.registers.A = self.cpl();
                        self.program_counter.wrapping_add(1)
                 
                
            }
            Instruction::DAA =>
            {
                        self.registers.A = self.daa();
                        self.program_counter.wrapping_add(1)
               
                
            }
            Instruction::ADC(target) =>
            {
               let addc= match target
                {
                    ArithmeticTarget::C =>
                    {
                        let value = self.registers.C;
                        let new_value = self.addc(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::B =>
                    {
                        let value = self.registers.B;
                        let new_value = self.addc(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::A =>
                    {
                        let value = self.registers.A;
                        let new_value = self.addc(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::D =>
                    {
                        let value = self.registers.D;
                        let new_value = self.addc(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::E =>
                    {
                        let value = self.registers.E;
                        let new_value = self.addc(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::H =>
                    {
                        let value = self.registers.H;
                        let new_value = self.addc(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::L =>
                    {
                        let value = self.registers.L;
                        let new_value = self.addc(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::HL =>
                    {
                        let value = self.bus.read_byte(self.registers.get_HL());
                        let new_value = self.addc(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    }

                };
                
                    self.addc(addc)
                
            }
            // SUB
            Instruction::SUB(target) =>
            {
               let subb= match target
                {
                    ArithmeticTarget::C =>
                    {
                        let value = self.registers.C;
                        let new_value = self.sub(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::B =>
                    {
                        let value = self.registers.B;
                        let new_value = self.sub(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::A =>
                    {
                        let value = self.registers.A;
                        let new_value = self.sub(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::D =>
                    {
                        let value = self.registers.D;
                        let new_value = self.sub(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::E =>
                    {
                        let value = self.registers.E;
                        let new_value = self.sub(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::H =>
                    {
                        let value = self.registers.H;
                        let new_value = self.sub(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::L =>
                    {
                        let value = self.registers.L;
                        let new_value = self.sub(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::HL =>
                    {
                        let value = self.bus.read_byte( self.registers.get_HL());
                        let new_value = self.sub(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    }

                };
                
                    self.sub(subb)
                
            }   
            // SBC
            Instruction::SBC(target) =>
            {
               let sbc= match target
                {
                    ArithmeticTarget::C =>
                    {
                        let value = self.registers.C;
                        let new_value = self.sbc(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::B =>
                    {
                        let value = self.registers.B;
                        let new_value = self.sbc(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::A =>
                    {
                        let value = self.registers.A;
                        let new_value = self.sbc(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::D =>
                    {
                        let value = self.registers.D;
                        let new_value = self.sbc(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::E =>
                    {
                        let value = self.registers.E;
                        let new_value = self.sbc(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::H =>
                    {
                        let value = self.registers.H;
                        let new_value = self.sbc(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::L =>
                    {
                        let value = self.registers.L;
                        let new_value = self.sbc(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::HL =>
                    {
                        let value = self.bus.read_byte(self.registers.get_HL());
                        let new_value = self.sbc(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    }
                };
                
                    self.sbc(sbc)
                
            } 
            // AND
            Instruction::AND(target) =>
            {
               let and= match target
                {
                    ArithmeticTarget::C =>
                    {
                        let value = self.registers.C;
                        let new_value = self.and(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::B =>
                    {
                        let value = self.registers.B;
                        let new_value = self.and(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::A =>
                    {
                        let value = self.registers.A;
                        let new_value = self.and(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::D =>
                    {
                        let value = self.registers.D;
                        let new_value = self.and(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::E =>
                    {
                        let value = self.registers.E;
                        let new_value = self.and(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::H =>
                    {
                        let value = self.registers.H;
                        let new_value = self.and(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::L =>
                    {
                        let value = self.registers.L;
                        let new_value = self.and(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    }
                    ArithmeticTarget::HL =>
                    {
                        let value = self.bus.read_byte(self.registers.get_HL());
                        let new_value = self.and(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    }

                };
                
                    self.and(and)
                
            }
            // OR
            Instruction::OR(target) =>
            {
               let or= match target
                {
                    ArithmeticTarget::C =>
                    {
                        let value = self.registers.C;
                        let new_value = self.or(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::B =>
                    {
                        let value = self.registers.B;
                        let new_value = self.or(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::A =>
                    {
                        let value = self.registers.A;
                        let new_value = self.or(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::D =>
                    {
                        let value = self.registers.D;
                        let new_value = self.or(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::E =>
                    {
                        let value = self.registers.E;
                        let new_value = self.or(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::H =>
                    {
                        let value = self.registers.H;
                        let new_value = self.or(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::L =>
                    {
                        let value = self.registers.L;
                        let new_value = self.or(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    }

                };
                
                    self.or(and)
                
            }
            // XOR
            Instruction::XOR(target) =>
            {
               let xor= match target
                {
                    ArithmeticTarget::C =>
                    {
                        let value = self.registers.C;
                        let new_value = self.xor(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::B =>
                    {
                        let value = self.registers.B;
                        let new_value = self.xor(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::A =>
                    {
                        let value = self.registers.A;
                        let new_value = self.xor(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::D =>
                    {
                        let value = self.registers.D;
                        let new_value = self.xor(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::E =>
                    {
                        let value = self.registers.E;
                        let new_value = self.xor(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::H =>
                    {
                        let value = self.registers.H;
                        let new_value = self.xor(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::L =>
                    {
                        let value = self.registers.L;
                        let new_value = self.xor(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    }

                };
                
                    self.xor(and)
                
            }
            // CP 
            Instruction::CP(target) =>
            {
               let xor= match target
                {
                    ArithmeticTarget::C =>
                    {
                        let value = self.registers.C;
                        let new_value = self.cp(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::B =>
                    {
                        let value = self.registers.B;
                        let new_value = self.cp(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::A =>
                    {
                        let value = self.registers.A;
                        let new_value = self.cp(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::D =>
                    {
                        let value = self.registers.D;
                        let new_value = self.cp(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::E =>
                    {
                        let value = self.registers.E;
                        let new_value = self.cp(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::H =>
                    {
                        let value = self.registers.H;
                        let new_value = self.cp(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::L =>
                    {
                        let value = self.registers.L;
                        let new_value = self.cp(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    }

                };
                
                    self.cp(and)
                
            }
            // INCREMENT 
            Instruction::INC(target) =>
            {
               let inc= match target
                {
                    ArithmeticTarget::C =>
                    {
                        let value = self.registers.C;
                        let new_value = self.inc(value);
                        self.registers.C = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::B =>
                    {
                        let value = self.registers.B;
                        let new_value = self.inc(value);
                        self.registers.B = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::A =>
                    {
                        let value = self.registers.A;
                        let new_value = self.inc(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::D =>
                    {
                        let value = self.registers.D;
                        let new_value = self.inc(value);
                        self.registers.D = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::E =>
                    {
                        let value = self.registers.E;
                        let new_value = self.inc(value);
                        self.registers.E = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::H =>
                    {
                        let value = self.registers.H;
                        let new_value = self.inc(value);
                        self.registers.H = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::L =>
                    {
                        let value = self.registers.L;
                        let new_value = self.inc(value);
                        self.registers.L = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::HLI =>
                    {
                        let value = self.registers.HLI;
                        let new_value = self.inchl(value);
                        self.registers.HLI = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::BC =>
                    {
                        let value = self.registers.BC;
                        let new_value = self.inchl(value);
                        self.registers.BC = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::DE =>
                    {
                        let value = self.registers.DE;
                        let new_value = self.inchl(value);
                        self.registers.DE = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::DE =>
                    {
                        let value = self.registers.DE;
                        let new_value = self.inchl(value);
                        self.registers.DE = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::HL =>
                    {
                        let value = self.registers.HL;
                        let new_value = self.inchl(value);
                        self.registers.HL = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::SP =>
                    {
                        let value = self.registers.SP;
                        let new_value = self.inchl(value);
                        self.registers.SP = new_value;
                        self.program_counter.wrapping_add(1)
                    }

                };
                
                    self.inc(inc)
                
            }
            // DECREMENT 
            Instruction::DEC(target) =>
            {
               let dec= match target
                {
                    ArithmeticTarget::C =>
                    {
                        let value = self.registers.C;
                        let new_value = self.dec(value);
                        self.registers.C = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::B =>
                    {
                        let value = self.registers.B;
                        let new_value = self.dec(value);
                        self.registers.B = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::A =>
                    {
                        let value = self.registers.A;
                        let new_value = self.dec(value);
                        self.registers.A = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::D =>
                    {
                        let value = self.registers.D;
                        let new_value = self.dec(value);
                        self.registers.D = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::E =>
                    {
                        let value = self.registers.E;
                        let new_value = self.dec(value);
                        self.registers.E = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::H =>
                    {
                        let value = self.registers.H;
                        let new_value = self.dec(value);
                        self.registers.H = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::L =>
                    {
                        let value = self.registers.L;
                        let new_value = self.dec(value);
                        self.registers.L = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::HLI =>
                    {
                        let value = self.registers.HLI;
                        let new_value = self.dechl(value);
                        self.registers.HLI = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::BC =>
                    {
                        let value = self.registers.BC;
                        let new_value = self.dechl(value);
                        self.registers.BC = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::DE =>
                    {
                        let value = self.registers.DE;
                        let new_value = self.dechl(value);
                        self.registers.DE = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::DE =>
                    {
                        let value = self.registers.DE;
                        let new_value = self.dechl(value);
                        self.registers.DE = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::HL =>
                    {
                        let value = self.registers.HL;
                        let new_value = self.dechl(value);
                        self.registers.HL = new_value;
                        self.program_counter.wrapping_add(1)
                    },
                    ArithmeticTarget::SP =>
                    {
                        let value = self.registers.SP;
                        let new_value = self.dechl(value);
                        self.registers.SP = new_value;
                        self.program_counter.wrapping_add(1)
                    }

                };
                
                    self.dec(inc)
                
            }
            // BIT OPERATION 
            Instruction::BIT(target1, target2) =>
            {
               let (r,b)= match (target1, target2)
                {
                    (PrefixTarget::C, BitPosition::B0) =>
                    {                       
                        let new_value = self.bit_generic(self.registers.C, 0);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::B, BitPosition::B0) =>
                    {
                        let new_value = self.bit_generic(self.registers.B, 0);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::A, BitPosition::B0) =>
                    {
                        let new_value = self.bit_generic(self.registers.A, 0);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::D, BitPosition::B0)=>
                    {
                        let new_value = self.bit_generic(self.registers.D, 0);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::E, BitPosition::B0) =>
                    {
                        let new_value = self.bit_generic(self.registers.E, 0);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::H, BitPosition::B0) =>
                    {
                        let new_value = self.bit_generic(self.registers.H, 0);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::L, BitPosition::B0) =>
                    {
                        let new_value = self.bit_generic(self.registers.L, 0);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::HL, BitPosition::B0) =>
                    {
                        let new_value = self.bit_generic2(self.registers.get_HL(), 0);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::C, BitPosition::B1) =>
                    {                       
                        let new_value = self.bit_generic(self.registers.C, 1);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::B, BitPosition::B1) =>
                    {
                        let new_value = self.bit_generic(self.registers.B, 1);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::A, BitPosition::B1) =>
                    {
                        let new_value = self.bit_generic(self.registers.A, 1);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::D, BitPosition::B1)=>
                    {
                        let new_value = self.bit_generic(self.registers.D, 1);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::E, BitPosition::B1) =>
                    {
                        let new_value = self.bit_generic(self.registers.E, 1);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::H, BitPosition::B1) =>
                    {
                        let new_value = self.bit_generic(self.registers.H, 1);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::L, BitPosition::B1) =>
                    {
                        let new_value = self.bit_generic(self.registers.L, 1);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::HL, BitPosition::B1) =>
                    {
                        let new_value = self.bit_generic2(self.registers.get_HL(), 1);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::C, BitPosition::B2) =>
                    {                       
                        let new_value = self.bit_generic(self.registers.C, 2);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::B, BitPosition::B2) =>
                    {
                        let new_value = self.bit_generic(self.registers.B, 2);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::A, BitPosition::B2) =>
                    {
                        let new_value = self.bit_generic(self.registers.A, 2);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::D, BitPosition::B2)=>
                    {
                        let new_value = self.bit_generic(self.registers.D, 2);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::E, BitPosition::B2) =>
                    {
                        let new_value = self.bit_generic(self.registers.E, 2);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::H, BitPosition::B2) =>
                    {
                        let new_value = self.bit_generic(self.registers.H, 2);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::L, BitPosition::B2) =>
                    {
                        let new_value = self.bit_generic(self.registers.L, 2);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::HL, BitPosition::B2) =>
                    {
                        let new_value = self.bit_generic2(self.registers.get_HL(), 2);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::C, BitPosition::B3) =>
                    {                       
                        let new_value = self.bit_generic(self.registers.C, 3);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::B, BitPosition::B3) =>
                    {
                        let new_value = self.bit_generic(self.registers.B, 3);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::A, BitPosition::B3) =>
                    {
                        let new_value = self.bit_generic(self.registers.A, 3);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::D, BitPosition::B3)=>
                    {
                        let new_value = self.bit_generic(self.registers.D, 3);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::E, BitPosition::B3) =>
                    {
                        let new_value = self.bit_generic(self.registers.E, 3);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::H, BitPosition::B3) =>
                    {
                        let new_value = self.bit_generic(self.registers.H, 3);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::L, BitPosition::B3) =>
                    {
                        let new_value = self.bit_generic(self.registers.L, 3);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::HL, BitPosition::B3) =>
                    {
                        let new_value = self.bit_generic2(self.registers.get_HL(), 3);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::C, BitPosition::B4) =>
                    {                       
                        let new_value = self.bit_generic(self.registers.C, 4);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::B, BitPosition::B4) =>
                    {
                        let new_value = self.bit_generic(self.registers.B, 4);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::A, BitPosition::B4) =>
                    {
                        let new_value = self.bit_generic(self.registers.A, 4);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::D, BitPosition::B4)=>
                    {
                        let new_value = self.bit_generic(self.registers.D, 4);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::E, BitPosition::B4) =>
                    {
                        let new_value = self.bit_generic(self.registers.E, 4);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::H, BitPosition::B4) =>
                    {
                        let new_value = self.bit_generic(self.registers.H, 4);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::L, BitPosition::B4) =>
                    {
                        let new_value = self.bit_generic(self.registers.L, 4);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::HL, BitPosition::B4) =>
                    {
                        let new_value = self.bit_generic2(self.registers.get_HL(), 4);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::C, BitPosition::B5) =>
                    {                       
                        let new_value = self.bit_generic(self.registers.C, 5);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::B, BitPosition::B5) =>
                    {
                        let new_value = self.bit_generic(self.registers.B, 5);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::A, BitPosition::B5) =>
                    {
                        let new_value = self.bit_generic(self.registers.A, 5);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::D, BitPosition::B5)=>
                    {
                        let new_value = self.bit_generic(self.registers.D, 5);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::E, BitPosition::B5) =>
                    {
                        let new_value = self.bit_generic(self.registers.E, 5);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::H, BitPosition::B5) =>
                    {
                        let new_value = self.bit_generic(self.registers.H, 5);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::L, BitPosition::B5) =>
                    {
                        let new_value = self.bit_generic(self.registers.L, 5);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::HL, BitPosition::B5) =>
                    {
                        let new_value = self.bit_generic2(self.registers.get_HL(), 5);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::C, BitPosition::B6) =>
                    {                       
                        let new_value = self.bit_generic(self.registers.C, 6);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::B, BitPosition::B6) =>
                    {
                        let new_value = self.bit_generic(self.registers.B, 6);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::A, BitPosition::B6) =>
                    {
                        let new_value = self.bit_generic(self.registers.A, 6);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::D, BitPosition::B6)=>
                    {
                        let new_value = self.bit_generic(self.registers.D, 6);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::E, BitPosition::B6) =>
                    {
                        let new_value = self.bit_generic(self.registers.E, 6);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::H, BitPosition::B6) =>
                    {
                        let new_value = self.bit_generic(self.registers.H, 6);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::L, BitPosition::B6) =>
                    {
                        let new_value = self.bit_generic(self.registers.L, 6);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::HL, BitPosition::B6) =>
                    {
                        let new_value = self.bit_generic2(self.registers.get_HL(), 6);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::C, BitPosition::B7) =>
                    {                       
                        let new_value = self.bit_generic(self.registers.C, 7);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::B, BitPosition::B7) =>
                    {
                        let new_value = self.bit_generic(self.registers.B, 7);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::A, BitPosition::B7) =>
                    {
                        let new_value = self.bit_generic(self.registers.A, 7);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::D, BitPosition::B7)=>
                    {
                        let new_value = self.bit_generic(self.registers.D, 7);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::E, BitPosition::B7) =>
                    {
                        let new_value = self.bit_generic(self.registers.E, 7);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::H, BitPosition::B7) =>
                    {
                        let new_value = self.bit_generic(self.registers.H, 7);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::L, BitPosition::B7) =>
                    {
                        let new_value = self.bit_generic(self.registers.L, 7);                        
                        self.program_counter.wrapping_add(1)
                    },
                    (PrefixTarget::HL, BitPosition::B7) =>
                    {
                        let new_value = self.bit_generic2(self.registers.get_HL(), 7);                        
                        self.program_counter.wrapping_add(1)
                    }
                    

                };
                
                    self.bit_generic(r, b)
                
            }
            // JP
            Instruction::JP(target) =>
            {
                let jump_condition = match target
                {
                    JumpTest::NZ => 
                    {
                        self.jp_nz_nn();                       
                    },
                    JumpTest::NC => 
                    {
                        self.jp_nc_nn();                 
                    },
                    JumpTest::Z  => 
                    {
                        self.jp_z_nn();                    
                    },
                    JumpTest::C  => 
                    {
                        self.jp_c_nn();
                    },
                    JumpTest::A  => 
                    {                        
                        self.jp_nn();    
                    }
                };
                self.jump(jump_condition)
            }
            // JR
            Instruction::JR(target) =>
            {
                let jump_condition = match target
                {
                    JumpTest::NZ => 
                    {
                        self.jr_nz_sn();                       
                    },
                    JumpTest::NC => 
                    {
                        self.jr_nc_sn();                 
                    },
                    JumpTest::Z  => 
                    {
                        self.jr_z_sn();                    
                    },
                    JumpTest::C  => 
                    {
                        self.jr_c_sn();
                    },
                    JumpTest::A  => 
                    {                        
                        self.jp_sn();    
                    }
                };
                self.jump(jump_condition)
            }
            Instruction::JPHL =>
            {
                self.jp_hl()               
            }
            // LD
            Instruction::LD(load_type) =>
            {
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
                        LoadByteSource::BC => self.registers.get_BC(),
                        LoadByteSource::DE => self.registers.get_HL(),
                        LoadByteSource::HL => self.registers.get_HL(),
                        LoadByteSource::D8 => self.read_next_byte(),
                       // LoadByteSource:: => self.registers.C,
                        LoadByteSource::HLD => self.bus.read_byte(self.registers.get_HL()),
                        LoadByteSource::HLI => self.bus.read_byte(self.registers.get_HL())                       
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
                        _ =>
                        {
                            // TODO: implement other targets
                        }
                    };
                    match source
                    {
                        LoadByteSource::D8  => self.program_counter.wrapping_add(2),
                        _                   => self.program_counter.wrapping_add(1),
                    }
                    
                }
            }
            // CALL
            Instruction::CALL(target) =>
            {
                let jump_condition = match target
                {
                    JumpTest::NZ => 
                    { 
                        if !self.registers.F.zero 
                        {
                            self.program_counter = self.call(!self.registers.F.zero);
                        }
                    },
                    JumpTest::Z => 
                    { 
                        if self.registers.F.zero 
                        {
                            self.program_counter = self.call(self.registers.F.zero);
                        }
                    },
                    JumpTest::NC => 
                    {
                       if !self.registers.F.carry
                       {
                        self.program_counter = self.call(!self.registers.F.carry);
                       }
                    },
                    JumpTest::C => 
                    {
                       if self.registers.F.carry
                       {
                        self.program_counter = self.call(self.registers.F.carry);
                       }
                    },
                    JumpTest::A => 
                    {
                        self.program_counter = self.call(true);
                    }
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
    
    
    
    // Substract instruction
    fn subhl(&mut self, value: u16) -> u16
    {
        let (new_value, did_overflow) = self.registers.get_HL().overflowing_sub(value);

        // Set the flags
        self.registers.F.zero = (new_value == 0);
        self.registers.F.substract = true;
        self.registers.F.carry = did_overflow;

        // Half Carry is set if adding the lower nibbles of the value and
        // register A together results in a value bigger than 0xF.
        self.registers.F.half_carry = ((self.registers.A & 0x1FFF) < (value & 0x1FFF));

        new_value
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

    result = result.wrapping_sub((if self.registers.F.carry{ 1 } else { 0 }));
    self.registers.F.zero= result == 0;

    result

    }
   
    // Substract instruction
    fn sub(&mut self, value: u8) -> u8
    {
        let (new_value, did_overflow) = self.registers.A.overflowing_sub(value);

        // Set the flags
        self.registers.F.zero = (new_value == 0);
        self.registers.F.substract = true;
        self.registers.F.carry = did_overflow;

        // Half Carry is set if adding the lower nibbles of the value and
        // register A together results in a value bigger than 0xF.
        self.registers.F.half_carry = ((self.registers.A & 0x0F) < (value & 0x0F));

        new_value
    }
    // AND instruction
    fn and(&mut self, value: u8) -> u8
    {
        let new_value = self.registers.A & value;

        // Set the flags
        self.registers.F.zero = (new_value == 0);
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
        self.registers.F.zero = (new_value == 0);
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
        self.registers.F.zero = (new_value == 0);
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
        let (new_value, did_overflow) = self.registers.A.overflowing_sub(value);

        // Set the flags
        self.registers.F.zero = (new_value == 0);
        self.registers.F.substract = true;
        self.registers.F.carry = did_overflow;

        // Half Carry is set if adding the lower nibbles of the value and
        // register A together results in a value bigger than 0xF.
        self.registers.F.half_carry = ((self.registers.A & 0x0F) < (value & 0x0F));
        
        value

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
        self.registers.F.half_carry = ((value & 0x0FFF + (value + 0x0FFF)) > 0x0FFF);

        new_value
    }
    // Decrement instruction
    fn dec(&mut self, value: u8) -> u8
    {
        let (new_value, did_overflow) = value.overflowing_sub(1);

        // Set the flags
        self.registers.F.zero = (new_value == 0);
        self.registers.F.substract = true;
        self.registers.F.carry = did_overflow;

        // Half Carry is set if adding the lower nibbles of the value and
        // register A together results in a value bigger than 0xF.
        self.registers.F.half_carry = ((value & 0x0F) < (value & 0x0F));

        new_value
    }
    // Decrement 2 bytes instruction
    fn dechl(&mut self, value: u16) -> u16
    {
        let (new_value, did_overflow) = value.overflowing_sub(1);

        // Set the flags
        self.registers.F.zero = (new_value == 0);
        self.registers.F.substract = true;
        self.registers.F.carry = did_overflow;

        // Half Carry is set if adding the lower nibbles of the value and
        // register A together results in a value bigger than 0xF.
        self.registers.F.half_carry = ((value & 0x1FFF) < (value & 0x1FFF));

        new_value
    }
    // addsp instr
    fn addsp(&mut self)
    {
        //self.registers.A =value;
        let (new_value, did_overflow) = self.stack_pointer.wrapping_add(1);

        // Set the flags
        self.registers.F.zero = false;

        self.registers.F.substract=false;
        self.registers.F.half_carry=did_overflow || (new_value & 0x0F);
        self.registers.F.carry= (new_value as u32 + n as u32) > 0xFFFF,

        new_value
    }

    // Accumulate
    fn addhl(&mut self, value: u16) -> u16
    {
        let (new_value, did_overflow) = self.registers.get_HL().overflowing_add(value);

        // Set the flags
        self.registers.F.zero = (new_value == 0);

        self.registers.F.substract = false;
        self.registers.F.carry = did_overflow;

        // Half Carry is set if adding the lower nibbles of the value and
        // register A together results in a value bigger than 0xF.

        self.registers.F.half_carry = ((self.registers.SP & 0xF) + (value + 0xF)) > 0xF;

        new_value

    }
    fn bit_zero(val: u8, bit: u8) -> bool {
        (val & (1u8 << (bit as usize))) == 0
    }
    fn bit_zero2(val: u16, bit: u16) -> bool {
        (val & (1u16 << (bit as usize))) == 0
    }
    
    fn bit_generic(&mut self, r: u8, bit: u8) {
        self.registers.F.substract = false;
        self.registers.F.zero = bit_zero(r, bit);
        self.registers.F.half_carry = true;
    }
    fn bit_generic2(&mut self, r: u16, bit: u16) {
        self.registers.F.substract = false;
        self.registers.F.zero = bit_zero(r, bit);
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
        self.registers.A        
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

        self.registers.F.zero = (res == 0);
        self.registers.F.half_carry = false;
        self.registers.F.carry = (adjust & 0x60 == 0x60);
     
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
        let addr = self.read_next_word();

        if !self.registers.F.zero {
            self.program_counter = addr ;
        }
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
        let off = self.read_next_byte() as i8;

        let mut pc = self.program_counter as i16;

        pc = pc.wrapping_add(off as i16);

        self.program_counter = pc as u16;
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
        let off = self.read_next_byte() as i8;

        if self.registers.F.zero {
            let mut pc = self.program_counter as i16;

            pc = pc.wrapping_add(off as i16);

            self.program_counter = pc as u16;
        }
    }

    /// Jump to relative address if `!C`
    fn jr_nc_sn(&mut self) {
        let off = self.read_next_byte() as i8;

        if !self.registers.F.carry {
            let mut pc = self.program_counter as i16;

            pc = pc.wrapping_add(off as i16);

            self.program_counter = pc as u16;
        }
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
        self.registers.F.zero = (new_value == 0);
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
        
        self.jump(value)

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