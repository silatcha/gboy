use std;
use targets::*;

// -------------------------------------------------------
// OpCodes Summary
pub enum Instruction
{
    // 8-bit & 16-bit Loads
    LD(LoadType),

    LDC(LoadType),
    LDHLI(LoadType),
    LDHLD(LoadType),
    LDHLI(LoadType),
    LDR(LoadType),
    PUSH(StackTarget),
    POP(StackTarget),
    LDH(targetA),
    LDHS(sourceA),

    // 8-bit ALU
    ADD(ArithmeticTarget),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),
    CP(ArithmeticTarget),

    // 8-bit & 16-bit Arithmetic
    INC(IncDecTarget),
    DEC(IncDecTarget),

    // 16-bit Arithmetic
    ADDHL(ADDHLTarget),

    SUBHL(ADDHLTarget),

    ADDSP,

    // Miscellaneous
    SWAP(PrefixTarget),
    DAA,
    CPL,

    CCF,
    SCF,

    NOP,
    HALT,
    STOP,
    DI,
    EI,    

    // Rotates & Shifts
    RLCA,
    RLA,
    RRCA,
    RRA,

    RLC(PrefixTarget),
    RL(PrefixTarget),
    RRC(PrefixTarget),
    RR(PrefixTarget),

    SLA(PrefixTarget),
    SRA(PrefixTarget),
    SRL(PrefixTarget),
    
    // Bit Opcodes
    BIT(PrefixTarget, BitPosition),
    RES(PrefixTarget, BitPosition),
    SET(PrefixTarget, BitPosition),

    // Jumps
    JP(JumpTest),
    JR(JumpTest),

    JPHL,

    JPI,


    // Calls
    CALL(JumpTest),

    // Restarts
    RST(RSTPosition),

    // Returns
    RET(JumpTest),
    RETI,
}

impl Instruction
{
    // Decodes an instruction into a byte
    fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction>
    {
        if prefixed
        {
            Instruction::from_byte_prefixed(byte)
        }
        else
        {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    // Decodes an instruction into a byte, if prefixed then one extra byte
    fn from_byte_prefixed(byte: u8) -> Option<Instruction>
    {
        match byte
        {
            0x00 => Some(Instruction::RLC(PrefixTarget::B)),
            0x01 => Some(Instruction::RLC(PrefixTarget::C)),
            0x02 => Some(Instruction::RLC(PrefixTarget::D)),
            0x03 => Some(Instruction::RLC(PrefixTarget::E)),
            0x04 => Some(Instruction::RLC(PrefixTarget::H)),
            0x05 => Some(Instruction::RLC(PrefixTarget::L)),
            0x06 => Some(Instruction::RLC(PrefixTarget::HLI)),
            0x07 => Some(Instruction::RLC(PrefixTarget::A)),
            0x08 => Some(Instruction::RRC(PrefixTarget::B)),
            0x09 => Some(Instruction::RRC(PrefixTarget::C)),
            0x0A => Some(Instruction::RRC(PrefixTarget::D)),
            0x0B => Some(Instruction::RRC(PrefixTarget::E)),
            0x0C => Some(Instruction::RRC(PrefixTarget::H)),
            0x0E => Some(Instruction::RRC(PrefixTarget::HLI)),
            0x0F => Some(Instruction::RRC(PrefixTarget::A)),
            0x10 => Some(Instruction::RL(PrefixTarget::B)),
            0x11 => Some(Instruction::RL(PrefixTarget::C)),
            0x12 => Some(Instruction::RL(PrefixTarget::D)),
            0x13 => Some(Instruction::RL(PrefixTarget::E)),
            0x14 => Some(Instruction::RL(PrefixTarget::H)),
            0x15 => Some(Instruction::RL(PrefixTarget::L)),
            0x16 => Some(Instruction::RL(PrefixTarget::HLI)),
            0x17 => Some(Instruction::RL(PrefixTarget::A)),
            0x18 => Some(Instruction::RR(PrefixTarget::B)),
            0x19 => Some(Instruction::RR(PrefixTarget::C)),
            0x1A => Some(Instruction::RR(PrefixTarget::D)),
            0x1B => Some(Instruction::RR(PrefixTarget::E)),
            0x1C => Some(Instruction::RR(PrefixTarget::H)),
            0x1E => Some(Instruction::RR(PrefixTarget::HLI)),
            0x1F => Some(Instruction::RR(PrefixTarget::A)),
            0x20 => Some(Instruction::SLA(PrefixTarget::B)),
            0x21 => Some(Instruction::SLA(PrefixTarget::C)),
            0x22 => Some(Instruction::SLA(PrefixTarget::D)),
            0x23 => Some(Instruction::SLA(PrefixTarget::E)),
            0x24 => Some(Instruction::SLA(PrefixTarget::H)),
            0x25 => Some(Instruction::SLA(PrefixTarget::L)),
            0x26 => Some(Instruction::SLA(PrefixTarget::HLI)),
            0x27 => Some(Instruction::SLA(PrefixTarget::A)),
            0x28 => Some(Instruction::SRA(PrefixTarget::B)),
            0x29 => Some(Instruction::SRA(PrefixTarget::C)),
            0x2A => Some(Instruction::SRA(PrefixTarget::D)),
            0x2B => Some(Instruction::SRA(PrefixTarget::E)),
            0x2C => Some(Instruction::SRA(PrefixTarget::H)),
            0x2E => Some(Instruction::SRA(PrefixTarget::HLI)),
            0x2F => Some(Instruction::SRA(PrefixTarget::A)),
            0x30 => Some(Instruction::SWAP(PrefixTarget::B)),
            0x31 => Some(Instruction::SWAP(PrefixTarget::C)),
            0x32 => Some(Instruction::SWAP(PrefixTarget::D)),
            0x33 => Some(Instruction::SWAP(PrefixTarget::E)),
            0x34 => Some(Instruction::SWAP(PrefixTarget::H)),
            0x35 => Some(Instruction::SWAP(PrefixTarget::L)),
            0x36 => Some(Instruction::SWAP(PrefixTarget::HLI)),
            0x37 => Some(Instruction::SWAP(PrefixTarget::A)),
            0x38 => Some(Instruction::SRL(PrefixTarget::B)),
            0x39 => Some(Instruction::SRL(PrefixTarget::C)),
            0x3A => Some(Instruction::SRL(PrefixTarget::D)),
            0x3B => Some(Instruction::SRL(PrefixTarget::E)),
            0x3C => Some(Instruction::SRL(PrefixTarget::H)),
            0x3E => Some(Instruction::SRL(PrefixTarget::HLI)),
            0x3F => Some(Instruction::SRL(PrefixTarget::A)),
            0x40 => Some(Instruction::BIT(PrefixTarget::B,BitPosition::B0)),
            0x41 => Some(Instruction::BIT(PrefixTarget::C,BitPosition::B0)),
            0x42 => Some(Instruction::BIT(PrefixTarget::D,BitPosition::B0)),
            0x43 => Some(Instruction::BIT(PrefixTarget::E,BitPosition::B0)),
            0x44 => Some(Instruction::BIT(PrefixTarget::H,BitPosition::B0)),
            0x45 => Some(Instruction::BIT(PrefixTarget::L,BitPosition::B0)),
            0x46 => Some(Instruction::BIT(PrefixTarget::HLI,BitPosition::B0)),
            0x47 => Some(Instruction::BIT(PrefixTarget::A,BitPosition::B0)),
            0x48 => Some(Instruction::BIT(PrefixTarget::B,BitPosition::B1)),
            0x49 => Some(Instruction::BIT(PrefixTarget::C,BitPosition::B1)),
            0x4A => Some(Instruction::BIT(PrefixTarget::D,BitPosition::B1)),
            0x4B => Some(Instruction::BIT(PrefixTarget::E,BitPosition::B1)),
            0x4C => Some(Instruction::BIT(PrefixTarget::H,BitPosition::B1)),
            0x4E => Some(Instruction::BIT(PrefixTarget::HLI,BitPosition::B1)),
            0x4F => Some(Instruction::BIT(PrefixTarget::A,BitPosition::B1)),
            0x50 => Some(Instruction::BIT(PrefixTarget::B,BitPosition::B2)),
            0x51 => Some(Instruction::BIT(PrefixTarget::C,BitPosition::B2)),
            0x52 => Some(Instruction::BIT(PrefixTarget::D,BitPosition::B2)),
            0x53 => Some(Instruction::BIT(PrefixTarget::E,BitPosition::B2)),
            0x54 => Some(Instruction::BIT(PrefixTarget::H,BitPosition::B2)),
            0x55 => Some(Instruction::BIT(PrefixTarget::L,BitPosition::B2)),
            0x56 => Some(Instruction::BIT(PrefixTarget::HLI,BitPosition::B2)),
            0x57 => Some(Instruction::BIT(PrefixTarget::A,BitPosition::B2)),
            0x58 => Some(Instruction::BIT(PrefixTarget::B,BitPosition::B3)),
            0x59 => Some(Instruction::BIT(PrefixTarget::C,BitPosition::B3)),
            0x5A => Some(Instruction::BIT(PrefixTarget::D,BitPosition::B3)),
            0x5B => Some(Instruction::BIT(PrefixTarget::E,BitPosition::B3)),
            0x5C => Some(Instruction::BIT(PrefixTarget::H,BitPosition::B3)),
            0x5E => Some(Instruction::BIT(PrefixTarget::HLI,BitPosition::B3)),
            0x5F => Some(Instruction::BIT(PrefixTarget::A,BitPosition::B3)),
            0x60 => Some(Instruction::BIT(PrefixTarget::B,BitPosition::B4)),
            0x61 => Some(Instruction::BIT(PrefixTarget::C,BitPosition::B4)),
            0x62 => Some(Instruction::BIT(PrefixTarget::D,BitPosition::B4)),
            0x63 => Some(Instruction::BIT(PrefixTarget::E,BitPosition::B4)),
            0x64 => Some(Instruction::BIT(PrefixTarget::H,BitPosition::B4)),
            0x65 => Some(Instruction::BIT(PrefixTarget::L,BitPosition::B4)),
            0x66 => Some(Instruction::BIT(PrefixTarget::HLI,BitPosition::B4)),
            0x67 => Some(Instruction::BIT(PrefixTarget::A,BitPosition::B4)),
            0x68 => Some(Instruction::BIT(PrefixTarget::B,BitPosition::B5)),
            0x69 => Some(Instruction::BIT(PrefixTarget::C,BitPosition::B5)),
            0x6A => Some(Instruction::BIT(PrefixTarget::D,BitPosition::B5)),
            0x6B => Some(Instruction::BIT(PrefixTarget::E,BitPosition::B5)),
            0x6C => Some(Instruction::BIT(PrefixTarget::H,BitPosition::B5)),
            0x6E => Some(Instruction::BIT(PrefixTarget::HLI,BitPosition::B5)),
            0x6F => Some(Instruction::BIT(PrefixTarget::A,BitPosition::B5)),
            0x70 => Some(Instruction::BIT(PrefixTarget::B,BitPosition::B6)),
            0x71 => Some(Instruction::BIT(PrefixTarget::C,BitPosition::B6)),
            0x72 => Some(Instruction::BIT(PrefixTarget::D,BitPosition::B6)),
            0x73 => Some(Instruction::BIT(PrefixTarget::E,BitPosition::B6)),
            0x74 => Some(Instruction::BIT(PrefixTarget::H,BitPosition::B6)),
            0x75 => Some(Instruction::BIT(PrefixTarget::L,BitPosition::B6)),
            0x76 => Some(Instruction::BIT(PrefixTarget::HLI,BitPosition::B6)),
            0x77 => Some(Instruction::BIT(PrefixTarget::A,BitPosition::B6)),
            0x78 => Some(Instruction::BIT(PrefixTarget::B,BitPosition::B7)),
            0x79 => Some(Instruction::BIT(PrefixTarget::C,BitPosition::B7)),
            0x7A => Some(Instruction::BIT(PrefixTarget::D,BitPosition::B7)),
            0x7B => Some(Instruction::BIT(PrefixTarget::E,BitPosition::B7)),
            0x7C => Some(Instruction::BIT(PrefixTarget::H,BitPosition::B7)),
            0x7E => Some(Instruction::BIT(PrefixTarget::HLI,BitPosition::B7)),
            0x7F => Some(Instruction::BIT(PrefixTarget::A,BitPosition::B7)),
            0x80 => Some(Instruction::RES(PrefixTarget::B,BitPosition::B0)),
            0x81 => Some(Instruction::RES(PrefixTarget::C,BitPosition::B0)),
            0x82 => Some(Instruction::RES(PrefixTarget::D,BitPosition::B0)),
            0x83 => Some(Instruction::RES(PrefixTarget::E,BitPosition::B0)),
            0x84 => Some(Instruction::RES(PrefixTarget::H,BitPosition::B0)),
            0x85 => Some(Instruction::RES(PrefixTarget::L,BitPosition::B0)),
            0x86 => Some(Instruction::RES(PrefixTarget::HLI,BitPosition::B0)),
            0x87 => Some(Instruction::RES(PrefixTarget::A,BitPosition::B0)),
            0x88 => Some(Instruction::RES(PrefixTarget::B,BitPosition::B1)),
            0x89 => Some(Instruction::RES(PrefixTarget::C,BitPosition::B1)),
            0x8A => Some(Instruction::RES(PrefixTarget::D,BitPosition::B1)),
            0x8B => Some(Instruction::RES(PrefixTarget::E,BitPosition::BBitPosition::B1)),
            0x8C => Some(Instruction::RES(PrefixTarget::H,BitPosition::B1)),
            0x8E => Some(Instruction::RES(PrefixTarget::HLI,BitPosition::B1)),
            0x8F => Some(Instruction::RES(PrefixTarget::A,BitPosition::B1)),
            0x90 => Some(Instruction::RES(PrefixTarget::B,BitPosition::B2)),
            0x91 => Some(Instruction::RES(PrefixTarget::C,BitPosition::B2)),
            0x92 => Some(Instruction::RES(PrefixTarget::D,BitPosition::B2)),
            0x93 => Some(Instruction::RES(PrefixTarget::E,BitPosition::B2)),
            0x94 => Some(Instruction::RES(PrefixTarget::H,BitPosition::B2)),
            0x95 => Some(Instruction::RES(PrefixTarget::L,BitPosition::B2)),
            0x96 => Some(Instruction::RES(PrefixTarget::HLI,BitPosition::B2)),
            0x97 => Some(Instruction::RES(PrefixTarget::A,BitPosition::B2)),
            0x98 => Some(Instruction::RES(PrefixTarget::B,BitPosition::B3)),
            0x99 => Some(Instruction::RES(PrefixTarget::C,BitPosition::B3)),
            0x9A => Some(Instruction::RES(PrefixTarget::D,BitPosition::B3)),
            0x9B => Some(Instruction::RES(PrefixTarget::E,BitPosition::B3)),
            0x9C => Some(Instruction::RES(PrefixTarget::H,BitPosition::B3)),
            0x9E => Some(Instruction::RES(PrefixTarget::HLI,BitPosition::B3)),
            0x9F => Some(Instruction::RES(PrefixTarget::A,BitPosition::B3)),
            0xA0 => Some(Instruction::RES(PrefixTarget::B,BitPosition::B4)),
            0xA1 => Some(Instruction::RES(PrefixTarget::C,BitPosition::B4)),
            0xA2 => Some(Instruction::RES(PrefixTarget::D,BitPosition::B4)),
            0xA3 => Some(Instruction::RES(PrefixTarget::E,BitPosition::B4)),
            0xA4 => Some(Instruction::RES(PrefixTarget::H,BitPosition::B4)),
            0xA5 => Some(Instruction::RES(PrefixTarget::L,BitPosition::B4)),
            0xA6 => Some(Instruction::RES(PrefixTarget::HLI,BitPosition::B4)),
            0xA7 => Some(Instruction::RES(PrefixTarget::A,BitPosition::B4)),
            0xA8 => Some(Instruction::RES(PrefixTarget::B,BitPosition::B5)),
            0xA9 => Some(Instruction::RES(PrefixTarget::C,BitPosition::B5)),
            0xAA => Some(Instruction::RES(PrefixTarget::D,BitPosition::B5)),
            0xAB => Some(Instruction::RES(PrefixTarget::E,BitPosition::B5)),
            0xAC => Some(Instruction::RES(PrefixTarget::H,BitPosition::B5)),
            0xAE => Some(Instruction::RES(PrefixTarget::HLI,BitPosition::B5)),
            0xAF => Some(Instruction::RES(PrefixTarget::A,BitPosition::B5)),
            0xB0 => Some(Instruction::RES(PrefixTarget::B,BitPosition::B6)),
            0xB1 => Some(Instruction::RES(PrefixTarget::C,BitPosition::B6)),
            0xB2 => Some(Instruction::RES(PrefixTarget::D,BitPosition::B6)),
            0xB3 => Some(Instruction::RES(PrefixTarget::E,BitPosition::B6)),
            0xB4 => Some(Instruction::RES(PrefixTarget::H,BitPosition::B6)),
            0xB5 => Some(Instruction::RES(PrefixTarget::L,BitPosition::B6)),
            0xB6 => Some(Instruction::RES(PrefixTarget::HLI,BitPosition::B6)),
            0xB7 => Some(Instruction::RES(PrefixTarget::A,BitPosition::B6)),
            0xB8 => Some(Instruction::RES(PrefixTarget::B,BitPosition::B7)),
            0xB9 => Some(Instruction::RES(PrefixTarget::C,BitPosition::B7)),
            0xBA => Some(Instruction::RES(PrefixTarget::D,BitPosition::B7)),
            0xBB => Some(Instruction::RES(PrefixTarget::E,BitPosition::B7)),
            0xBC => Some(Instruction::RES(PrefixTarget::H,BitPosition::B7)),
            0xBE => Some(Instruction::RES(PrefixTarget::HLI,BitPosition::B7)),
            0xBF => Some(Instruction::RES(PrefixTarget::A,BitPosition::B7)),
            0xC0 => Some(Instruction::SET(PrefixTarget::B,BitPosition::B0)),
            0xC1 => Some(Instruction::SET(PrefixTarget::C,BitPosition::B0)),
            0xC2 => Some(Instruction::SET(PrefixTarget::D,BitPosition::B0)),
            0xC3 => Some(Instruction::SET(PrefixTarget::E,BitPosition::B0)),
            0xC4 => Some(Instruction::SET(PrefixTarget::H,BitPosition::B0)),
            0xC5 => Some(Instruction::SET(PrefixTarget::L,BitPosition::B0)),
            0xC6 => Some(Instruction::SET(PrefixTarget::HLI,BitPosition::B0)),
            0xC7 => Some(Instruction::SET(PrefixTarget::A,BitPosition::B0)),
            0xC8 => Some(Instruction::SET(PrefixTarget::B,BitPosition::B1)),
            0xC9 => Some(Instruction::SET(PrefixTarget::C,BitPosition::B1)),
            0xCA => Some(Instruction::SET(PrefixTarget::D,BitPosition::B1)),
            0xCB => Some(Instruction::SET(PrefixTarget::E,BitPosition::B1)),
            0xCC => Some(Instruction::SET(PrefixTarget::H,BitPosition::B1)),
            0xCE => Some(Instruction::SET(PrefixTarget::HLI,BitPosition::B1)),
            0xCF => Some(Instruction::SET(PrefixTarget::A,BitPosition::B1)),
            0xD0 => Some(Instruction::SET(PrefixTarget::B,BitPosition::B2)),
            0xD1 => Some(Instruction::SET(PrefixTarget::C,BitPosition::B2)),
            0xD2 => Some(Instruction::SET(PrefixTarget::D,BitPosition::B2)),
            0xD3 => Some(Instruction::SET(PrefixTarget::E,BitPosition::B2)),
            0xD4 => Some(Instruction::SET(PrefixTarget::H,BitPosition::B2)),
            0xD5 => Some(Instruction::SET(PrefixTarget::L,BitPosition::B2)),
            0xD6 => Some(Instruction::SET(PrefixTarget::HLI,BitPosition::B2)),
            0xD7 => Some(Instruction::SET(PrefixTarget::A,BitPosition::B2)),
            0xD8 => Some(Instruction::SET(PrefixTarget::B,BitPosition::B3)),
            0xD9 => Some(Instruction::SET(PrefixTarget::C,BitPosition::B3)),
            0xDA => Some(Instruction::SET(PrefixTarget::D,BitPosition::B3)),
            0xDB => Some(Instruction::SET(PrefixTarget::E,BitPosition::B3)),
            0xDC => Some(Instruction::SET(PrefixTarget::H,BitPosition::B3)),
            0xDE => Some(Instruction::SET(PrefixTarget::HLI,BitPosition::B3)),
            0xDF => Some(Instruction::SET(PrefixTarget::A,BitPosition::B3)),
            0xE0 => Some(Instruction::SET(PrefixTarget::B,BitPosition::B4)),
            0xE1 => Some(Instruction::SET(PrefixTarget::C,BitPosition::B4)),
            0xE2 => Some(Instruction::SET(PrefixTarget::D,BitPosition::B4)),
            0xE3 => Some(Instruction::SET(PrefixTarget::E,BitPosition::B4)),
            0xE4 => Some(Instruction::SET(PrefixTarget::H,BitPosition::B4)),
            0xE5 => Some(Instruction::SET(PrefixTarget::L,BitPosition::B4)),
            0xE6 => Some(Instruction::SET(PrefixTarget::HLI,BitPosition::B4)),
            0xE7 => Some(Instruction::SET(PrefixTarget::A,BitPosition::B4)),
            0xE8 => Some(Instruction::SET(PrefixTarget::B,BitPosition::B5)),
            0xE9 => Some(Instruction::SET(PrefixTarget::C,BitPosition::B5)),
            0xEA => Some(Instruction::SET(PrefixTarget::D,BitPosition::B5)),
            0xEB => Some(Instruction::SET(PrefixTarget::E,BitPosition::B5)),
            0xEC => Some(Instruction::SET(PrefixTarget::H,BitPosition::B5)),
            0xEE => Some(Instruction::SET(PrefixTarget::HLI,BitPosition::B5)),
            0xEF => Some(Instruction::SET(PrefixTarget::A,BitPosition::B5)),
            0xF0 => Some(Instruction::SET(PrefixTarget::B,BitPosition::B6)),
            0xF1 => Some(Instruction::SET(PrefixTarget::C,BitPosition::B6)),
            0xF2 => Some(Instruction::SET(PrefixTarget::D,BitPosition::B6)),
            0xF3 => Some(Instruction::SET(PrefixTarget::E,BitPosition::B6)),
            0xF4 => Some(Instruction::SET(PrefixTarget::H,BitPosition::B6)),
            0xF5 => Some(Instruction::SET(PrefixTarget::L,BitPosition::B6)),
            0xF6 => Some(Instruction::SET(PrefixTarget::HLI,BitPosition::B6)),
            0xF7 => Some(Instruction::SET(PrefixTarget::A,BitPosition::B6)),
            0xF8 => Some(Instruction::SET(PrefixTarget::B,BitPosition::B7)),
            0xF9 => Some(Instruction::SET(PrefixTarget::C,BitPosition::B7)),
            0xFA => Some(Instruction::SET(PrefixTarget::D,BitPosition::B7)),
            0xFB => Some(Instruction::SET(PrefixTarget::E,BitPosition::B7)),
            0xFC => Some(Instruction::SET(PrefixTarget::H,BitPosition::B7)),
            0xFE => Some(Instruction::SET(PrefixTarget::HLI,BitPosition::B7)),
            0xFF => Some(Instruction::SET(PrefixTarget::A,BitPosition::B7)),
            
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction>
    {
        match byte
        {
            0x00 => Some(Instruction::NOP),
            0x01 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::BC))),//verif
            0x02 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::BC, LoadByteSource::A))),
            0x03 => Some(Instruction::INC(IncDecTarget::BC)),
            0x04 => Some(Instruction::INC(IncDecTarget::B)),
            0x05 => Some(Instruction::DEC(IncDecTarget::B)),
            0x06 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D8))),
            0x07 => Some(Instruction::RLCA),
            0x08 => Some(Instruction::LD(LoadType::IndirectFromSP)),
            0x09 => Some(Instruction::ADDHL(ADDHLTarget::BC)),
            0x0A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::BC))),
            0x0B => Some(Instruction::DEC(IncDecTarget::BC)),
            0x0C => Some(Instruction::INC(IncDecTarget::C)),
            0x0D => Some(Instruction::DEC(IncDecTarget::C)),
            0x0E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D8))),
            0x0F => Some(Instruction::RRCA),
            0x10 => Some(Instruction::STOP),
            0x11 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::DE))),
            0x12 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::DE, LoadByteSource::A))),
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            0x14 => Some(Instruction::INC(IncDecTarget::D)),
            0x15 => Some(Instruction::DEC(IncDecTarget::D)),
            0x16 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D8))),
            0x17 => Some(Instruction::RLA),
            0x18 => Some(Instruction::JR(JumpTest)),//refaire
            0x19 => Some(Instruction::ADDHL(Instruction::ADDHL(ADDHLTarget::DE))),
            0x1A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::DE))),
            0x1B => Some(Instruction::DEC(IncDecTarget::DE)),
            0x1C => Some(Instruction::INC(IncDecTarget::E)),
            0x1D => Some(Instruction::DEC(IncDecTarget::E)),
            0x1E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D8))),
            0x1F => Some(Instruction::RRA),
            0x20 => Some(Instruction::JR(JumpTest::NZ)),
            0x21 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::HL))),
            0x22 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::A))),
            0x23 => Some(Instruction::INC(IncDecTarget::HL)),
            0x24 => Some(Instruction::INC(IncDecTarget::H)),
            0x25 => Some(Instruction::DEC(IncDecTarget::H)),
            0x26 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D8))),
            0x27 => Some(Instruction::DAA),
            0x28 => Some(Instruction::JR(JumpTest::Z)),//refaire
            0x29 => Some(Instruction::ADDHL(Instruction::ADDHL(ADDHLTarget::HL))),
            0x2A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::HLI))),
            0x2B => Some(Instruction::DEC(IncDecTarget::HL)),
            0x2C => Some(Instruction::INC(IncDecTarget::L)),
            0x2D => Some(Instruction::DEC(IncDecTarget::L)),
            0x2E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D8))),
            0x2F => Some(Instruction::CPL),
            0x30 => Some(Instruction::JR(JumpTest::NC)),
            0x31 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::SP))),
            0x32 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLD, LoadByteSource::A))),
            0x33 => Some(Instruction::INC(IncDecTarget::SP)),
            0x34 => Some(Instruction::INC(IncDecTarget::HL)),
            0x35 => Some(Instruction::DEC(IncDecTarget::HL)),
            0x36 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::D8))),
            0x37 => Some(Instruction::SCF),
            0x38 => Some(Instruction::JR(JumpTest::C)),
            0x39 => Some(Instruction::ADDHL(Instruction::ADDHL(ADDHLTarget::SP))),
            0x3A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::HLD))),
            0x3B => Some(Instruction::DEC(IncDecTarget::SP)),
            0x3C => Some(Instruction::INC(IncDecTarget::A)),
            0x3D => Some(Instruction::DEC(IncDecTarget::A)),
            0x3E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D8))),
            0x3F => Some(Instruction::CCF),
            0x40 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::B))),
            0x41 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::C))),
            0x42 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D))),
            0x43 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::E))),
            0x44 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::H))),
            0x45 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::L))),
            0x46 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::HL))),
            0x47 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::A))),
            0x48 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::B))),
            0x49 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::C))),
            0x4A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D))),
            0x4B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::E))),
            0x4C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::H))),
            0x4D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::L))),
            0x4E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::HL))),
            0x4F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::A))),
            0x50 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::B))),
            0x51 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::C))),
            0x52 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D))),
            0x53 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::E))),
            0x54 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::H))),
            0x55 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::L))),
            0x56 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::HL))),
            0x57 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::A))),
            0x58 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::B))),
            0x59 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::C))),
            0x5A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D))),
            0x5B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::E))),
            0x5C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::H))),
            0x5D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::L))),
            0x5E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::HL))),
            0x5F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::A))),
            0x60 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::B))),
            0x61 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::C))),
            0x62 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D))),
            0x63 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::E))),
            0x64 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::H))),
            0x65 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::L))),
            0x66 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::HL))),
            0x67 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::A))),
            0x68 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::B))),
            0x69 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::C))),
            0x6A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D))),
            0x6B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::E))),
            0x6C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::H))),
            0x6D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::L))),
            0x6E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::HL))),
            0x6F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::A))),
            0x70 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::B))),
            0x71 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::C))),
            0x72 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::D))),
            0x73 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::E))),
            0x74 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::H))),
            0x75 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::L))),
            0x76 => Some(Instruction::HALT),
            0x77 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::A))),
            0x78 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::B))),
            0x79 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::C))),
            0x7A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D))),
            0x7B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::E))),
            0x7C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::H))),
            0x7D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::L))),
            0x7E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::HL))),
            0x7F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::A))),
            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),
            0x86 => Some(Instruction::ADD(ArithmeticTarget::HL)),
            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),
            0x88 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x89 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x8A => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x8B => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x8C => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x8D => Some(Instruction::ADD(ArithmeticTarget::L)),
            0x8E => Some(Instruction::ADD(ArithmeticTarget::HL)),
            0x8F => Some(Instruction::ADD(ArithmeticTarget::A)),
            0x90 => Some(Instruction::SUB(ArithmeticTarget::B)),
            0x91 => Some(Instruction::SUB(ArithmeticTarget::C)),
            0x92 => Some(Instruction::SUB(ArithmeticTarget::D)),
            0x93 => Some(Instruction::SUB(ArithmeticTarget::E)),
            0x94 => Some(Instruction::SUB(ArithmeticTarget::H)),
            0x95 => Some(Instruction::SUB(ArithmeticTarget::L)),
            0x96 => Some(Instruction::SUB(ArithmeticTarget::HL)),
            0x97 => Some(Instruction::SUB(ArithmeticTarget::A)),
            0x98 => Some(Instruction::SBC(ArithmeticTarget::B)),
            0x99 => Some(Instruction::SBC(ArithmeticTarget::C)),
            0x9A => Some(Instruction::SBC(ArithmeticTarget::D)),
            0x9B => Some(Instruction::SBC(ArithmeticTarget::E)),
            0x9C => Some(Instruction::SBC(ArithmeticTarget::H)),
            0x9D => Some(Instruction::SBC(ArithmeticTarget::L)),
            0x9E => Some(Instruction::SBC(ArithmeticTarget::HL)),
            0x9F => Some(Instruction::SBC(ArithmeticTarget::A)),
            0xA0 => Some(Instruction::AND(ArithmeticTarget::B)),
            0xA1 => Some(Instruction::AND(ArithmeticTarget::C)),
            0xA2 => Some(Instruction::AND(ArithmeticTarget::D)),
            0xA3 => Some(Instruction::AND(ArithmeticTarget::E)),
            0xA4 => Some(Instruction::AND(ArithmeticTarget::H)),
            0xA5 => Some(Instruction::AND(ArithmeticTarget::L)),
            0xA6 => Some(Instruction::AND(ArithmeticTarget::HL)),
            0xA7 => Some(Instruction::AND(ArithmeticTarget::A)),
            0xA8 => Some(Instruction::XOR(ArithmeticTarget::B)),
            0xA9 => Some(Instruction::XOR(ArithmeticTarget::C)),
            0xAA => Some(Instruction::XOR(ArithmeticTarget::D)),
            0xAB => Some(Instruction::XOR(ArithmeticTarget::E)),
            0xAC => Some(Instruction::XOR(ArithmeticTarget::H)),
            0xAD => Some(Instruction::XOR(ArithmeticTarget::L)),
            0xAE => Some(Instruction::XOR(ArithmeticTarget::HL)),
            0xAF => Some(Instruction::XOR(ArithmeticTarget::A)),
            0xB0 => Some(Instruction::OR(ArithmeticTarget::B)),
            0xB1 => Some(Instruction::OR(ArithmeticTarget::C)),
            0xB2 => Some(Instruction::OR(ArithmeticTarget::D)),
            0xB3 => Some(Instruction::OR(ArithmeticTarget::E)),
            0xB4 => Some(Instruction::OR(ArithmeticTarget::H)),
            0xB5 => Some(Instruction::OR(ArithmeticTarget::L)),
            0xB6 => Some(Instruction::OR(ArithmeticTarget::HL)),
            0xB7 => Some(Instruction::OR(ArithmeticTarget::A)),
            0xB8 => Some(Instruction::CP(ArithmeticTarget::B)),
            0xB9 => Some(Instruction::CP(ArithmeticTarget::C)),
            0xBA => Some(Instruction::CP(ArithmeticTarget::D)),
            0xBB => Some(Instruction::CP(ArithmeticTarget::E)),
            0xBC => Some(Instruction::CP(ArithmeticTarget::H)),
            0xBD => Some(Instruction::CP(ArithmeticTarget::L)),
            0xBE => Some(Instruction::CP(ArithmeticTarget::HL)),
            0xBF => Some(Instruction::CP(ArithmeticTarget::A)),
            0xC0 => Some(Instruction::RET(JumpTest::NZ)),
            0xC1 => Some(Instruction::POP(StackTarget::BC)),
            0xC2 => Some(Instruction::JP(JumpTest::NZ)),
            0xC3 => Some(Instruction::JP(JumpTest::E)),
            0xC4 => Some(Instruction::CALL(JumpTest::NZ)),
            0xC5 => Some(Instruction::PUSH(StackTarget::BC)),
            0xC6 => Some(Instruction::ADD(ArithmeticTarget::D8)),
            0xC7 => Some(Instruction::RST(RSTPosition::X00)),
            0xC8 => Some(Instruction::RET(JumpTest::Z)),
            0xC9 => Some(Instruction::RET(JumpTest)),// revoir
            0xCA => Some(Instruction::JP(JumpTest::Z)),

            0xCB => Some(Instruction::from_byte_prefixed(byte)),

            0xCB => Some(Instruction::from_byte_prefixed(byte)),// pas d'idée

            0xCC => Some(Instruction::CALL(JumpTest::Z)),
            0xCD => Some(Instruction::CALL(ArithmeticTarget::A)),// revoir
            0xCE => Some(Instruction::ADC(ArithmeticTarget::D8)),
            0xCF => Some(Instruction::RST(RSTPosition::X08)),
            0xD0 => Some(Instruction::RET(JumpTest::NC)),
            0xD1 => Some(Instruction::POP(StackTarget::DE)),
            0xD2 => Some(Instruction::JP(JumpTest::NC)),
            //0xD3 => Some(Instruction::JP(JumpTest::E)),
            0xD4 => Some(Instruction::CALL(JumpTest::NC)),
            0xD5 => Some(Instruction::PUSH(StackTarget::DE)),
            0xD6 => Some(Instruction::SUB(ArithmeticTarget::D8)),
            0xD7 => Some(Instruction::RST(RSTPosition::X10)),
            0xD8 => Some(Instruction::RET(JumpTest::C)),
            0xD9 => Some(Instruction::RETI),
            0xDA => Some(Instruction::JP(JumpTest::C)),
            //0xDB => Some(Instruction::JP(JumpTest::Z)),// pas d'idée
            0xDC => Some(Instruction::CALL(JumpTest::C)),
            //0xDD => Some(Instruction::CALL(JumpTest)),// revoir
            0xDE => Some(Instruction::SBC(ArithmeticTarget::D8)),
            0xDF => Some(Instruction::RST(RSTPosition::X18)),
            0xE0 => Some(Instruction::LDH(sourceA)),
            0xE1 => Some(Instruction::POP(StackTarget::HL)),
            0xE2 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::CC, LoadByteSource::A))),
            //0xD3 => Some(Instruction::JP(JumpTest::E)),
            //0xE4 => Some(Instruction::CALL(JumpTest::NC)),
            0xE5 => Some(Instruction::PUSH(StackTarget::HL)),
            0xE6 => Some(Instruction::AND(ArithmeticTarget::D8)),
            0xE7 => Some(Instruction::RST(RSTPosition::X20)),
            0xE8 => Some(Instruction::ADDSP),

            0xE9 => Some(Instruction::JPHL),

            0xEA => Some(Instruction::LD(JumpTest::C)),// pas d'idée
            //0xDB => Some(Instruction::JP(JumpTest::Z)),// pas d'idée
            //0xEC => Some(Instruction::CALL(JumpTest::C)),
            //0xDD => Some(Instruction::CALL(JumpTest)),// revoir
            0xEE => Some(Instruction::XOR(ArithmeticTarget::D8)),
            0xEF => Some(Instruction::RST(RSTPosition::X28)),
            0xF0 => Some(Instruction::LDH(targetA)),
            0xF1 => Some(Instruction::POP(StackTarget::AF)),
            0xF2 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::C))),

            0xF3 => Some(Instruction::DI),

            //0xE4 => Some(Instruction::CALL(JumpTest::NC)),
            0xF5 => Some(Instruction::PUSH(StackTarget::AF)),
            0xF6 => Some(Instruction::OR(ArithmeticTarget::D8)),
            0xF7 => Some(Instruction::RST(RSTPosition::X30)),
            0xF8 => Some(Instruction::LD(LoadType::HLFromSPN)),// revoir
            0xF9 => Some(Instruction::LD(LoadType::SPFromHL)),
            0xFA => Some(Instruction::LD(LoadType::Word(LoadWordTarget::A))),// pas d'idée

            0xFB => Some(Instruction::EI),

            //0xFC => Some(Instruction::CALL(JumpTest::C)),
            //0xDD => Some(Instruction::CALL(JumpTest)),// revoir
            0xFE => Some(Instruction::CP(ArithmeticTarget::D8)),
            0xFF => Some(Instruction::RST(RSTPosition::X38)),
           
            
        }
    }
}
