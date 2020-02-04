#include "ast.h"
#include "parser.h"
#include <stdio.h>
#include <stdlib.h>

// parse racket to ast
ASTNode *parse_ast(char *source) {
  Parser p;
  init_parser(&p, source);
  return parse_program(&p);
}

void partial_eval(ASTNode *node) {
  switch (node->token) {
  case Neg:
    partial_eval(node->lhs);
    if (node->lhs->token == Fixnum) {
      node->token = Fixnum;
      node->value = -node->lhs->value;
      node->lhs = 0;
    }
    break;
  case Add:
    partial_eval(node->lhs);
    partial_eval(node->rhs);
    if (node->lhs->token == Fixnum && node->rhs->token == Fixnum) {
      node->token = Fixnum;
      node->value = node->lhs->value + node->rhs->value;
      node->lhs = 0;
      node->rhs = 0;
    }
    break;
  default:
    break;
  }
}

int main() {
  // char s[] = "(+ (read) (- (+ 5 3)))";
  char s[] = "(let ([x 32]) (+ (let ([x 10]) x) x))";
  ASTNode *root = parse_ast(s);
  printf("inputs:\n");
  print_ast(root);
  printf("\n");
  printf("\n");
  printf("partial eval:\n");
  partial_eval(root);
  print_ast(root);
  printf("\n");
  return 0;
}
