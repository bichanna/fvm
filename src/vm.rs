use crate::instruction::Opcode;

pub struct VM {
    /// Array that simulates having hardware registers
    registers: [i32; 32],
    /// Heap for the VM
    heap: Vec<u8>,
    /// Program counter that tracks which byte is being executed
    pc: usize,
    /// The bytecodes of the program being executed
    program: Vec<u8>,
    /// Contains
    remainder: u32,
    /// Contains the result of the last comparison operation
    equal_flag: bool,
}

impl VM {
    pub fn new() -> Self {
        VM {
            registers: [0; 32],
            heap: vec![],
            pc: 0,
            program: vec![],
            remainder: 0,
            equal_flag: false,
        }
    }

    pub fn get_program(&mut self) -> &Vec<u8> {
        &self.program
    }

    pub fn get_registers(&mut self) -> [i32; 32] {
        self.registers
    }

    pub fn add_byte(&mut self, b: u8) {
        self.program.push(b);
    }

    /// Loops through the instructions as long as instructions can be executed.
    pub fn run(&mut self) {
        let mut done = false;
        while !done {
            done = self.execute_instruction();
        }
    }

    /// Executes only one single instruction.
    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    /// Executes single instruction and returns true if no instructions can be executed.
    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }
        match self.decode_opcode() {
            // format: LOAD [0] [number] [number]
            // Load [number] to register [0]
            Opcode::LOAD => {
                let register = usize::from(self.next_8_bits());
                let number = u16::from(self.next_16_bits());
                self.registers[register] = i32::from(number);
                false
            }
            // format: ADD [0] [1] [2]
            // Add [0] and [1], and then store the result to register [2].
            Opcode::ADD => {
                let register1 = self.registers[usize::from(self.next_8_bits())];
                let register2 = self.registers[usize::from(self.next_8_bits())];
                self.registers[usize::from(self.next_8_bits())] = register1 + register2;
                false
            }
            // format: SUB [0] [1] [2]
            // Subtract [1] from [0], and then store the result to register [2].
            Opcode::SUB => {
                let register1 = self.registers[usize::from(self.next_8_bits())];
                let register2 = self.registers[usize::from(self.next_8_bits())];
                self.registers[usize::from(self.next_8_bits())] = register1 - register2;
                false
            }
            // format: MUL [0] [1] [2]
            // Multiply [0] by [1], and then store the result to register [2].
            Opcode::MUL => {
                let register1 = self.registers[usize::from(self.next_8_bits())];
                let register2 = self.registers[usize::from(self.next_8_bits())];
                self.registers[usize::from(self.next_8_bits())] = register1 * register2;
                false
            }
            // format: DIV [0] [1] [2]
            // Divide [0] by [1], and then store the result to register [2], and the remainder is
            // stored in `remainder`.
            Opcode::DIV => {
                let register1 = self.registers[usize::from(self.next_8_bits())];
                let register2 = self.registers[usize::from(self.next_8_bits())];
                self.registers[usize::from(self.next_8_bits())] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
                false
            }
            // format: JMP [0]
            // Set the counter to the value of register[0].
            Opcode::JMP => {
                let target = self.registers[usize::from(self.next_8_bits())];
                self.pc = target as usize;
                false
            }
            // format: JMPF [0]
            // Add the value of register [0] to the counter. (relative forward jump)
            Opcode::JMPF => {
                let value = self.registers[usize::from(self.next_8_bits())];
                self.pc += value as usize;
                false
            }
            // format: JMPB [0]
            // Subtract the value of register [0] from the counter. (relative backward jump)
            Opcode::JMPB => {
                let value = self.registers[usize::from(self.next_8_bits())];
                self.pc -= value as usize;
                false
            }
            // format: EQ [0] [1]
            // Checks if the values of register [0] and register [1] are equal, and stores the
            // result to `equal_flag`.
            Opcode::EQ => {
                let register1 = self.registers[usize::from(self.next_8_bits())];
                let register2 = self.registers[usize::from(self.next_8_bits())];
                if register1 == register2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
                self.next_8_bits();
                false
            }
            // format: JEQ [0]
            // If `equal_flag` is true, set the counter to the value of register [0].
            Opcode::JEQ => {
                let target = self.registers[usize::from(self.next_8_bits())];
                if self.equal_flag {
                    self.pc = target as usize;
                }
                false
            }
            // format: JNEQ [0]
            // If `equal_flag` is not true, set the counter to the value of register [0].
            Opcode::JNEQ => {
                let target = self.registers[usize::from(self.next_8_bits())];
                if !self.equal_flag {
                    self.pc = target as usize;
                }
                false
            }
            // format: ALOC [0]
            // Extends the size of the heap vector by the amount in the register [0].
            Opcode::ALOC => {
                let register = usize::from(self.next_8_bits());
                let bytes = self.registers[register];
                let new_end = self.heap.len() as i32 + bytes;
                self.heap.resize(new_end as usize, 0);
                false
            }
            // format: INC [0]
            // Increments the value stored in register [0] by 1.
            Opcode::INC => {
                let register = usize::from(self.next_8_bits());
                self.registers[register] += 1;
                false
            }
            // format: DEC [0]
            // Decrements the value stored in register [0] by 1.
            Opcode::DEC => {
                let register = usize::from(self.next_8_bits());
                self.registers[register] -= 1;
                false
            }
            Opcode::IGL => true,
            Opcode::HLT => true,
        }
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }

    /// Returns the next 8 bits.
    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        return result;
    }

    /// Returns the next 16 bits.
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
        // Load 500 to register 0 and load 250 to register 1.
        test_vm.program = vec![0, 0, 1, 244, 0, 1, 0, 250];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
        assert_eq!(test_vm.registers[1], 250);
    }

    #[test]
    fn test_opcode_add() {
        let mut test_vm = VM::new();
        // Load 500 to register 1, load 500 to register 2, add register 1 and 2, and store the
        // result to register 0.
        test_vm.program = vec![0, 1, 1, 244, 0, 2, 1, 244, 1, 1, 2, 0];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 1000);
    }

    #[test]
    fn test_opcode_sub() {
        let mut test_vm = VM::new();
        // Load 500 to register 1, load 500 to register 1, subtract register 2 from register 1 and
        // store the result to register 0.
        test_vm.program = vec![0, 1, 1, 244, 0, 2, 1, 244, 2, 1, 2, 0];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 0);
    }

    #[test]
    fn test_opcode_mul() {
        let mut test_vm = VM::new();
        // Load 500 to register 1, load 500 to register 2, multiply register 1 by register 2 and store
        // the result to register 0.
        test_vm.program = vec![0, 1, 1, 244, 0, 2, 1, 244, 3, 1, 2, 0];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 250000);
    }

    #[test]
    fn test_opcode_div() {
        let mut test_vm = VM::new();
        // Load 100 to register 1, load 3 to register 2, divide register 1 by register 2 and store
        // the result to register 0.
        test_vm.program = vec![0, 1, 0, 100, 0, 2, 0, 3, 4, 1, 2, 0];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 33);
        assert_eq!(test_vm.remainder, 1);
    }

    #[test]
    fn test_opcode_jmp() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 1;
        test_vm.program = vec![6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_jmpf() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.program = vec![7, 0, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_opcode_jmpb() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 3;
        test_vm.pc = 1;
        test_vm.program = vec![0, 8, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 0);
    }

    #[test]
    fn test_opcode_eq() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![9, 0, 1, 0, 9, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_opcode_jeq() {
        let mut test_vm = VM::new();
        test_vm.equal_flag = true;
        // Load 7 to register 0, and set the counter the value of the register 0 if `equal_flag` is
        // true.
        test_vm.program = vec![0, 0, 0, 7, 10, 0, 0];
        test_vm.run();
        assert_eq!(test_vm.pc, 7)
    }

    #[test]
    fn test_opcode_aloc() {
        let mut test_vm = VM::new();
        // Load 5 to register 0, and allocate the value stored in register 5 to the heap.
        test_vm.program = vec![0, 0, 0, 5, 12, 0];
        test_vm.run();
        assert_eq!(test_vm.heap.len(), 5);
    }

    #[test]
    fn test_opcode_inc() {
        let mut test_vm = VM::new();
        // Load 5 to register 0, and increments the value stored in register 0 by 1.
        test_vm.program = vec![0, 0, 0, 5, 13, 0];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 6);
    }

    #[test]
    fn test_opcode_dec() {
        let mut test_vm = VM::new();
        // Load 5 to register 0, and decrements the value stored in register 0 by 1.
        test_vm.program = vec![0, 0, 0, 5, 14, 0];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 4);
    }
}
