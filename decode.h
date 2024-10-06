#ifndef DECODE_H
#define DECODE_H

#include <inttypes.h>

// Add COPz specific codes (MFCz, CFCz, MTCz, CTCz, BCzT, BCzF, RFE (z == 0 only?))
typedef enum {
  RESERVED,
  ADD,
  ADDI,
  ADDIU,
  ADDU,
  AND,
  ANDI,
  BEQ,
  BGEZ,
  BGEZAL,
  BGTZ,
  BLEZ,
  BLTZ,
  BLTZAL,
  BNE,
  BREAK,
  COP0,
  COP1,
  COP2,
  COP3,
  DIV,
  DIVU,
  J,
  JAL,
  JALR,
  JR,
  LB,
  LBU,
  LH,
  LHU,
  LUI,
  LW,
  LWC1,
  LWC2,
  LWC3,
  LWL,
  LWR,
  MFHI,
  MFLO,
  MTHI,
  MTLO,
  MULT,
  MULTU,
  NOR,
  OR,
  ORI,
  SB,
  SH,
  SLL,
  SLLV,
  SLT,
  SLTI,
  SLTIU,
  SLTU,
  SRA,
  SRAV,
  SRL,
  SRLV,
  SUB,
  SUBU,
  SW,
  SWC1,
  SWC2,
  SWC3,
  SWL,
  SWR,
  SYSCALL,
  XOR,
  XORI,
} Mnemonic;

typedef struct {
  Mnemonic mnemonic;
  uint32_t instr;
  uint8_t opcode;
  uint8_t rd;
  uint8_t rs;
  uint8_t rt;
  uint16_t immediate; 
  uint32_t instr_index; 
  uint8_t sa;
  uint8_t function;
} Instr;

Instr decode(uint32_t instr);
extern const char* mnemonic_strings[];
#endif
