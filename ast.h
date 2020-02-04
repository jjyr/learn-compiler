#ifndef _AST_H
#include "token.h"
#include <stdlib.h>

#define MAX_NODE 1024

typedef struct ast_node {
    Token token;
    struct ast_node * lhs;
    struct ast_node * rhs;
    size_t value;
} ASTNode;

void print_ast(ASTNode *node);

ASTNode ast_node_pool[MAX_NODE];

ASTNode * alloc_node();

#define _AST_H
#endif
