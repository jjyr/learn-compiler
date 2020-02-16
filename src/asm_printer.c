#include "ast.h"
#include "defs.h"
#include "error.h"
#include <stdio.h>

void print_var(FILE *f, ASTNode *node) {
  switch (node->token) {
  case Fixnum:
    fprintf(f, "$%ld", node->value);
    break;
  case STACK_LOC:
    fprintf(f, "%ld(%%rbp)", node->value);
    break;
  case REG:
    switch (node->value) {
    case RAX:
      fprintf(f, "%%rax");
      break;
    default:
      error("unexpected reg");
    }
    break;
  default:
    fprintf(f, "\nprint_var: failed to parse token %d\n", node->token);
    exit(-1);
  }
}

void print_asm(FILE *f, ASTNode *node) {
  CallInfo *info = (CallInfo *)node->value;
  fprintf(f, ".global main\n");
  fprintf(f, "main:\n");
  fprintf(f, "PUSHQ %%rbp\n");
  fprintf(f, "MOVQ %%rsp, %%rbp\n");
  fprintf(f, "SUBQ $%d, %%rsp\n", info->variables_cnt * 8);
  node = node->rhs;
  while (node != 0) {
    switch (node->token) {
    case MOVQ:
      fprintf(f, "MOVQ ");
      print_var(f, node->lhs);
      fprintf(f, ", ");
      print_var(f, (ASTNode *)node->value);
      fprintf(f, "\n");
      break;
    case ADDQ:
      fprintf(f, "ADDQ ");
      print_var(f, node->lhs);
      fprintf(f, ", ");
      print_var(f, (ASTNode *)node->value);
      fprintf(f, "\n");
      break;
    case CALLQ:
      fprintf(f, "CALLQ %s", (char *)node->value);
      fprintf(f, "\n");
      break;
    default:
      fprintf(f, "\nprint_ast: failed to parse token %d\n", node->token);
      exit(-1);
    }
    node = node->rhs;
  }
  // print out return value
  fprintf(f, "MOVQ %%rax, %%rdi\n");
  fprintf(f, "CALLQ print_int\n");
  // end call
  fprintf(f, "ADDQ $%d, %%rsp\n", info->variables_cnt * 8);
  fprintf(f, "MOVQ $0, %%rax\n");
  fprintf(f, "POPQ %%rbp\n");
  fprintf(f, "retq\n");
}
