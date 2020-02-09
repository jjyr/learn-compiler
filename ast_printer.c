#include "ast_printer.h"
#include "ast.h"
#include "defs.h"
#include "error.h"
#include <stdio.h>

void print_var(size_t value, ASTMode mod) {
  switch (mod) {
  case VarName:
    printf("%s", (char *)value);
    break;
  case StackLoc:
    printf("(deref rbp %ld)", value * -8);
    break;
  default:
    error("unexpected value");
  }
}

void print_ast(ASTNode *node, ASTMode mod) {
  switch (node->token) {
  case Fixnum:
    printf("%ld", node->value);
    break;
  case Neg:
    printf("(- ");
    print_ast(node->lhs, mod);
    printf(")");
    break;
  case Add:
    printf("(+ ");
    print_ast(node->lhs, mod);
    printf(" ");
    print_ast(node->rhs, mod);
    printf(")");
    break;
  case Read:
    printf("(read)");
    break;
  case Var:
    print_var(node->value, mod);
    break;
  case Let:
    printf("(let ([%s %ld]) ", (char *)node->lhs->value, node->value);
    print_ast(node->rhs, mod);
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
    print_var(node->value, mod);
    printf(" ");
    print_ast(node->lhs, mod);
    printf(")");
    break;
  case MOVQ:
    printf("MOVQ ");
    print_ast(node->lhs, mod);
    printf(" ");
    print_var(node->value, mod);
    break;
  case ADDQ:
    printf("ADDQ ");
    print_ast(node->lhs, mod);
    printf(" ");
    print_var(node->value, mod);
    break;
  case CALLQ:
    printf("CALLQ %s", (char *)node->value);
    break;
  default:
    printf("\nprint_ast: failed to parse token %d\n", node->token);
    exit(-1);
  }
}
