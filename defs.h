#ifndef _TOKEN_H
typedef enum token {
  Neg,
  Add,
  Fixnum,
  Read,
  Exp,
  Program,
  Let,
  Var,
  Assign,
  REG,
  /* X86 */
  ADDQ,
  MOVQ,
  CALLQ,
  /* registers */
  RAX,
} Token;

#define _TOKEN_H
#endif
