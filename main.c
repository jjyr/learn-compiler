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
    printf("\nfailed to parse token %d\n", node->token);
    exit(-1);
  }
}

int main() {
  char s[] = "(+ (read) (- (+ 5 3)))";
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
