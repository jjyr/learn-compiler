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
} Token;

#define _TOKEN_H
#endif
