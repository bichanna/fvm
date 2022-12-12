use crate::instruction::Opcode;

pub struct VM {
    pub registers: [i32; 32],
    pc: usize,
    pub program: Vec<u8>,
}

impl VM {
    pub fn new() -> VM {
        return VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
        };
    }

    pub fn run(&mut self) {
        while self.pc < self.program.len() {
            match self.decode_opcode() {
                Opcode::HLT => {
                    println!("HLT encountered!!");
                    return;
                }
                Opcode::LOAD => {
                    let register = usize::from(self.next_8_bits());
                    let number = u16::from(self.next_16_bits());
                    self.registers[register] = i32::from(number);
                    continue;
                }
                _ => {
                    println!("Unrecognized opcode!");
                    return;
                }
            }
        }
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        println!("{:?}", result);
        return result;
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = (u16::from(self.program[self.pc]) << 8) | u16::from(self.program[self.pc + 1]);
        self.pc += 2;
        println!("{:?}", result);
        return result;
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[16], 0);
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![5, 0, 0, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_load() {
        let mut test_vm = VM::new();
        test_vm.program = vec![0, 0, 1, 244];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }
}
