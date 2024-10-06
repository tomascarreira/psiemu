#include <stdlib.h>
#include <stdio.h>

#include "common.h"

// TODO: improve jumps and branches offset disassembled
void disassemble(char* buffer, size_t size, Instr instr) {
  switch (instr.mnemonic) {
    case RESERVED:
      snprintf(buffer, size, "RESERVED");
      break;

    case ADD:
      snprintf(buffer, size, "ADD %s, %s, %s", register_names[instr.rd], register_names[instr.rs], register_names[instr.rt]);
      break;

    case ADDI:
      snprintf(buffer, size, "ADDI %s, %s, %08x", register_names[instr.rt], register_names[instr.rs], instr.immediate);
      break;

    case ADDIU:
      snprintf(buffer, size, "ADDIU %s, %s, %08x", register_names[instr.rt], register_names[instr.rs], instr.immediate);
      break;

    case ADDU:
      snprintf(buffer, size, "ADDU %s, %s, %s", register_names[instr.rd], register_names[instr.rs], register_names[instr.rt]);
      break;

    case AND:
      snprintf(buffer, size, "AND %s, %s, %s", register_names[instr.rd], register_names[instr.rs], register_names[instr.rt]);
      break;

    case ANDI:
      snprintf(buffer, size, "ANDI %s, %s, %08x", register_names[instr.rt], register_names[instr.rs], instr.immediate);
      break;

    case BEQ:
      snprintf(buffer, size, "BEQ %s, %s, %d", register_names[instr.rs], register_names[instr.rt], instr.immediate);
      break;

    case BGEZ:
      snprintf(buffer, size, "BGEZ %s, %d", register_names[instr.rs], instr.immediate);
      break;
      
    case BGEZAL:
      snprintf(buffer, size, "BGEZAL %s, %d", register_names[instr.rs], instr.immediate);
      break;

    case BGTZ:
      snprintf(buffer, size, "BGTZ %s, %d", register_names[instr.rs], instr.immediate);
      break;

    case BLEZ:
      snprintf(buffer, size, "BLEZ %s, %d", register_names[instr.rs], instr.immediate);
      break;

    case BLTZ:
      snprintf(buffer, size, "BLTZ %s, %d", register_names[instr.rs], instr.immediate);
      break;

    case BLTZAL:
      snprintf(buffer, size, "BLTZAL %s, %d", register_names[instr.rs], instr.immediate);
      break;

    case BNE:
      snprintf(buffer, size, "BNE %s, %s, %d", register_names[instr.rs], register_names[instr.rt], instr.immediate);
      break;

    case BREAK:
      snprintf(buffer, size, "BREAK");
      break;

    case COP0:
      snprintf(buffer, size, "COP0 %016x", instr.instr_index);
      break;

    case COP1:
      snprintf(buffer, size, "COP1 %016x", instr.instr_index);
      break;

    case COP2:
      snprintf(buffer, size, "COP2 %016x", instr.instr_index);
      break;

    case COP3:
      snprintf(buffer, size, "COP3 %016x", instr.instr_index);
      break;

    case DIV:
      snprintf(buffer, size, "DIV %s, %s", register_names[instr.rs], register_names[instr.rt]);
      break;

    case DIVU:
      snprintf(buffer, size, "DIVU %s, %s", register_names[instr.rs], register_names[instr.rt]);
      break;

    case J:
      snprintf(buffer, size, "J %016x", instr.instr_index);
      break;
      
    case JAL:
      snprintf(buffer, size, "JAL %016x", instr.instr_index);
      break;

    case JALR:
      if (instr.rd == 31) {
        snprintf(buffer, size, "JALR %s", register_names[instr.rs]);
      } else {
        snprintf(buffer, size, "JALR %s, %s", register_names[instr.rd], register_names[instr.rs]);
      }
      break;

    case JR:
      snprintf(buffer, size, "JR %s", register_names[instr.rs]);
      break;

    case LB:
      snprintf(buffer, size, "LB %s, %04x(%s)", register_names[instr.rt], instr.immediate, register_names[instr.rs]);
      break;

    case LBU:
      snprintf(buffer, size, "LBU %s, %04x(%s)", register_names[instr.rt], instr.immediate, register_names[instr.rs]);
      break;

    case LH:
      snprintf(buffer, size, "LH %s, %04x(%s)", register_names[instr.rt], instr.immediate, register_names[instr.rs]);
      break;

    case LHU:
      snprintf(buffer, size, "LHU %s, %04x(%s)", register_names[instr.rt], instr.immediate, register_names[instr.rs]);
      break;

    case LUI:
      snprintf(buffer, size, "LUI %s, %04x", register_names[instr.rt], instr.immediate);
      break;

    case LW:
      snprintf(buffer, size, "LW %s, %04x(%s)", register_names[instr.rt], instr.immediate, register_names[instr.rs]);
      break;

    case LWC1:
      snprintf(buffer, size, "LWC1 %s, %04x(%s)", register_names[instr.rt], instr.immediate, register_names[instr.rs]);
      break;

    case LWC2:
      snprintf(buffer, size, "LWC2 %s, %04x(%s)", register_names[instr.rt], instr.immediate, register_names[instr.rs]);
      break;

    case LWC3:
      snprintf(buffer, size, "LWC3 %s, %04x(%s)", register_names[instr.rt], instr.immediate, register_names[instr.rs]);
      break;

    case LWL:
      snprintf(buffer, size, "LWL %s, %04x(%s)", register_names[instr.rt], instr.immediate, register_names[instr.rs]);
      break;

    case LWR:
      snprintf(buffer, size, "LWR %s, %04x(%s)", register_names[instr.rt], instr.immediate, register_names[instr.rs]);
      break;

    case MFHI:
      snprintf(buffer, size, "MFHI %s", register_names[instr.rd]);
      break;

    case MFLO:
      snprintf(buffer, size, "MFLO %s", register_names[instr.rd]);
      break;

    case MTHI:
      snprintf(buffer, size, "MTHI %s", register_names[instr.rs]);
      break;

    case MTLO:
      snprintf(buffer, size, "MTLO %s", register_names[instr.rs]);
      break;

    case MULT:
      snprintf(buffer, size, "MULT %s, %s", register_names[instr.rs], register_names[instr.rt]);
      break;

    case MULTU:
      snprintf(buffer, size, "MULTU %s, %s", register_names[instr.rs], register_names[instr.rt]);
      break;

    case NOR:
      snprintf(buffer, size, "NOR %s, %s, %s", register_names[instr.rd], register_names[instr.rs], register_names[instr.rt]);
      break;

    case OR:
      snprintf(buffer, size, "OR %s, %s, %s", register_names[instr.rd], register_names[instr.rs], register_names[instr.rt]);
      break;

    case ORI:
      snprintf(buffer, size, "ORI %s, %s, %04x", register_names[instr.rt], register_names[instr.rs], instr.immediate);
      break;

    case SB:
      snprintf(buffer, size, "SB %s, %04x(%s)", register_names[instr.rt], instr.immediate, register_names[instr.rs]);
      break;

    case SH:
      snprintf(buffer, size, "SH %s, %04x(%s)", register_names[instr.rt], instr.immediate, register_names[instr.rs]);
      break;

    case SLL:
      snprintf(buffer, size, "SLL %s, %s, %02x", register_names[instr.rd], register_names[instr.rt], instr.sa);
      break;

    case SLLV:
      snprintf(buffer, size, "SLLV %s, %s, %s", register_names[instr.rd], register_names[instr.rt], register_names[instr.rs]);
      break;

    case SLT:
      snprintf(buffer, size, "SLT %s, %s, %s", register_names[instr.rd], register_names[instr.rs], register_names[instr.rt]);
      break;

    case SLTI:
      snprintf(buffer, size, "SLTI %s, %s, %04x", register_names[instr.rt], register_names[instr.rs], instr.immediate);
      break;

    case SLTIU:
      snprintf(buffer, size, "SLTIU %s, %s, %04x", register_names[instr.rt], register_names[instr.rs], instr.immediate);
      break;

    case SLTU:
      snprintf(buffer, size, "SLTU %s, %s, %s", register_names[instr.rd], register_names[instr.rs], register_names[instr.rt]);
      break;

    case SRA:
      snprintf(buffer, size, "SRA %s, %s, %02x", register_names[instr.rd], register_names[instr.rt], instr.sa);
      break;

    case SRAV:
      snprintf(buffer, size, "SRAV %s, %s, %s", register_names[instr.rd], register_names[instr.rt], register_names[instr.rs]);
      break;

    case SRL:
      snprintf(buffer, size, "SRL %s, %s, %02x", register_names[instr.rd], register_names[instr.rt], instr.sa);
      break;

    case SRLV:
      snprintf(buffer, size, "SRLV %s, %s, %s", register_names[instr.rd], register_names[instr.rt], register_names[instr.rs]);
      break;

    case SUB:
      snprintf(buffer, size, "SUB %s, %s, %s", register_names[instr.rd], register_names[instr.rs], register_names[instr.rt]);
      break;

    case SUBU:
      snprintf(buffer, size, "SUBU %s, %s, %s", register_names[instr.rd], register_names[instr.rs], register_names[instr.rt]);
      break;

    case SW:
      snprintf(buffer, size, "SW %s, %04x(%s)", register_names[instr.rt], instr.immediate, register_names[instr.rs]);
      break;

    case SWC1:
      snprintf(buffer, size, "SWC1 %s, %04x(%s)", register_names[instr.rt], instr.immediate, register_names[instr.rs]);
      break;

    case SWC2:
      snprintf(buffer, size, "SWC2 %s, %04x(%s)", register_names[instr.rt], instr.immediate, register_names[instr.rs]);
      break;

    case SWC3:
      snprintf(buffer, size, "SWC3 %s, %04x(%s)", register_names[instr.rt], instr.immediate, register_names[instr.rs]);
      break;

    case SWL:
      snprintf(buffer, size, "SWL %s, %04x(%s)", register_names[instr.rt], instr.immediate, register_names[instr.rs]);
      break;

    case SWR:
      snprintf(buffer, size, "SWR %s, %04x(%s)", register_names[instr.rt], instr.immediate, register_names[instr.rs]);
      break;

    case SYSCALL:
      snprintf(buffer, size, "SYSCALL");
      break;

    case XOR:
      snprintf(buffer, size, "XOR %s, %s, %s", register_names[instr.rd], register_names[instr.rs], register_names[instr.rt]);
      break;

    case XORI:
      snprintf(buffer, size, "XORI %s, %s, %04x", register_names[instr.rd], register_names[instr.rs], instr.immediate);
      break;
  }  
}
