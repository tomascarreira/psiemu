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

  uint8_t opcode = get_opcode(instr);
  res.opcode = opcode;
  switch (opcode) {
    case 0x00:
      ; uint8_t function = get_function(instr);
      res.function = function;
      switch (function) {
        case 0x00:
          res.mnemonic = SLL;
          res.rt = get_rt(instr);
          res.rd = get_rd(instr);
          res.sa = get_sa(instr);
          break;

        case 0x02:
          res.mnemonic = SRL;
          res.rt = get_rt(instr);
          res.rd = get_rd(instr);
          res.sa = get_sa(instr);
          break;

        case 0x03:
          res.mnemonic = SRA;
          res.rt = get_rt(instr);
          res.rd = get_rd(instr);
          res.sa = get_sa(instr);
          break;

        case 0x04:
          res.mnemonic = SLLV;
          res.rs = get_rs(instr);
          res.rt = get_rt(instr);
          res.rd = get_rd(instr);
          break;

        
        case 0x06:
          res.mnemonic = SRLV;
          res.rs = get_rs(instr);
          res.rt = get_rt(instr);
          res.rd = get_rd(instr);
          break;

        case 0x07:
          res.mnemonic = SRAV;
          res.rs = get_rs(instr);
          res.rt = get_rt(instr);
          res.rd = get_rd(instr);
          break;

        case 0x08:
          res.mnemonic = JR;
          res.rs = get_rs(instr);
          break;

        case 0x09:
          res.mnemonic = JALR;
          res.rs = get_rs(instr);
          res.rd = get_rd(instr);
          break;

        case 0x0c:
          res.mnemonic = SYSCALL;
          break;

        case 0x0d:
          res.mnemonic = BREAK;
          break;

        case 0x10:
          res.mnemonic = MFHI;
          res.rd = get_rd(instr);
          break;

        case 0x11:
          res.mnemonic = MTHI;
          res.rs = get_rd(instr);
          break;

        case 0x12:
          res.mnemonic = MFLO;
          res.rd = get_rd(instr);
          break;

        case 0x13:
          res.mnemonic = MTLO;
          res.rs = get_rd(instr);
          break;

        case 0x18:
          res.mnemonic = MULT;
          res.rs = get_rs(instr);
          res.rt = get_rt(instr);
          break;

        case 0x19:
          res.mnemonic = MULTU;
          res.rs = get_rs(instr);
          res.rt = get_rt(instr);
          break;

        case 0x1a:
          res.mnemonic = DIV;
          res.rs = get_rs(instr);
          res.rt = get_rt(instr);
          break;

        case 0x1b:
          res.mnemonic = DIVU;
          res.rs = get_rs(instr);
          res.rt = get_rt(instr);
          break;

        case 0x20:
          res.mnemonic = ADD;
          res.rs = get_rs(instr);
          res.rt = get_rt(instr);
          res.rd = get_rd(instr);
          break;

        case 0x21:
          res.mnemonic = ADDU;
          res.rs = get_rs(instr);
          res.rt = get_rt(instr);
          res.rd = get_rd(instr);
          break;

        case 0x22:
          res.mnemonic = SUB;
          res.rs = get_rs(instr);
          res.rt = get_rt(instr);
          res.rd = get_rd(instr);
          break;

        case 0x23:
          res.mnemonic = SUBU;
          res.rs = get_rs(instr);
          res.rt = get_rt(instr);
          res.rd = get_rd(instr);
          break;

        case 0x24:
          res.mnemonic = AND;
          res.rs = get_rs(instr);
          res.rt = get_rt(instr);
          res.rd = get_rd(instr);
          break;
        case 0x25:
          res.mnemonic = OR;
          res.rs = get_rs(instr);
          res.rt = get_rt(instr);
          res.rd = get_rd(instr);
          break;

        case 0x26:
          res.mnemonic = XOR;
          res.rs = get_rs(instr);
          res.rt = get_rt(instr);
          res.rd = get_rd(instr);
          break;

        case 0x27:
          res.mnemonic = NOR;
          res.rs = get_rs(instr);
          res.rt = get_rt(instr);
          res.rd = get_rd(instr);
          break;

        case 0x2a:
          res.mnemonic = SLT;
          res.rs = get_rs(instr);
          res.rt = get_rt(instr);
          res.rd = get_rd(instr);
          break;

        case 0x2b:
          res.mnemonic = SLTU;
          res.rs = get_rs(instr);
          res.rt = get_rt(instr);
          res.rd = get_rd(instr);
          break;

        default:
          fprintf(stderr, "Reserved instrucion decoded %02x %02x\n", opcode, function);
      }
      break;

    case 0x01:
      ; uint8_t rt = get_rt(instr);
      res.rt = rt;
      switch (rt) {
        case 0x00:
          res.mnemonic = BLTZ;
          res.rs = get_rs(instr);
          res.immediate = get_immediate(instr);
          break;

        case 0x01:
          res.mnemonic = BGEZ;
          res.rs = get_rs(instr);
          res.immediate = get_immediate(instr);
          break;

        case 0x10:
          res.mnemonic = BLTZAL;
          res.rs = get_rs(instr);
          res.immediate = get_immediate(instr);
          break;

        case 0x11:
          res.mnemonic = BGEZAL;
          res.rs = get_rs(instr);
          res.immediate = get_immediate(instr);
          break;
      }
      break;

    case 0x02:
      res.mnemonic = J;
      res.instr_index = get_instr_index(instr);
      break;

    case 0x03:
      res.mnemonic = JAL;
      res.instr_index = get_instr_index(instr);
      break;

    case 0x04:
      break;
    case 0x05:
      break;
    case 0x06:
      break;
    case 0x07:
      break;

    case 0x08:
      break;
    case 0x09:
      break;
    case 0x0a:
      break;
    case 0x0b:
      break;
    case 0x0c:
      break;
    case 0x0d:
      break;
    case 0x0e:
      break;
    case 0x0f:
      break;

    case 0x10:
      break;
    case 0x11:
      break;
    case 0x12:
      break;
    case 0x13:
      break;

    case 0x20:
      break;
    case 0x21:
      break;
    case 0x22:
      break;
    case 0x23:
      break;
    case 0x24:
      break;
    case 0x25:
      break;
    case 0x26:
      break;

    case 0x28:
      break;
    case 0x29:
      break;
    case 0x2a:
      break;
    case 0x2b:
      break;

    case 0x2e:
      break;

    case 0x31:
      break;
    case 0x32:
      break;
    case 0x33:
      break;

    case 0x39:
      break;
    case 0x3a:
      break;
    case 0x3b:
      break;

    default:
      fprintf(stderr, "Reserved instruction decode %02x\n", opcode);
  }

  return (Instr){0};
}
