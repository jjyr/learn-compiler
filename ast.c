#include <stdio.h>
#include "ast.h"
#include "token.h"

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
    printf("(let ([ %s %ld]) ", (char *)node->lhs->value, node->value);
    print_ast(node->rhs);
    printf(")");
    break;
  default:
    printf("\nfailed to parse token %d\n", node->token);
    exit(-1);
  }
}

