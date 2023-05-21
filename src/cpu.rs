#[derive(Debug, PartialEq)]
enum Exception {
    Reset,
    BusErrorInstruction,
    BusErrorData,
    AddressErrorLoad,
    AddressErrorStore,
    Overflow,
    SystemCall,
    Breakpoint,
    ReservedInstructio,
    CoprocessorUnusable,
    Interrupt,
    Debug,
}

enum Register {
    Zero,
    Normal(u32),
}

impl Register {
    fn read(&self) -> u32 {
        match self {
            Register::Zero => 0,
            Register::Normal(value) => *value,
        }
    }
    
    fn write(&mut self, value: u32) {
        match self {
            Register::Zero => (),
            Register::Normal(old) => *old = value,
        }
    }
}

struct Cpu {
    register_file: [Register; 32],
    pc: u32,
}

impl Cpu {
    fn new() -> Self {
        let register_file: [Register; 32] = [
            Register::Zero,
            Register::Normal(0),
            Register::Normal(0),
            Register::Normal(0),
            Register::Normal(0),
            Register::Normal(0),
            Register::Normal(0),
            Register::Normal(0),
            Register::Normal(0),
            Register::Normal(0),
            Register::Normal(0),
            Register::Normal(0),
            Register::Normal(0),
            Register::Normal(0),
            Register::Normal(0),
            Register::Normal(0),
            Register::Normal(0),
            Register::Normal(0),
            Register::Normal(0),
            Register::Normal(0),
            Register::Normal(0),
            Register::Normal(0),
            Register::Normal(0),
            Register::Normal(0),
            Register::Normal(0),
            Register::Normal(0),
            Register::Normal(0),
            Register::Normal(0),
            Register::Normal(0),
            Register::Normal(0),
            Register::Normal(0),
            Register::Normal(0),
        ];
        
        Cpu {
            pc: 0xbfc00000,
            register_file,
        }
    }

    fn add(&mut self, rs: u8, rt: u8, rd: u8) -> Option<Exception> {
        let a = self.register_file[rs as usize].read();
        let b = self.register_file[rt as usize].read();

        // Overflow is detected for two.complements
        let (res, overflow) = (a as i32).overflowing_add(b as i32);

        if overflow {
            Some(Exception::Overflow)
        } else {
            self.register_file[rd as usize].write(res as u32);
            None
        }
    }

    fn addi(&mut self, rs: u8, rt: u8, immediate: u16) -> Option<Exception> {
        let a = self.register_file[rs as usize].read();

        // sign extend the immediate to 32 bits
        let (res, overflow) = (a as i32).overflowing_add(immediate as i16 as i32);

        if overflow {
            Some(Exception::Overflow)
        } else {
            self.register_file[rt as usize].write(res as u32);
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(60);
        cpu.register_file[2].write(9);

        assert_eq!(cpu.add(1, 2, 3), None);
        assert_eq!(cpu.register_file[3].read(), 69);
    }

    #[test]
    fn add_with_overflow() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(0x80000000);
        cpu.register_file[2].write(0x80000000);
        cpu.register_file[3].write(12);

        assert_eq!(cpu.add(1, 2, 3), Some(Exception::Overflow));
        assert_eq!(cpu.register_file[3].read(), 12);
    }

    #[test]
    fn addi() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(60);

        assert_eq!(cpu.addi(1, 2, 9), None);
        assert_eq!(cpu.register_file[2].read(), 69);
    }

    #[test]
    fn addi_with_overflow() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(0x80000000);
        cpu.register_file[2].write(12);

        assert_eq!(cpu.addi(1, 2, 0xffff), Some(Exception::Overflow));
        assert_eq!(cpu.register_file[2].read(), 12);
    }
}
