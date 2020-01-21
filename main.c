#include "ast.h"
#include "parser.h"
#include <stdio.h>

// parse racket to ast
ASTNode *parse_ast(char *source) {
  Parser p;
  init_parser(&p, source);
  return parse_program(&p);
}

// partial evaluate root
int partial_eval(ASTNode *root) {
  // TODO
  return 1;
}

void print_ast(ASTNode *node) {
  switch (node->token) {
  case Fixnum:
    printf("%d", node->value);
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
  default:
    break;
  }
}

int main() {
  char s[] = "(+ (read) (- (+ 5 3)))";
  ASTNode *root = parse_ast(s);
  print_ast(root);
  partial_eval(root);
  return 0;
}
