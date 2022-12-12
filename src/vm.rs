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

    // Loops through the instructions as long as instructions can be executed.
    pub fn run(&mut self) {
        let mut done = false;
        while !done {
            done = self.execute_instruction();
        }
    }

    // Executes only one single instruction.
    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    // Executes single instruction and returns true if no instructions can be executed.
    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }
        match self.decode_opcode() {
            Opcode::HLT => true,
            Opcode::LOAD => {
                let register = usize::from(self.next_8_bits());
                let number = u16::from(self.next_16_bits());
                self.registers[register] = i32::from(number);
                false
            }
            Opcode::ADD => {
                let register1 = self.registers[usize::from(self.next_8_bits())];
                let register2 = self.registers[usize::from(self.next_8_bits())];
                self.registers[usize::from(self.next_8_bits())] = register1 + register2;
                false
            }
            Opcode::IGL => true,
        }
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }

    // Returns the next 8 bits.
    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        return result;
    }

    // Returns the next 16 bits.
    fn next_16_bits(&mut self) -> u16 {
        let result = (u16::from(self.program[self.pc]) << 8) | u16::from(self.program[self.pc + 1]);
        self.pc += 2;
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
        // Load 500 to register 0.
        test_vm.program = vec![0, 0, 1, 244];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_opcode_add() {
        let mut test_vm = VM::new();
        // Load 500 to register 1, load 500 to register 1, add register 1 and 2, and store the
        // result to register 0.
        test_vm.program = vec![0, 1, 1, 244, 0, 2, 1, 244, 1, 1, 2, 0];
        test_vm.run();
        println!("{:?}", test_vm.registers);
        assert_eq!(test_vm.registers[0], 1000)
    }
}
