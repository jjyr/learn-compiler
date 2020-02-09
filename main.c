#include "ast.h"
#include "ast_printer.h"
#include "parser.h"
#include "pass_defs.h"
#include "table.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// parse racket to ast
ASTNode *parse_ast(char *source) {
  Parser p;
  init_parser(&p, source);
  return parse_program(&p);
}

char *rewrite_var(char *p, int num) {
  int len = strlen(p);
  char *buf = malloc(len + 2);
  strcpy(buf, p);
  buf[len] = num + '0';
  buf[len + 1] = '\0';
  free((void *)p);
  return buf;
}

void uniquify(ASTNode *node, Table *t) {
  int cnt;
  int len;
  char *buf;
  switch (node->token) {
  case Neg:
    uniquify(node->lhs, t);
    break;
  case Add:
    uniquify(node->lhs, t);
    uniquify(node->rhs, t);
    break;
  case Var:
    cnt = table_get(t, (char *)node->value);
    node->value = (size_t)rewrite_var((char *)node->value, cnt);
    break;
  case Let:
    cnt = table_get(t, (char *)node->lhs->value);
    // increase suffix
    table_store(t, (char *)node->lhs->value, cnt + 1);
    uniquify(node->rhs, t);
    // set back suffix
    table_store(t, (char *)node->lhs->value, cnt);
    node->lhs->value = (size_t)rewrite_var((char *)node->lhs->value, cnt + 1);
    break;
  default:
    break;
  }
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

void print_stmt(ASTNode *p) {
  while (p != 0) {
    print_ast(p);
    printf("\n");
    p = p->rhs;
  }
}

int test(char s[]) {
  ASTNode *root = parse_ast(s);
  printf("inputs:\n");
  print_ast(root);
  printf("\n");
  printf("\n");
  printf("partial eval:\n");
  partial_eval(root);
  print_ast(root);
  printf("\n");
  printf("\n");
  printf("uniquify:\n");
  Table t;
  table_init(&t);
  uniquify(root, &t);
  print_ast(root);
  printf("\n");
  printf("\n");
  printf("flattern:\n");
  root = flattern(root);
  print_stmt(root);
  printf("\n");
  printf("\n");
  printf("select instructions:\n");
  select_inst(root);
  print_stmt(root);
  printf("\n");
  printf("\n");
  printf("assign homes:\n");
  assign_homes(root);
  print_stmt(root);
  printf("\n");
  printf("\n");
  printf("patch instructions:\n");
  patch_inst(root);
  print_stmt(root);
}

int main() {
  test("(+ (read) (let ([x 32]) (+ (let ([x 10]) x) x)))");
  // test("(- (+ 5 3))");
  return 0;
}
