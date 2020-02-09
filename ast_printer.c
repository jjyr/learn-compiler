#include "ast_printer.h"
#include "ast.h"
#include "defs.h"
#include "error.h"
#include <stdio.h>

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
  case STACK_LOC:
    printf("(deref RBP %ld)", node->value);
    break;
  case Let:
    printf("(let ([%s %ld]) ", (char *)node->lhs->value, node->value);
    print_ast(node->rhs);
    printf(")");
    break;
  case REG:
    printf("(reg ");
    switch (node->value) {
    case RAX:
      printf("RAX");
      break;
    default:
      error("unexpected reg");
    }
    printf(")");
    break;
  case Assign:
    printf("(assign ");
    print_ast((ASTNode *)node->value);
    printf(" ");
    print_ast(node->lhs);
    printf(")");
    break;
  case MOVQ:
    printf("MOVQ ");
    print_ast(node->lhs);
    printf(" ");
    print_ast((ASTNode *)node->value);
    break;
  case ADDQ:
    printf("ADDQ ");
    print_ast(node->lhs);
    printf(" ");
    print_ast((ASTNode *)node->value);
    break;
  case CALLQ:
    printf("CALLQ %s", (char *)node->value);
    break;
  default:
    printf("\nprint_ast: failed to parse token %d\n", node->token);
    exit(-1);
  }
}
