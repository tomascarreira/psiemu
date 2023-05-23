use crate::Bus;

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
    ReservedInstruction,
    CoprocessorUnusable,
    Interrupt,
    Debug,
}

enum MemoryArea {
    Kuseg(u32),
    Kseg0(u32),
    Kseg1(u32),
    Kseg2(u32),
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

    fn read(&self, address: u32, bus: &Bus) -> Result<u32, Exception> {
        let logical_address = match address {
            0x00000000..=0x7fffffff => MemoryArea::Kuseg(address),
            0x80000000..=0x9fffffff => MemoryArea::Kseg0(address),
            0xa0000000..=0xbfffffff => MemoryArea::Kseg1(address),
            0xc0000000..=0xffffffff => MemoryArea::Kseg2(address),
        };

        todo!()
    }

    fn write(&self, address: u32, value: u32, bus: &mut Bus) -> Result<(), Exception> {
        let logical_address = match address {
            0x00000000..=0x7fffffff => MemoryArea::Kuseg(address),
            0x80000000..=0x9fffffff => MemoryArea::Kseg0(address),
            0xa0000000..=0xbfffffff => MemoryArea::Kseg1(address),
            0xc0000000..=0xffffffff => MemoryArea::Kseg2(address),
        };

        todo!()
    }

    fn read_byte(&self, address: u32, bus: &Bus) -> Result<u8, Exception> {
        todo!()
    }

    fn read_halfword(&self, address: u32, bus: &Bus) -> Result<u16, Exception> {
        todo!()
    }

    fn read_word(&self, address: u32, bus: &Bus) -> Result<u32, Exception> {
        todo!()
    }

    fn write_byte(&self, address: u32, value: u8, bus: &Bus) -> Result<(), Exception> {
        todo!()
    }

    fn write_halfword(&self, address: u32, value: u16, bus: &Bus) -> Result<(), Exception> {
        todo!()
    }

    fn write_word(&self, address: u32, value: u32, bus: &Bus) -> Result<(), Exception> {
        todo!()
    }

    fn add(&mut self, rs: u8, rt: u8, rd: u8) -> Result<(), Exception> {
        let a = self.register_file[rs as usize].read() as i32;
        let b = self.register_file[rt as usize].read() as i32;

        // Overflow is detected for two's complement
        self.register_file[rd as usize].write(a.checked_add(b).ok_or(Exception::Overflow)? as u32);
        Ok(())
    }

    fn addi(&mut self, rs: u8, rt: u8, immediate: u16) -> Result<(), Exception> {
        let a = self.register_file[rs as usize].read() as i32;

        // sign extend the immediate to 32 bits
        self.register_file[rt as usize].write(
            a.checked_add(immediate as i16 as i32)
                .ok_or(Exception::Overflow)? as u32,
        );
        Ok(())
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
        self.register_file[31].write(self.pc + 8);
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
        self.register_file[31].write(self.pc + 8);
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

    fn divu(&mut self, rs: u8, rt: u8) {
        let a = self.register_file[rs as usize].read();
        let b = self.register_file[rt as usize].read();

        self.lo = a.checked_div(b).unwrap_or(0);
        self.hi = a.checked_rem(b).unwrap_or(0);
    }

    fn j(&mut self, target: u32) {
        self.pc = (self.pc & 0xf0000000) | (target << 2);
    }

    fn jal(&mut self, target: u32) {
        self.register_file[31].write(self.pc + 8);
        self.pc = (self.pc & 0xf0000000) | (target << 2);
    }

    fn jalr(&mut self, rs: u8, rd: u8) -> Result<(), Exception> {
        let target = self.register_file[rs as usize].read();

        self.register_file[rd as usize].write(self.pc + 8);
        self.pc = target;

        // TODO: when is this exception trapped?
        // in this instruction
        // or when fetching the next
        if target & 0x00000003 != 0 {
            Err(Exception::AddressErrorLoad)
        } else {
            Ok(())
        }
    }

    fn jr(&mut self, rs: u8) -> Result<(), Exception> {
        let target = self.register_file[rs as usize].read();

        self.pc = target;

        // TODO: when is this exception trapped?
        // in this instruction
        // or when fetching the next
        if target & 0x00000003 != 0 {
            Err(Exception::AddressErrorLoad)
        } else {
            Ok(())
        }
    }

    fn lb(&mut self, base: u8, rt: u8, offset: u16, bus: &mut Bus) -> Result<(), Exception> {
        let a = self.register_file[base as usize].read();
        let address = a.wrapping_add_signed(offset as i16 as i32);
        let value = self.read_byte(address, bus)? as i8 as i32 as u32;

        self.register_file[rt as usize].write(value);
        Ok(())
    }

    fn lbu(&mut self, base: u8, rt: u8, offset: u16, bus: &mut Bus) -> Result<(), Exception> {
        let a = self.register_file[base as usize].read();
        let address = a.wrapping_add_signed(offset as i16 as i32);
        let value = self.read_byte(address, bus)? as u32;

        self.register_file[rt as usize].write(value);
        Ok(())
    }

    fn lh(&mut self, base: u8, rt: u8, offset: u16, bus: &mut Bus) -> Result<(), Exception> {
        let a = self.register_file[base as usize].read();
        let address = a.wrapping_add_signed(offset as i16 as i32);
        let value = self.read_halfword(address, bus)? as i16 as i32 as u32;

        self.register_file[rt as usize].write(value);
        Ok(())
    }

    fn lhu(&mut self, base: u8, rt: u8, offset: u16, bus: &mut Bus) -> Result<(), Exception> {
        let a = self.register_file[base as usize].read();
        let address = a.wrapping_add_signed(offset as i16 as i32);
        let value = self.read_halfword(address, bus)? as u32;

        self.register_file[rt as usize].write(value);
        Ok(())
    }

    fn lui(&mut self, rt: u8, immediate: u16) {
        self.register_file[rt as usize].write((immediate as u32) << 16);
    }

    fn lw(&mut self, base: u8, rt: u8, offset: u16, bus: &mut Bus) -> Result<(), Exception> {
        let a = self.register_file[base as usize].read();
        let address = a.wrapping_add_signed(offset as i16 as i32);
        let value = self.read_word(address, bus)?;

        self.register_file[rt as usize].write(value);
        Ok(())
    }

    fn lwl(&mut self, base: u8, rt: u8, offset: u16, bus: &mut Bus) -> Result<(), Exception> {
        let a = self.register_file[base as usize].read();
        let address = a.wrapping_add_signed(offset as i16 as i32);
        let value = self.read_word(address & 0xfffffffc, bus)?;

        let index = address & 0x00000003;
        let mask = 0xffffffff >> (index * 8);
        let value = (value & mask) << (index * 8);

        let mask = 0xffffffff >> ((!index + 1) * 8);
        let value = value | (self.register_file[rt as usize].read() & mask);

        self.register_file[rt as usize].write(value);
        Ok(())
    }

    fn lwr(&mut self, base: u8, rt: u8, offset: u16, bus: &mut Bus) -> Result<(), Exception> {
        let a = self.register_file[base as usize].read();
        let address = a.wrapping_add_signed(offset as i16 as i32);
        let value = self.read_word(address & 0xfffffffc, bus)?;

        let index = !address & 0x00000003;
        let mask = 0xffffffff << (index * 8);
        let value = (value & mask) >> (index * 8);

        let mask = 0xffffffff << ((!index + 1) * 8);
        let value = (self.register_file[rt as usize].read() & mask) | value;

        self.register_file[rt as usize].write(value);
        Ok(())
    }

    fn mfhi(&mut self, rd: u8) {
        self.register_file[rd as usize].write(self.hi);
    }

    fn mflo(&mut self, rd: u8) {
        self.register_file[rd as usize].write(self.lo);
    }

    fn mthi(&mut self, rs: u8) {
        self.hi = self.register_file[rs as usize].read();
    }

    fn mtlo(&mut self, rs: u8) {
        self.lo = self.register_file[rs as usize].read();
    }

    fn mult(&mut self, rs: u8, rt: u8) {
        let a = self.register_file[rs as usize].read() as i32 as i64;
        let b = self.register_file[rt as usize].read() as i32 as i64;

        let res = a * b;
        self.lo = res as u32;
        self.hi = (res >> 32) as u32;
    }

    fn multu(&mut self, rs: u8, rt: u8) {
        let a = self.register_file[rs as usize].read() as u64;
        let b = self.register_file[rt as usize].read() as u64;

        let res = a * b;
        self.lo = res as u32;
        self.hi = (res >> 32) as u32;
    }

    fn nor(&mut self, rs: u8, rt: u8, rd: u8) {
        let a = self.register_file[rs as usize].read();
        let b = self.register_file[rt as usize].read();

        self.register_file[rd as usize].write(!(a | b));
    }

    fn or(&mut self, rs: u8, rt: u8, rd: u8) {
        let a = self.register_file[rs as usize].read();
        let b = self.register_file[rt as usize].read();

        self.register_file[rd as usize].write(a | b);
    }

    fn ori(&mut self, rs: u8, rt: u8, immediate: u16) {
        let a = self.register_file[rs as usize].read();
        let b = immediate as u32;

        self.register_file[rt as usize].write(a | b);
    }

    fn sb(&self, base: u8, rt: u8, offset: u16, bus: &Bus) -> Result<(), Exception> {
        let a = self.register_file[base as usize].read();
        let address = a.wrapping_add_signed(offset as i16 as i32);

        let value = self.register_file[rt as usize].read() as u8;
        self.write_byte(address, value, bus)?;
        Ok(())
    }

    fn sh(&self, base: u8, rt: u8, offset: u16, bus: &Bus) -> Result<(), Exception> {
        let a = self.register_file[base as usize].read();
        let address = a.wrapping_add_signed(offset as i16 as i32);

        let value = self.register_file[rt as usize].read() as u16;
        self.write_halfword(address, value, bus)?;
        Ok(())
    }

    fn sll(&mut self, rt: u8, rd: u8, sa: u8) {
        let a = self.register_file[rt as usize].read();

        self.register_file[rd as usize].write(a << sa);
    }

    fn sllv(&mut self, rs: u8, rt: u8, rd: u8) {
        let a = self.register_file[rt as usize].read();
        let sa = self.register_file[rs as usize].read();

        self.register_file[rd as usize].write(a << sa);
    }

    fn slt(&mut self, rs: u8, rt: u8, rd: u8) {
        let a = self.register_file[rs as usize].read() as i32;
        let b = self.register_file[rt as usize].read() as i32;

        self.register_file[rd as usize].write((a < b) as u32);
    }

    fn slti(&mut self, rs: u8, rt: u8, immediate: u16) {
        let a = self.register_file[rs as usize].read() as i32;
        let b = immediate as i16 as i32;

        self.register_file[rt as usize].write((a < b) as u32);
    }

    fn sltiu(&mut self, rs: u8, rt: u8, immediate: u16) {
        let a = self.register_file[rs as usize].read();
        let b = immediate as u32;

        self.register_file[rt as usize].write((a < b) as u32);
    }

    fn sltu(&mut self, rs: u8, rt: u8, rd: u8) {
        let a = self.register_file[rs as usize].read();
        let b = self.register_file[rt as usize].read();

        self.register_file[rd as usize].write((a < b) as u32);
    }

    fn sra(&mut self, rt: u8, rd: u8, sa: u8) {
        let a = self.register_file[rt as usize].read() as i32;

        self.register_file[rd as usize].write((a >> sa) as u32);
    }

    fn srav(&mut self, rs: u8, rt: u8, rd: u8) {
        let a = self.register_file[rt as usize].read() as i32;
        let sa = self.register_file[rs as usize].read();

        self.register_file[rd as usize].write((a >> sa) as u32);
    }

    fn srl(&mut self, rt: u8, rd: u8, sa: u8) {
        let a = self.register_file[rt as usize].read();

        self.register_file[rd as usize].write(a >> sa);
    }

    fn srlv(&mut self, rs: u8, rt: u8, rd: u8) {
        let a = self.register_file[rt as usize].read();
        let sa = self.register_file[rs as usize].read();

        self.register_file[rd as usize].write(a >> sa);
    }

    fn sub(&mut self, rs: u8, rt: u8, rd: u8) -> Result<(), Exception> {
        let a = self.register_file[rs as usize].read() as i32;
        let b = self.register_file[rt as usize].read() as i32;

        self.register_file[rd as usize].write(a.checked_sub(b).ok_or(Exception::Overflow)? as u32);

        Ok(())
    }

    fn subu(&mut self, rs: u8, rt: u8, rd: u8) {
        let a = self.register_file[rs as usize].read();
        let b = self.register_file[rt as usize].read();

        self.register_file[rd as usize].write(a.wrapping_sub(b));
    }

    fn sw(&self, base: u8, rt: u8, offset: u16, bus: &mut Bus) -> Result<(), Exception> {
        let a = self.register_file[base as usize].read();
        let address = a.wrapping_add_signed(offset as i16 as i32);

        let value = self.register_file[rt as usize].read();
        self.write_word(address, value, bus)?;
        Ok(())
    }

    fn swl(&self, base: u8, rt: u8, offset: u16) -> Result<(), Exception> {
        todo!()
    }

    fn swr(&self, base: u8, rt: u8, offset: u16) -> Result<(), Exception> {
        todo!()
    }

    fn syscall(&self) -> Exception {
        Exception::SystemCall
    }

    fn xor(&mut self, rs: u8, rt: u8, rd: u8) {
        let a = self.register_file[rs as usize].read();
        let b = self.register_file[rt as usize].read();

        self.register_file[rd as usize].write(a ^ b);
    }

    fn xori(&mut self, rs: u8, rt: u8, immediate: u16) {
        let a = self.register_file[rs as usize].read();

        self.register_file[rt as usize].write(a ^ immediate as u32);
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

        assert_eq!(cpu.add(1, 2, 3), Ok(()));
        assert_eq!(cpu.register_file[3].read(), 69);
    }

    #[test]
    fn add_with_overflow() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(0x80000000);
        cpu.register_file[2].write(0x80000000);
        cpu.register_file[3].write(12);

        assert_eq!(cpu.add(1, 2, 3), Err(Exception::Overflow));
        assert_eq!(cpu.register_file[3].read(), 12);
    }

    #[test]
    fn addi() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(60);

        assert_eq!(cpu.addi(1, 2, 9), Ok(()));
        assert_eq!(cpu.register_file[2].read(), 69);
    }

    #[test]
    fn addi_with_overflow() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(0x80000000);
        cpu.register_file[2].write(12);

        assert_eq!(cpu.addi(1, 2, 0xffff), Err(Exception::Overflow));
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
        assert_eq!(cpu.register_file[31].read(), 24);
    }

    #[test]
    fn bgezal_branch_not_taken() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(-20i32 as u32);
        cpu.pc = 16;

        cpu.bgezal(1, 25);
        assert_eq!(cpu.pc, 16);
        assert_eq!(cpu.register_file[31].read(), 24);
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
        assert_eq!(cpu.register_file[31].read(), 24);
    }

    #[test]
    fn bltzal_branch_not_taken() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(0);
        cpu.pc = 16;

        cpu.bltzal(1, 25);
        assert_eq!(cpu.pc, 16);
        assert_eq!(cpu.register_file[31].read(), 24);
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

    #[test]
    fn divu() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(5);
        cpu.register_file[2].write(2);

        cpu.divu(1, 2);
        assert_eq!((cpu.hi, cpu.lo), (1, 2));
    }

    #[test]
    fn divu_zero() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(6);
        cpu.register_file[2].write(0);

        cpu.divu(1, 2);
        assert_eq!((cpu.hi, cpu.lo), (0, 0));
    }

    #[test]
    fn j() {
        let mut cpu = Cpu::new();
        cpu.pc = 0xbfc00000;

        cpu.j(0x03f00054);
        assert_eq!(cpu.pc, 0xbfc00150);
    }

    #[test]
    fn jal() {
        let mut cpu = Cpu::new();
        cpu.pc = 0xbfc00000;

        cpu.jal(0x03f00054);
        assert_eq!(cpu.pc, 0xbfc00150);
        assert_eq!(cpu.register_file[31].read(), 0xbfc00008);
    }

    #[test]
    fn jalr() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(0xfffffffc);
        cpu.pc = 0;

        assert_eq!(cpu.jalr(1, 2), Ok(()));
        assert_eq!(cpu.pc, 0xfffffffc);
        assert_eq!(cpu.register_file[2].read(), 0x00000008);
    }

    #[test]
    fn jalr_exception() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(0xfffffffd);
        cpu.pc = 0;

        assert_eq!(cpu.jalr(1, 2), Err(Exception::AddressErrorLoad));
        assert_eq!(cpu.pc, 0xfffffffd);
        assert_eq!(cpu.register_file[2].read(), 0x00000008);
    }

    #[test]
    fn jr() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(0xfffffffc);

        assert_eq!(cpu.jr(1), Ok(()));
        assert_eq!(cpu.pc, 0xfffffffc);
    }

    #[test]
    fn jr_exception() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(0xfffffffd);

        assert_eq!(cpu.jr(1), Err(Exception::AddressErrorLoad));
        assert_eq!(cpu.pc, 0xfffffffd);
    }

    #[test]
    fn lui() {
        let mut cpu = Cpu::new();

        cpu.lui(1, 0xffff);
        assert_eq!(cpu.register_file[1].read(), 0xffff0000);
    }

    #[test]
    fn mfhi() {
        let mut cpu = Cpu::new();
        cpu.hi = 69;

        cpu.mfhi(1);
        assert_eq!(cpu.register_file[1].read(), 69);
    }

    #[test]
    fn mflo() {
        let mut cpu = Cpu::new();
        cpu.lo = 69;

        cpu.mflo(1);
        assert_eq!(cpu.register_file[1].read(), 69);
    }

    #[test]
    fn mthi() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(33);

        cpu.mthi(1);
        assert_eq!(cpu.hi, 33);
    }

    #[test]
    fn mtlo() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(33);

        cpu.mtlo(1);
        assert_eq!(cpu.lo, 33);
    }

    #[test]
    fn mult() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(-1i32 as u32);
        cpu.register_file[2].write(-2i32 as u32);

        cpu.mult(1, 2);
        assert_eq!(cpu.lo, 2);
        assert_eq!(cpu.hi, 0);
    }

    #[test]
    fn mult_hi_not_zero() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(0x7fffffff);
        cpu.register_file[2].write(0x7fffffff);

        cpu.mult(1, 2);
        assert_eq!(cpu.lo, 0x00000001);
        assert_eq!(cpu.hi, 0x3fffffff);
    }

    #[test]
    fn multu() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(3);
        cpu.register_file[2].write(4);

        cpu.mult(1, 2);
        assert_eq!(cpu.lo, 12);
        assert_eq!(cpu.hi, 0);
    }

    #[test]
    fn multu_hi_not_zero() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(2_000_000_000);
        cpu.register_file[2].write(4);

        cpu.mult(1, 2);
        assert_eq!(cpu.lo, 0xDCD65000);
        assert_eq!(cpu.hi, 0x00000001);
    }

    #[test]
    fn nor() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(0x01234567);
        cpu.register_file[2].write(0xfedcba98);

        cpu.nor(1, 2, 3);
        assert_eq!(cpu.register_file[3].read(), 0);
    }

    #[test]
    fn or() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(0x01234567);
        cpu.register_file[2].write(0xfedcba98);

        cpu.or(1, 2, 3);
        assert_eq!(cpu.register_file[3].read(), 0xffffffff);
    }

    #[test]
    fn ori() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(0x01234567);

        cpu.ori(1, 2, 0xba98);
        assert_eq!(cpu.register_file[2].read(), 0x0123ffff);
    }

    #[test]
    fn sll() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(2);

        cpu.sll(1, 2, 1);
        assert_eq!(cpu.register_file[2].read(), 4);
    }

    #[test]
    fn sllv() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(2);
        cpu.register_file[2].write(1);

        cpu.sllv(2, 1, 3);
        assert_eq!(cpu.register_file[3].read(), 4);
    }

    #[test]
    fn slt() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(-2i32 as u32);
        cpu.register_file[2].write(-1i32 as u32);

        cpu.slt(1, 2, 3);
        assert_eq!(cpu.register_file[3].read(), 1);

        cpu.slt(2, 1, 3);
        assert_eq!(cpu.register_file[3].read(), 0);

        cpu.slt(1, 1, 3);
        assert_eq!(cpu.register_file[3].read(), 0);
    }

    #[test]
    fn slti() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(-2i32 as u32);

        cpu.slti(1, 2, -1i16 as u16);
        assert_eq!(cpu.register_file[2].read(), 1);

        cpu.slti(1, 2, -3i16 as u16);
        assert_eq!(cpu.register_file[2].read(), 0);

        cpu.slti(1, 2, -2i16 as u16);
        assert_eq!(cpu.register_file[2].read(), 0);
    }

    #[test]
    fn sltiu() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(1);

        cpu.slti(1, 2, 2);
        assert_eq!(cpu.register_file[2].read(), 1);

        cpu.slti(1, 2, 0);
        assert_eq!(cpu.register_file[2].read(), 0);

        cpu.slti(1, 2, 1);
        assert_eq!(cpu.register_file[2].read(), 0);
    }

    #[test]
    fn sltu() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(1);
        cpu.register_file[2].write(2);

        cpu.slt(1, 2, 3);
        assert_eq!(cpu.register_file[3].read(), 1);

        cpu.slt(2, 1, 3);
        assert_eq!(cpu.register_file[3].read(), 0);

        cpu.slt(1, 1, 3);
        assert_eq!(cpu.register_file[3].read(), 0);
    }

    #[test]
    fn sra() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(-4i32 as u32);

        cpu.sra(1, 2, 1);
        assert_eq!(cpu.register_file[2].read(), -2i32 as u32);

        cpu.register_file[1].write(16);

        cpu.sra(1, 2, 2);
        assert_eq!(cpu.register_file[2].read(), 4);
    }

    #[test]
    fn srav() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(-4i32 as u32);
        cpu.register_file[2].write(1);

        cpu.srav(2, 1, 3);
        assert_eq!(cpu.register_file[3].read(), -2i32 as u32);

        cpu.register_file[1].write(16);
        cpu.register_file[2].write(2);

        cpu.srav(2, 1, 3);
        assert_eq!(cpu.register_file[3].read(), 4);
    }

    #[test]
    fn srl() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(4);

        cpu.srl(1, 2, 1);
        assert_eq!(cpu.register_file[2].read(), 2);
    }

    #[test]
    fn srlv() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(4);
        cpu.register_file[2].write(1);

        cpu.srlv(2, 1, 3);
        assert_eq!(cpu.register_file[3].read(), 2);
    }

    #[test]
    fn sub() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(2);
        cpu.register_file[2].write(1);

        assert_eq!(cpu.sub(1, 2, 3), Ok(()));
        assert_eq!(cpu.register_file[3].read(), 1);
    }

    #[test]
    fn sub_with_overflow() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(0x80000000);
        cpu.register_file[2].write(1);

        assert_eq!(cpu.sub(1, 2, 3), Err(Exception::Overflow));
        assert_eq!(cpu.register_file[3].read(), 0);
    }

    #[test]
    fn subu() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(2);
        cpu.register_file[2].write(1);

        cpu.subu(1, 2, 3);
        assert_eq!(cpu.register_file[3].read(), 1);
    }

    #[test]
    fn xor() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(0x01234567);
        cpu.register_file[2].write(0xfedcba98);

        cpu.xor(1, 2, 3);
        assert_eq!(cpu.register_file[3].read(), 0xffffffff);
    }

    #[test]
    fn xori() {
        let mut cpu = Cpu::new();
        cpu.register_file[1].write(0x01234567);

        cpu.xori(1, 2, 0xba98);
        assert_eq!(cpu.register_file[2].read(), 0x0123ffff);
    }
}
