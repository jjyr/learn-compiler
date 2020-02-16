/* translate variables to stack */

#include "ast.h"
#include "defs.h"
#include "error.h"
#include "table.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static int assign_count = 0;

/* assign variables to stack */
void assign_loc(ASTNode *val, Table *t) {
  if (val->token != Var) {
    return;
  }
  val->token = STACK_LOC;
  size_t n = table_get(t, (char *)val->value);
  if (n == 0) {
    /* assign a stack location to var */
    assign_count++;
    n = assign_count;
    table_store(t, (char *)val->value, assign_count);
  }
  val->value = -8 * n;
  return;
}

void assign_homes_exp(ASTNode *node, Table *t) {
  switch (node->token) {
  case Add:
    assign_homes_exp(node->lhs, t);
    assign_homes_exp(node->rhs, t);
    break;
  case Var:
    assign_loc(node, t);
    break;
  case Read:
  case REG:
  case Fixnum:
  case STACK_LOC:
    break;
  default:
    error("assign exp unexpected %d", node->token);
  }
}

void assign_homes(ASTNode *node) {
  ASTNode *program_node = node;
  Table t;
  table_init(&t);
  while (node != 0) {
    switch (node->token) {
    case MOVQ:
    case ADDQ:
      assign_loc((ASTNode *)node->value, &t);
      assign_homes_exp(node->lhs, &t);
    }
    node = node->rhs;
  }
  // attach call info on program node
  CallInfo *info = (CallInfo *)malloc(sizeof(CallInfo));
  info->variables_cnt = assign_count;
  program_node->value = (size_t)info;
}
