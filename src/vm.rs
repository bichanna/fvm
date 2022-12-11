pub struct VM {
    registers: [i32; 32],
}

impl VM {
    pub fn new() -> VM {
        return VM { registers: [0; 32] };
    }
}

// Tests
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[16], 0);
    }
}
