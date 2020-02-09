#ifndef _DEFS_H
/* common defines */

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
  /* stack location */
  STACK_LOC,
} Token;

#define _DEFS_H
#endif
