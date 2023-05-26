use parsmips::Decode;
use parsmips::Immediate as ImmediateType;
use parsmips::Jump as JumpType;
use parsmips::MipsI;
use parsmips::Register as RegisterType;

use crate::bus::Bus;

#[derive(Debug, PartialEq)]
pub enum Exception {
    Reset,
    BusError,
    AddressError,
    Overflow,
    SystemCall,
    Breakpoint,
    ReservedInstruction,
    CoprocessorUnusable,
    Interrupt,
    Debug,
}

enum MemorySpace {
    Kuseg(u32),
    Kseg0(u32),
    Kseg1(u32),
    Kseg2(u32),
}

impl MemorySpace {
    fn into_inner(self) -> u32 {
        match self {
            MemorySpace::Kuseg(address) => address,
            MemorySpace::Kseg0(address) => address,
            MemorySpace::Kseg1(address) => address,
            MemorySpace::Kseg2(address) => address,
        }
    }
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

    pub fn cpu_cycle(&mut self, bus: &mut Bus) {
        let instr = match self.fetch_decode_instruction(bus) {
            Ok(instr) => instr,
            Err(exception) => {
                self.handle_exception(exception);
                return;
            }
        };

        println!("pc:{:x} instr:{:?}", self.pc, instr);

        self.pc += 4;

        match self.execute_instruction(instr, bus) {
            Ok(_) => (),
            Err(exception) => self.handle_exception(exception),
        }
    }

    fn fetch_decode_instruction(&mut self, bus: &Bus) -> Result<MipsI, Exception> {
        let instr = self.read_word(self.pc, bus)?;
        let instr = match instr.decode() {
            Ok(instr) => instr,
            Err(e) => {
                println!("{e}");
                std::process::exit(1);
            }
        };

        match instr {
            MipsI::Tlbp(_) | MipsI::Tlbr(_) | MipsI::Tlbwi(_) | MipsI::Tlbwr(_) => {
                Err(Exception::ReservedInstruction)
            }
            _ => Ok(instr),
        }
    }

    fn execute_instruction(&mut self, instr: MipsI, bus: &mut Bus) -> Result<(), Exception> {
        match instr {
            MipsI::Add(RegisterType { rs, rt, rd, sa: _ }) => self.add(rs, rt, rd),
            MipsI::Addi(ImmediateType { rs, rt, immediate }) => self.addi(rs, rt, immediate),
            MipsI::Addiu(ImmediateType { rs, rt, immediate }) => {
                self.addiu(rs, rt, immediate);
                Ok(())
            },
            MipsI::Addu(RegisterType { rs, rt, rd, sa: _ }) => {
                self.addu(rs, rt, rd);
                Ok(())
            },
            MipsI::And(RegisterType { rs, rt, rd, sa: _ }) => {
                self.and(rs, rt, rd);
                Ok(())
            },
            MipsI::Andi(ImmediateType { rs, rt, immediate }) => {
                self.andi(rs, rt, immediate);
                Ok(())
            },
            MipsI::Bc0f(ImmediateType { rs: _, rt: _, immediate }) => {
                self.bc0f(immediate);
                Ok(())
            },
            MipsI::Bc0t(ImmediateType { rs: _, rt: _, immediate }) => {
                self.bc0t(immediate);
                Ok(())
            },
            MipsI::Bc1f(ImmediateType { rs: _, rt: _, immediate }) => {
                self.bc1f(immediate);
                Ok(())
            },
            MipsI::Bc1t(ImmediateType { rs: _, rt: _, immediate }) => {
                self.bc1t(immediate);
                Ok(())
            },
            MipsI::Bc2f(ImmediateType { rs: _, rt: _, immediate }) => {
                self.bc2f(immediate);
                Ok(())
            },
            MipsI::Bc2t(ImmediateType { rs: _, rt: _, immediate }) => {
                self.bc2t(immediate);
                Ok(())
            },
            MipsI::Bc3f(ImmediateType { rs: _, rt: _, immediate }) => {
                self.bc3f(immediate);
                Ok(())
            },
            MipsI::Bc3t(ImmediateType { rs: _, rt: _, immediate }) => {
                self.bc3t(immediate);
                Ok(())
            },
            MipsI::Beq(ImmediateType { rs, rt, immediate }) => {
                self.beq(rs, rt, immediate);
                Ok(())
            },
            MipsI::Bgez(ImmediateType { rs, rt: _, immediate }) => {
                self.bgez(rs, immediate);
                Ok(())
            },
            MipsI::Bgezal(ImmediateType { rs, rt: _, immediate }) => {
                self.bgezal(rs, immediate);
                Ok(())
            },
            MipsI::Bgtz(ImmediateType { rs, rt: _, immediate }) => {
                self.bgtz(rs, immediate);
                Ok(())
            },
            MipsI::Blez(ImmediateType { rs, rt: _, immediate }) => {
                self.blez(rs, immediate);
                Ok(())
            },
            MipsI::Bltz(ImmediateType { rs, rt: _, immediate }) => {
                self.bltz(rs, immediate);
                Ok(())
            },
            MipsI::Bltzal(ImmediateType { rs, rt: _, immediate }) => {
                self.bltzal(rs, immediate);
                Ok(())
            },
            MipsI::Bne(ImmediateType { rs, rt, immediate }) => {
                self.bne(rs, rt, immediate);
                Ok(())
            },
            MipsI::Break(RegisterType { rs: _, rt: _, rd: _, sa: _ }) => Err(self.r#break()),
            MipsI::Cfc1(_) => todo!(),
            MipsI::Cfc2(_) => todo!(),
            MipsI::Cfc3(_) => todo!(),
            MipsI::Cop0(_) => todo!(),
            MipsI::Cop1(_) => todo!(),
            MipsI::Cop2(_) => todo!(),
            MipsI::Cop3(_) => todo!(),
            MipsI::Ctc1(_) => todo!(),
            MipsI::Ctc2(_) => todo!(),
            MipsI::Ctc3(_) => todo!(),
            MipsI::Div(RegisterType { rs, rt, rd: _, sa: _ }) => {
                self.div(rs, rt);
                Ok(())
            },
            MipsI::Divu(RegisterType { rs, rt, rd: _, sa: _ }) => {
                self.divu(rs, rt);
                Ok(())
            },
            MipsI::J(JumpType { target }) => {
                self.j(target);
                Ok(())
            },
            MipsI::Jal(JumpType { target }) => {
                self.jal(target);
                Ok(())
            },
            MipsI::Jalr(RegisterType { rs, rt: _, rd, sa: _ }) => self.jalr(rs, rd),
            MipsI::Jr(RegisterType { rs, rt: _, rd: _, sa: _ }) => self.jr(rs),
            MipsI::Lb(ImmediateType { rs, rt, immediate }) => self.lb(rs, rt, immediate, bus),
            MipsI::Lbu(ImmediateType { rs, rt, immediate }) => self.lbu(rs, rt, immediate, bus),
            MipsI::Lh(ImmediateType { rs, rt, immediate }) => self.lh(rs, rt, immediate, bus),
            MipsI::Lhu(ImmediateType { rs, rt, immediate }) => self.lhu(rs, rt, immediate, bus),
            MipsI::Lui(ImmediateType { rs: _, rt, immediate }) => {
                self.lui(rt, immediate);
                Ok(())
            },
            MipsI::Lw(ImmediateType { rs, rt, immediate }) => self.lw(rs, rt, immediate, bus),
            MipsI::Lwc1(_) => todo!(),
            MipsI::Lwc2(_) => todo!(),
            MipsI::Lwc3(_) => todo!(),
            MipsI::Lwl(ImmediateType { rs, rt, immediate }) => self.lwl(rs, rt, immediate, bus),
            MipsI::Lwr(ImmediateType { rs, rt, immediate }) => self.lwr(rs, rt, immediate, bus),
            MipsI::Mfc0(_) => todo!(),
            MipsI::Mfc1(_) => todo!(),
            MipsI::Mfc2(_) => todo!(),
            MipsI::Mfc3(_) => todo!(),
            MipsI::Mfhi(RegisterType { rs: _, rt: _, rd, sa: _ }) => {
                self.mfhi(rd);
                Ok(())
            },
            MipsI::Mflo(RegisterType { rs: _, rt: _, rd, sa: _ }) => {
                self.mflo(rd);
                Ok(())
            },
            MipsI::Mtc0(_) => todo!(),
            MipsI::Mtc1(_) => todo!(),
            MipsI::Mtc2(_) => todo!(),
            MipsI::Mtc3(_) => todo!(),
            MipsI::Mthi(RegisterType { rs, rt: _, rd: _, sa: _ }) => {
                self.mthi(rs);
                Ok(())
            },
            MipsI::Mtlo(RegisterType { rs, rt: _, rd: _, sa: _ }) => {
                self.mtlo(rs);
                Ok(())
            },
            MipsI::Mult(RegisterType { rs, rt, rd: _, sa: _ }) => {
                self.mult(rs, rt);
                Ok(())
            },
            MipsI::Multu(RegisterType { rs, rt, rd: _, sa: _ }) => {
                self.multu(rs, rt);
                Ok(())
            },
            MipsI::Nor(RegisterType { rs, rt, rd, sa: _ }) => {
                self.nor(rs, rt, rd);
                Ok(())
            },
            MipsI::Or(RegisterType { rs, rt, rd, sa: _ }) => {
                self.or(rs, rt, rd);
                Ok(())
            },
            MipsI::Ori(ImmediateType { rs, rt, immediate }) => {
                self.ori(rs, rt, immediate);
                Ok(())
            },
            MipsI::Rfe(RegisterType { rs: _, rt: _, rd: _, sa: _ }) => self.rfe(),
            MipsI::Sb(ImmediateType { rs, rt, immediate }) => self.sb(rs, rt, immediate, bus),
            MipsI::Sh(ImmediateType { rs, rt, immediate }) => self.sh(rs, rt, immediate, bus),
            MipsI::Sll(RegisterType { rs: _, rt, rd, sa }) => {
                self.sll(rt, rd, sa);
                Ok(())
            },
            MipsI::Sllv(RegisterType { rs, rt, rd, sa: _ }) => {
                self.sllv(rs, rt, rd);
                Ok(())
            },
            MipsI::Slt(RegisterType { rs, rt, rd, sa: _ }) => {
                self.slt(rs, rt, rd);
                Ok(())
            },
            MipsI::Slti(ImmediateType { rs, rt, immediate }) => {
                self.slti(rs, rt, immediate);
                Ok(())
            },
            MipsI::Sltiu(ImmediateType { rs, rt, immediate }) => {
                self.sltiu(rs, rt, immediate);
                Ok(())
            },
            MipsI::Sltu(RegisterType { rs, rt, rd, sa: _ }) => {
                self.sltu(rs, rt, rd);
                Ok(())
            },
            MipsI::Sra(RegisterType { rs: _, rt, rd, sa }) => {
                self.sra(rt, rd, sa);
                Ok(())
            },
            MipsI::Srav(RegisterType { rs, rt, rd, sa: _ }) => {
                self.srav(rs, rt, rd);
                Ok(())
            },
            MipsI::Srl(RegisterType { rs: _, rt, rd, sa }) => {
                self.srl(rt, rd, sa);
                Ok(())
            },
            MipsI::Srlv(RegisterType { rs, rt, rd, sa: _ }) => {
                self.srlv(rs, rt, rd);
                Ok(())
            },
            MipsI::Sub(RegisterType { rs, rt, rd, sa: _ }) => self.sub(rs, rt, rd),
            MipsI::Subu(RegisterType { rs, rt, rd, sa: _ }) => {
                self.subu(rs, rt, rd);
                Ok(())
            },
            MipsI::Sw(ImmediateType { rs, rt, immediate }) => self.sw(rs, rt, immediate, bus),
            MipsI::Swc1(_) => todo!(),
            MipsI::Swc2(_) => todo!(),
            MipsI::Swc3(_) => todo!(),
            MipsI::Swl(ImmediateType { rs, rt, immediate }) => self.swl(rs, rt, immediate, bus),
            MipsI::Swr(ImmediateType { rs, rt, immediate }) => self.swr(rs, rt, immediate, bus),
            MipsI::Syscall(RegisterType { rs: _, rt: _, rd: _, sa: _ }) => Err(self.syscall()),
            MipsI::Tlbp(_) => unreachable!(),
            MipsI::Tlbr(_) => unreachable!(),
            MipsI::Tlbwi(_) => unreachable!(),
            MipsI::Tlbwr(_) => unreachable!(),
            MipsI::Xor(RegisterType { rs, rt, rd, sa: _ }) => {
                self.xor(rs, rt, rd);
                Ok(())
            },
            MipsI::Xori(ImmediateType { rs, rt, immediate }) => {
                self.xori(rs, rt, immediate);
                Ok(())
            },
        }
    }

    fn handle_exception(&mut self, _exception: Exception) {
        todo!()
    }

    fn read_byte(&self, address: u32, bus: &Bus) -> Result<u8, Exception> {
        bus.read_byte(translate_address(address).into_inner())
    }

    fn read_halfword(&self, address: u32, bus: &Bus) -> Result<u16, Exception> {
        if (address & 0x00000001) != 0 {
            return Err(Exception::AddressError);
        }

        bus.read_halfword(translate_address(address).into_inner())
    }

    fn read_word(&self, address: u32, bus: &Bus) -> Result<u32, Exception> {
        if (address & 0x00000003) != 0 {
            return Err(Exception::AddressError);
        }

        bus.read_word(translate_address(address).into_inner())
    }

    fn write_byte(&self, address: u32, value: u8, bus: &mut Bus) -> Result<(), Exception> {
        bus.write_byte(translate_address(address).into_inner(), value)
    }

    fn write_halfword(&self, address: u32, value: u16, bus: &mut Bus) -> Result<(), Exception> {
        if (address & 0x00000001) != 0 {
            return Err(Exception::AddressError);
        }

        bus.write_halfword(translate_address(address).into_inner(), value)
    }

    fn write_word(&self, address: u32, value: u32, bus: &mut Bus) -> Result<(), Exception> {
        if (address & 0x00000003) != 0 {
            return Err(Exception::AddressError);
        }

        bus.write_word(translate_address(address).into_inner(), value)
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

    fn bc0f(&mut self, _offset: u16) {
        todo!()
    }

    fn bc0t(&mut self, _offset: u16) {
        todo!()
    }

    fn bc1f(&mut self, _offset: u16) {
        todo!()
    }

    fn bc1t(&mut self, _offset: u16) {
        todo!()
    }

    fn bc2f(&mut self, _offset: u16) {
        todo!()
    }

    fn bc2t(&mut self, _offset: u16) {
        todo!()
    }

    fn bc3f(&mut self, _offset: u16) {
        todo!()
    }

    fn bc3t(&mut self, _offset: u16) {
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
            Err(Exception::AddressError)
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
            Err(Exception::AddressError)
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

        // overflow when index is 3
        let mask = 0xffffffff_u32
            .checked_shr(((!index & 0x00000003) + 1) * 8)
            .unwrap_or(0);
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

        let mask = 0xffffffff_u32
            .checked_shl(((!index & 0x00000003) + 1) * 8)
            .unwrap_or(0);
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

    fn rfe(&mut self) -> Result<(), Exception> {
        todo!()
    }

    fn sb(&self, base: u8, rt: u8, offset: u16, bus: &mut Bus) -> Result<(), Exception> {
        let a = self.register_file[base as usize].read();
        let address = a.wrapping_add_signed(offset as i16 as i32);

        let value = self.register_file[rt as usize].read() as u8;
        self.write_byte(address, value, bus)?;
        Ok(())
    }

    fn sh(&self, base: u8, rt: u8, offset: u16, bus: &mut Bus) -> Result<(), Exception> {
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

    fn swl(&self, base: u8, rt: u8, offset: u16, bus: &mut Bus) -> Result<(), Exception> {
        let a = self.register_file[base as usize].read();
        let address = a.wrapping_add_signed(offset as i16 as i32);

        // Dont know how the cpu does it
        // I will implement as a series of
        // writes of bytes
        let mut value = self.register_file[rt as usize].read().swap_bytes();
        let index = address & 0x00000003;
        for i in 0..=index {
            self.write_byte(address - i, value as u8, bus)?;
            value >>= 8;
        }
        Ok(())
    }

    fn swr(&self, base: u8, rt: u8, offset: u16, bus: &mut Bus) -> Result<(), Exception> {
        let a = self.register_file[base as usize].read();
        let address = a.wrapping_add_signed(offset as i16 as i32);

        let mut value = self.register_file[rt as usize].read();
        let index = !address & 0x00000003;
        for i in 0..=index {
            self.write_byte(address + i, value as u8, bus)?;
            value >>= 8;
        }
        Ok(())
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

fn translate_address(address: u32) -> MemorySpace {
    // program address to physical address
    match address {
        0x00000000..=0x7fffffff => MemorySpace::Kuseg(address & 0x1fffffff),
        0x80000000..=0x9fffffff => MemorySpace::Kseg0(address & 0x1fffffff),
        0xa0000000..=0xbfffffff => MemorySpace::Kseg1(address & 0x1fffffff),
        // dont know how the translation is for kseg2 on the psx
        0xc0000000..=0xffffffff => MemorySpace::Kseg2(address),
    }
}

#[cfg(test)]
mod opcodes {
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

        assert_eq!(cpu.jalr(1, 2), Err(Exception::AddressError));
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

        assert_eq!(cpu.jr(1), Err(Exception::AddressError));
        assert_eq!(cpu.pc, 0xfffffffd);
    }

    #[test]
    fn lb() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new(vec![]);
        cpu.register_file[1].write(0);
        bus.write_word(0, 0x0000ff00).unwrap();

        assert_eq!(cpu.lb(1, 2, 1, &mut bus), Ok(()));
        assert_eq!(cpu.register_file[2].read(), -1i32 as u32);
    }

    #[test]
    fn lbu() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new(vec![]);
        cpu.register_file[1].write(0);
        bus.write_word(0, 0x00ff0000).unwrap();

        cpu.lbu(1, 2, 2, &mut bus).unwrap();
        assert_eq!(cpu.register_file[2].read(), 0xff);
    }

    #[test]
    fn lh() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new(vec![]);
        cpu.register_file[1].write(0);
        bus.write_word(0, 0xffff0000).unwrap();

        cpu.lh(1, 2, 2, &mut bus).unwrap();
        assert_eq!(cpu.register_file[2].read(), -1i32 as u32);
    }

    #[test]
    fn lhu() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new(vec![]);
        cpu.register_file[1].write(0);
        bus.write_word(0, 0x0000ffff).unwrap();

        cpu.lhu(1, 2, 0, &mut bus).unwrap();
        assert_eq!(cpu.register_file[2].read(), 0xffff);
    }

    #[test]
    fn lui() {
        let mut cpu = Cpu::new();

        cpu.lui(1, 0xffff);
        assert_eq!(cpu.register_file[1].read(), 0xffff0000);
    }

    #[test]
    fn lw() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new(vec![]);
        cpu.register_file[1].write(0);
        bus.write_word(0, 0xffffffff).unwrap();

        cpu.lw(1, 2, 0, &mut bus).unwrap();
        assert_eq!(cpu.register_file[2].read(), 0xffffffff);
    }

    #[test]
    fn lwl() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new(vec![]);
        cpu.register_file[1].write(0);
        cpu.register_file[2].write(0x69696969);
        bus.write_word(0, 0xffffffff).unwrap();

        cpu.lwl(1, 2, 3, &mut bus).unwrap();
        assert_eq!(cpu.register_file[2].read(), 0xff696969);

        cpu.register_file[2].write(0x69696969);
        cpu.lwl(1, 2, 2, &mut bus).unwrap();
        assert_eq!(cpu.register_file[2].read(), 0xffff6969);

        cpu.register_file[2].write(0x69696969);
        cpu.lwl(1, 2, 1, &mut bus).unwrap();
        assert_eq!(cpu.register_file[2].read(), 0xffffff69);

        cpu.register_file[2].write(0x69696969);
        cpu.lwl(1, 2, 0, &mut bus).unwrap();
        assert_eq!(cpu.register_file[2].read(), 0xffffffff);
    }

    #[test]
    fn lwr() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new(vec![]);
        cpu.register_file[1].write(0);
        cpu.register_file[2].write(0x69696969);
        bus.write_word(4, 0xffffffff).unwrap();

        cpu.lwr(1, 2, 7, &mut bus).unwrap();
        assert_eq!(cpu.register_file[2].read(), 0xffffffff);

        cpu.register_file[2].write(0x69696969);
        cpu.lwr(1, 2, 6, &mut bus).unwrap();
        assert_eq!(cpu.register_file[2].read(), 0x69ffffff);

        cpu.register_file[2].write(0x69696969);
        cpu.lwr(1, 2, 5, &mut bus).unwrap();
        assert_eq!(cpu.register_file[2].read(), 0x6969ffff);

        cpu.register_file[2].write(0x69696969);
        cpu.lwr(1, 2, 4, &mut bus).unwrap();
        assert_eq!(cpu.register_file[2].read(), 0x696969ff);
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
    fn sb() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new(vec![]);
        cpu.register_file[1].write(0);
        cpu.register_file[2].write(0x0066aaff);

        cpu.sb(1, 2, 3, &mut bus).unwrap();
        assert_eq!(bus.read_byte(3), Ok(0xff));
    }

    #[test]
    fn sh() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new(vec![]);
        cpu.register_file[1].write(0);
        cpu.register_file[2].write(0x0066aaff);

        cpu.sh(1, 2, 2, &mut bus).unwrap();
        assert_eq!(bus.read_halfword(2), Ok(0xaaff));
    }

    #[test]
    fn sh_not_aligned() {
        let cpu = Cpu::new();
        let mut bus = Bus::new(vec![]);

        assert_eq!(cpu.sh(1, 2, 1, &mut bus), Err(Exception::AddressError));
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
    fn sw() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new(vec![]);
        cpu.register_file[1].write(0);
        cpu.register_file[2].write(0xffff8888);

        cpu.sw(1, 2, 0, &mut bus).unwrap();
        assert_eq!(bus.read_word(0), Ok(0xffff8888));
    }

    #[test]
    fn sw_not_aligned() {
        let cpu = Cpu::new();
        let mut bus = Bus::new(vec![]);

        assert_eq!(cpu.sw(1, 2, 3, &mut bus), Err(Exception::AddressError));
    }

    #[test]
    fn swl() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new(vec![]);
        cpu.register_file[2].write(0x00224466);
        bus.write_word(0, 0x55555555).unwrap();

        cpu.swl(1, 2, 0, &mut bus).unwrap();
        assert_eq!(bus.read_word(0), Ok(0x55555500));

        bus.write_word(0, 0x77777777).unwrap();
        cpu.swl(1, 2, 1, &mut bus).unwrap();
        assert_eq!(bus.read_word(0), Ok(0x77770022));

        bus.write_word(0, 0x33333333).unwrap();
        cpu.swl(1, 2, 2, &mut bus).unwrap();
        assert_eq!(bus.read_word(0), Ok(0x33002244));

        bus.write_word(0, 0x11111111).unwrap();
        cpu.swl(1, 2, 3, &mut bus).unwrap();
        assert_eq!(bus.read_word(0), Ok(0x00224466));
    }

    #[test]
    fn swr() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new(vec![]);
        cpu.register_file[2].write(0x00224466);
        bus.write_word(0, 0x55555555).unwrap();

        cpu.swr(1, 2, 0, &mut bus).unwrap();
        assert_eq!(bus.read_word(0), Ok(0x00224466));

        bus.write_word(0, 0x77777777).unwrap();
        cpu.swr(1, 2, 1, &mut bus).unwrap();
        assert_eq!(bus.read_word(0), Ok(0x22446677));

        bus.write_word(0, 0x33333333).unwrap();
        cpu.swr(1, 2, 2, &mut bus).unwrap();
        assert_eq!(bus.read_word(0), Ok(0x44663333));

        bus.write_word(0, 0x11111111).unwrap();
        cpu.swr(1, 2, 3, &mut bus).unwrap();
        assert_eq!(bus.read_word(0), Ok(0x66111111));
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
