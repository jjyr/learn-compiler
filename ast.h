#ifndef _AST_H
#include "token.h"
#include <stdlib.h>

typedef struct ast_node {
    Token token;
    struct ast_node * lhs;
    struct ast_node * rhs;
    size_t value;
} ASTNode;

void print_ast(ASTNode *node);

#define _AST_H
#endif
