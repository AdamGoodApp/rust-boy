
// The Game Boy's CPU is an 8-Bit CPU, meaning that each of its registers 
// can hold 8 bits (a.k.a 1 byte) of data.
struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

impl Registers {
    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8
        | self.c as u16
    }

    fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool
}

impl std::convert::From<FlagsRegister> for u8  {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}

// ADD Instruction
enum Instruction {
    ADD(ArithmeticTarget),
}
  
  enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}

struct CPU {}

impl CPU {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
        Instruction::ADD(target) => {
            match target {
            ArithmeticTarget::C => {
                let value = self.registers.c;
                let new_value = self.add(value);
                self.registers.a = new_value;
            }
            _ => { /* TODO: support more targets */ }
            }
        }
        _ => { /* TODO: support more instructions */ }
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        // TODO: set flags
        new_value
    }
}

  

fn main() {
    println!("Hello, world!");
}
