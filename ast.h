#ifndef _AST_H
#include "token.h"

typedef struct ast_node {
    Token token;
    struct ast_node * lhs;
    struct ast_node * rhs;
    int value;
} ASTNode;

#define _AST_H
#endif
