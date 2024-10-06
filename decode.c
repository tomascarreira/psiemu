#include <stdio.h>
#include <inttypes.h>

#include "decode.h"

uint8_t get_opcode(uint32_t instr) {
  return instr >> 26;
}

uint8_t get_rd(uint32_t instr) {
  return (instr >> 11) & 0x1f;
}

uint8_t get_rs(uint32_t instr) {
  return (instr >> 21) & 0x1f;
}

uint8_t get_rt(uint32_t instr) {
  return (instr >> 16) & 0x1f;
}

uint16_t get_immediate(uint32_t instr) {
  return instr & 0xffff;
}

uint32_t get_instr_index(uint32_t instr) {
  return instr & 0x03ffffff;
}

uint8_t get_sa(uint32_t instr) {
  return (instr >> 6) & 0x1f;
}

uint8_t get_function(uint32_t instr) {
  return instr & 0x3f;
}

Instr decode(uint32_t instr) {
  Instr res = {0};
  res.instr = instr;
  res.opcode = get_opcode(instr);
  res.rd = get_rd(instr);
  res.rs = get_rs(instr);
  res.rt = get_rt(instr);
  res.immediate = get_immediate(instr);
  res.instr_index = get_instr_index(instr);
  res.sa = get_sa(instr);
  res.function = get_function(instr);

  switch (res.opcode) {
    case 0x00:
      switch (res.function) {
        case 0x00:
          res.mnemonic = SLL;
          break;

        case 0x02:
          res.mnemonic = SRL;
          break;

        case 0x03:
          res.mnemonic = SRA;
          break;

        case 0x04:
          res.mnemonic = SLLV;
          break;

        
        case 0x06:
          res.mnemonic = SRLV;
          break;

        case 0x07:
          res.mnemonic = SRAV;
          break;

        case 0x08:
          res.mnemonic = JR;
          break;

        case 0x09:
          res.mnemonic = JALR;
          break;

        case 0x0c:
          res.mnemonic = SYSCALL;
          break;

        case 0x0d:
          res.mnemonic = BREAK;
          break;

        case 0x10:
          res.mnemonic = MFHI;
          break;

        case 0x11:
          res.mnemonic = MTHI;
          break;

        case 0x12:
          res.mnemonic = MFLO;
          break;

        case 0x13:
          res.mnemonic = MTLO;
          break;

        case 0x18:
          res.mnemonic = MULT;
          break;

        case 0x19:
          res.mnemonic = MULTU;
          break;

        case 0x1a:
          res.mnemonic = DIV;
          break;

        case 0x1b:
          res.mnemonic = DIVU;
          break;

        case 0x20:
          res.mnemonic = ADD;
          break;

        case 0x21:
          res.mnemonic = ADDU;
          break;

        case 0x22:
          res.mnemonic = SUB;
          break;

        case 0x23:
          res.mnemonic = SUBU;
          break;

        case 0x24:
          res.mnemonic = AND;
          break;
        case 0x25:
          res.mnemonic = OR;
          break;

        case 0x26:
          res.mnemonic = XOR;
          break;

        case 0x27:
          res.mnemonic = NOR;
          break;

        case 0x2a:
          res.mnemonic = SLT;
          break;

        case 0x2b:
          res.mnemonic = SLTU;
          break;

        default:
          fprintf(stderr, "Reserved instrucion decoded opcode:%02x function:%02x\n", res.opcode, res.function);
      }
      break;

    case 0x01:
      switch (res.rt) {
        case 0x00:
          res.mnemonic = BLTZ;
          break;

        case 0x01:
          res.mnemonic = BGEZ;
          break;

        case 0x10:
          res.mnemonic = BLTZAL;
          break;

        case 0x11:
          res.mnemonic = BGEZAL;
          break;

        default:
          fprintf(stderr, "Reserved instruction decoded opcode:%02x rt:%02x\n", res.opcode, res.rt);
      }
      break;

    case 0x02:
      res.mnemonic = J;
      break;

    case 0x03:
      res.mnemonic = JAL;
      break;

    case 0x04:
      res.mnemonic = BEQ;
      break;

    case 0x05:
      res.mnemonic = BNE;
      break;

    case 0x06:
      res.mnemonic = BLEZ;
      break;

    case 0x07:
      res.mnemonic = BGTZ;
      break;

    case 0x08:
      res.mnemonic = ADDI;
      break;

    case 0x09:
      res.mnemonic = ADDIU;
      break;

    case 0x0a:
      res.mnemonic = SLTI;
      break;

    case 0x0b:
      res.mnemonic = SLTIU;
      break;

    case 0x0c:
      res.mnemonic = ANDI;
      break;

    case 0x0d:
      res.mnemonic = ORI;
      break;

    case 0x0e:
      res.mnemonic = XORI;
      break;

    case 0x0f:
      res.mnemonic = LUI;
      break;

    // TODO: disassemble Coprocessor specific instructions
    case 0x10:
      res.mnemonic = COP0;
      break;

    case 0x11:
      res.mnemonic = COP1;
      break;

    case 0x12:
      res.mnemonic = COP2;
      break;

    case 0x13:
      res.mnemonic = COP3;
      break;

    case 0x20:
      res.mnemonic = LB;
      break;

    case 0x21:
      res.mnemonic = LH;
      break;

    case 0x22:
      res.mnemonic = LWL;
      break;

    case 0x23:
      res.mnemonic = LW;
      break;

    case 0x24:
      res.mnemonic = LBU;
      break;

    case 0x25:
      res.mnemonic = LHU;
      break;

    case 0x26:
      res.mnemonic = LWR;
      break;

    case 0x28:
      res.mnemonic = SB;
      break;

    case 0x29:
      res.mnemonic = SH;
      break;

    case 0x2a:
      res.mnemonic = SWL;
      break;

    case 0x2b:
      res.mnemonic = SW;
      break;

    case 0x2e:
      res.mnemonic = SWR;
      break;

    case 0x31:
      res.mnemonic = LWC1;
      break;

    case 0x32:
      res.mnemonic = LWC2;
      break;

    case 0x33:
      res.mnemonic = LWC3;
      break;

    case 0x39:
      res.mnemonic = SWC1;
      break;

    case 0x3a:
      res.mnemonic = SWC2;
      break;

    case 0x3b:
      res.mnemonic = SWC3;
      break;

    default:
      fprintf(stderr, "Reserved instruction decoded opcode:%02x\n", res.opcode);
  }

  return res;
}

const char* mnemonic_strings[] = {
  "RESERVED",
  "ADD",
  "ADDI",
  "ADDIU",
  "ADDU",
  "AND",
  "ANDI",
  "BEQ",
  "BGEZ",
  "BGEZAL",
  "BGTZ",
  "BLEZ",
  "BLTZ",
  "BLTZAL",
  "BNE",
  "BREAK",
  "COP0",
  "COP1",
  "COP2",
  "COP3",
  "DIV",
  "DIVU",
  "J",
  "JAL",
  "JALR",
  "JR",
  "LB",
  "LBU",
  "LH",
  "LHU",
  "LUI",
  "LW",
  "LWC1",
  "LWC2",
  "LWC3",
  "LWL",
  "LWR",
  "MFHI",
  "MFLO",
  "MTHI",
  "MTLO",
  "MULT",
  "MULTU",
  "NOR",
  "OR",
  "ORI",
  "SB",
  "SH",
  "SLL",
  "SLLV",
  "SLT",
  "SLTI",
  "SLTIU",
  "SLTU",
  "SRA",
  "SRAV",
  "SRL",
  "SRLV",
  "SUB",
  "SUBU",
  "SW",
  "SWC1",
  "SWC2",
  "SWC3",
  "SWL",
  "SWR",
  "SYSCALL",
  "XOR",
  "XORI"
};
