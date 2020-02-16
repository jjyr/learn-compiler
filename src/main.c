#include "ast.h"
#include "parser.h"
#include "pass_defs.h"
#include "printer.h"
#include "table.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

ASTNode *test(char s[]) {
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
  uniquify(root);
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
  printf("\n");
  printf("\n");
  return root;
}

int main() {
  ASTNode *root;
  root = test("(program (+ (read) (let ([x 32]) (+ (let ([x 10]) x) x))))");
  // test("(- (+ 5 3))");
  FILE *f = fopen("build/asm.S", "w+");
  if (f == NULL) {
    printf("open file error!\n");
    exit(1);
  }
  print_asm(f, root);
  fclose(f);
  return 0;
}
