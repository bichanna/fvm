#[derive(Debug, PartialEq)]
pub enum Opcode {
    HLT, // halt
    IGL, // illegal
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,  // absolute jump
    JMPF, // relative forward jump
    JMPB, // relative backward jump
    EQ,
    JEQ,  // jump if equal
    JNEQ, // jump if not equal
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Self {
        Instruction { opcode }
    }
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::LOAD,
            1 => Opcode::ADD,
            2 => Opcode::SUB,
            3 => Opcode::MUL,
            4 => Opcode::DIV,
            5 => Opcode::HLT,
            6 => Opcode::JMP,
            7 => Opcode::JMPF,
            8 => Opcode::JMPB,
            9 => Opcode::EQ,
            10 => Opcode::JEQ,
            11 => Opcode::JNEQ,
            _ => Opcode::IGL,
        }
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}
