#ifndef _AST_PRINTER_H
#include "ast.h"

typedef enum {
  VarName,
  StackLoc,
} ASTMode;

void print_ast(ASTNode *node, ASTMode mod);

#define _AST_PRINTER_H
#endif
