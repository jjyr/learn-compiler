#include "ast.h"
#include "error.h"
#include "token.h"
#include <stdio.h>

int alloc_cur = 0;

ASTNode *alloc_node() {
  if (alloc_cur == MAX_NODE) {
    error("can't allocate ASTNode, max: %d", MAX_NODE);
  }
  ASTNode *n = &ast_node_pool[alloc_cur++];
  n->lhs = 0;
  n->rhs = 0;
  n->token = 0;
  n->value = 0;
  return n;
}

void print_ast(ASTNode *node) {
  switch (node->token) {
  case Fixnum:
    printf("%ld", node->value);
    break;
  case Neg:
    printf("(- ");
    print_ast(node->lhs);
    printf(")");
    break;
  case Add:
    printf("(+ ");
    print_ast(node->lhs);
    printf(" ");
    print_ast(node->rhs);
    printf(")");
    break;
  case Read:
    printf("(read)");
    break;
  case Var:
    printf("%s", (char *)node->value);
    break;
  case Let:
    printf("(let ([%s %ld]) ", (char *)node->lhs->value, node->value);
    print_ast(node->rhs);
    printf(")");
    break;
  case Assign:
    printf("(assign %s ", (char *)node->value);
    print_ast(node->lhs);
    printf(")");
    break;
  default:
    printf("\nfailed to parse token %d\n", node->token);
    exit(-1);
  }
}
