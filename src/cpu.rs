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

pub struct Cpu {
    register_file: [Register; 32],
    hi: u32,
    lo: u32,
    pc: u32,
}

impl Cpu {
    pub fn new() -> Self {
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
            register_file,
            hi: 0,
            lo: 0,
            pc: 0xbfc00000,
        }
    }

    fn execute_instruction(&mut self) {
        todo!()
    }

    fn add(&mut self, rs: u8, rt: u8, rd: u8) -> Option<Exception> {
        let a = self.register_file[rs as usize].read();
        let b = self.register_file[rt as usize].read();

        // Overflow is detected for two's complement
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

    fn addiu(&mut self, rs: u8, rt: u8, immediate: u16) {
        let a = self.register_file[rs as usize].read();
        // Sign extend immediate
        let b = immediate as i16 as i32 as u32;

        let res = a.wrapping_add(b);

        self.register_file[rt as usize].write(res);
    }

    fn addu(&mut self, rs: u8, rt: u8, rd: u8) {
        let a = self.register_file[rs as usize].read();
        let b = self.register_file[rt as usize].read();

        let res = a.wrapping_add(b);

        self.register_file[rd as usize].write(res);
    }

    fn and(&mut self, rs: u8, rt: u8, rd: u8) {
        let a = self.register_file[rs as usize].read();
        let b = self.register_file[rt as usize].read();

        self.register_file[rd as usize].write(a & b);
    }

    fn andi(&mut self, rs: u8, rt: u8, immediate: u16) {
        let a = self.register_file[rs as usize].read();
        let b = immediate as u32;

        self.register_file[rt as usize].write(a & b);
    }

    fn bc0f(&mut self, offset: u16) {
        todo!()
    }

    fn bc0t(&mut self, offset: u16) {
        todo!()
    }

    fn bc1f(&mut self, offset: u16) {
        todo!()
    }

    fn bc1t(&mut self, offset: u16) {
        todo!()
    }

    fn bc2f(&mut self, offset: u16) {
        todo!()
    }

    fn bc2t(&mut self, offset: u16) {
        todo!()
    }

    fn bc3f(&mut self, offset: u16) {
        todo!()
    }

    fn bc3t(&mut self, offset: u16) {
        todo!()
    }

    fn beq(&mut self, rs: u8, rt: u8, offset: u16) {
        if self.register_file[rs as usize].read() == self.register_file[rt as usize].read() {
            let target = (offset as i16 as i32) << 2;
            self.pc = self.pc.wrapping_add_signed(target);
        }
    }

    fn bgez(&mut self, rs: u8, offset: u16) {
        if self.register_file[rs as usize].read() as i32 >= 0 {
            let target = (offset as i16 as i32) << 2;
            self.pc = self.pc.wrapping_add_signed(target);
        }
    }

    fn bgezal(&mut self, rs: u8, offset: u16) {
        // link register, r31 is loaded with the address of the instruction after the delay slot
        self.register_file[31].write(self.pc + 4);
        if self.register_file[rs as usize].read() as i32 >= 0 {
            let target = (offset as i16 as i32) << 2;
            self.pc = self.pc.wrapping_add_signed(target);
        }
    }

    fn bgtz(&mut self, rs: u8, offset: u16) {
        if self.register_file[rs as usize].read() as i32 > 0 {
            let target = (offset as i16 as i32) << 2;
            self.pc = self.pc.wrapping_add_signed(target);
        }
    }

    fn blez(&mut self, rs: u8, offset: u16) {
        if self.register_file[rs as usize].read() as i32 <= 0 {
            let target = (offset as i16 as i32) << 2;
            self.pc = self.pc.wrapping_add_signed(target);
        }
    }

    fn bltz(&mut self, rs: u8, offset: u16) {
        if (self.register_file[rs as usize].read() as i32) < 0 {
            let target = (offset as i16 as i32) << 2;
            self.pc = self.pc.wrapping_add_signed(target);
        }
    }

    fn bltzal(&mut self, rs: u8, offset: u16) {
        // link register, r31 is loaded with the address of the instruction after the delay slot
        self.register_file[31].write(self.pc + 4);
        if (self.register_file[rs as usize].read() as i32) < 0 {
            let target = (offset as i16 as i32) << 2;
            self.pc = self.pc.wrapping_add_signed(target);
        }
    }

    fn bne(&mut self, rs: u8, rt: u8, offset: u16) {
        if self.register_file[rs as usize].read() != self.register_file[rt as usize].read() {
            let target = (offset as i16 as i32) << 2;
            self.pc = self.pc.wrapping_add_signed(target);
        }
    }

    fn r#break(&self) -> Exception {
        Exception::Breakpoint
    }

    fn div(&mut self, rs: u8, rt: u8) {
        let a = self.register_file[rs as usize].read() as i32;
        let b = self.register_file[rt as usize].read() as i32;

        self.lo = a.checked_div(b).unwrap_or(0) as u32;
        self.hi = a.checked_rem(b).unwrap_or(0) as u32;
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

    #[test]
    fn addiu() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(0xffffffff);

        cpu.addiu(1, 2, 1);
        assert_eq!(cpu.register_file[2].read(), 0);
    }

    #[test]
    fn addu() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(0x80000000);
        cpu.register_file[2].write(0x80000000);

        cpu.addu(1, 2, 3);
        assert_eq!(cpu.register_file[3].read(), 0);
    }

    #[test]
    fn and() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(0xff00ff00);
        cpu.register_file[2].write(0x00ff00ff);

        cpu.and(1, 2, 3);
        assert_eq!(cpu.register_file[3].read(), 0x00000000);
    }

    #[test]
    fn andi() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(0xff00ff00);

        cpu.andi(1, 2, 0xff00);
        assert_eq!(cpu.register_file[2].read(), 0x0000ff00);
    }

    #[test]
    fn beq_branch_taken() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(1);
        cpu.register_file[2].write(1);
        cpu.pc = 68;

        cpu.beq(1, 2, -10i16 as u16);
        assert_eq!(cpu.pc, 28);
    }

    #[test]
    fn beq_branch_not_taken() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(1);
        cpu.register_file[2].write(2);
        cpu.pc = 68;

        cpu.beq(1, 2, -10i16 as u16);
        assert_eq!(cpu.pc, 68);
    }

    #[test]
    fn bgez_branch_taken() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(20);
        cpu.pc = 16;

        cpu.bgez(1, 25);
        assert_eq!(cpu.pc, 116);
    }

    #[test]
    fn bgez_branch_not_taken() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(-20i32 as u32);
        cpu.pc = 16;

        cpu.bgez(1, 25);
        assert_eq!(cpu.pc, 16);
    }

    #[test]
    fn bgezal_branch_taken() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(20);
        cpu.pc = 16;

        cpu.bgezal(1, 25);
        assert_eq!(cpu.pc, 116);
        assert_eq!(cpu.register_file[31].read(), 20);
    }

    #[test]
    fn bgezal_branch_not_taken() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(-20i32 as u32);
        cpu.pc = 16;

        cpu.bgezal(1, 25);
        assert_eq!(cpu.pc, 16);
        assert_eq!(cpu.register_file[31].read(), 20);
    }

    #[test]
    fn bgtz_branch_taken() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(20);
        cpu.pc = 16;

        cpu.bgtz(1, 25);
        assert_eq!(cpu.pc, 116);
    }

    #[test]
    fn bgtz_branch_not_taken() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(0);
        cpu.pc = 16;

        cpu.bgtz(1, 25);
        assert_eq!(cpu.pc, 16);
    }

    #[test]
    fn blez_branch_taken() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(-10i16 as u32);
        cpu.pc = 16;

        cpu.blez(1, 25);
        assert_eq!(cpu.pc, 116);
    }

    #[test]
    fn blez_branch_not_taken() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(20);
        cpu.pc = 16;

        cpu.blez(1, 25);
        assert_eq!(cpu.pc, 16);
    }

    #[test]
    fn bltz_branch_taken() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(-10i16 as u32);
        cpu.pc = 16;

        cpu.bltz(1, 25);
        assert_eq!(cpu.pc, 116);
    }

    #[test]
    fn bltz_branch_not_taken() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(0);
        cpu.pc = 16;

        cpu.bltz(1, 25);
        assert_eq!(cpu.pc, 16);
    }

    #[test]
    fn bltzal_branch_taken() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(-10i16 as u32);
        cpu.pc = 16;

        cpu.bltzal(1, 25);
        assert_eq!(cpu.pc, 116);
        assert_eq!(cpu.register_file[31].read(), 20);
    }

    #[test]
    fn bltzal_branch_not_taken() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(0);
        cpu.pc = 16;

        cpu.bltzal(1, 25);
        assert_eq!(cpu.pc, 16);
        assert_eq!(cpu.register_file[31].read(), 20);
    }

    #[test]
    fn bne_branch_taken() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(1);
        cpu.register_file[2].write(2);
        cpu.pc = 68;

        cpu.bne(1, 2, -10i16 as u16);
        assert_eq!(cpu.pc, 28);
    }

    #[test]
    fn bne_branch_not_taken() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(1);
        cpu.register_file[2].write(1);
        cpu.pc = 68;

        cpu.bne(1, 2, -10i16 as u16);
        assert_eq!(cpu.pc, 68);
    }

    #[test]
    fn div() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(5);
        cpu.register_file[2].write(2);

        cpu.div(1, 2);
        assert_eq!((cpu.hi, cpu.lo), (1, 2));
    }

    #[test]
    fn div_zero() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(6);
        cpu.register_file[2].write(0);

        cpu.div(1, 2);
        assert_eq!((cpu.hi, cpu.lo), (0, 0));
    }
}
