#ifndef _AST_PRINTER_H
#include "ast.h"
#include "stdio.h"

void print_ast(ASTNode *node);
void print_stmt(ASTNode *node);
void print_asm(FILE *f, ASTNode *node);

#define _AST_PRINTER_H
#endif
