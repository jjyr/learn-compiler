/* translate variables to stack */

#include "ast.h"
#include "defs.h"
#include "error.h"
#include "table.h"
#include <stdio.h>
#include <string.h>

static int assign_count = 0;

/* assign variables to stack */
size_t assign_loc(size_t v, Table *t) {
  size_t n = table_get(t, (char *)v);
  if (n == 0) {
    /* assign a stack location to var */
    assign_count++;
    n = assign_count;
    table_store(t, (char *)v, assign_count);
  }
  return n;
}

void assign_homes_exp(ASTNode *node, Table *t) {
  switch (node->token) {
  case Add:
    assign_homes_exp(node->lhs, t);
    assign_homes_exp(node->rhs, t);
    break;
  case Var:
    node->value = assign_loc(node->value, t);
    break;
  case Read:
  case REG:
  case Fixnum:
    break;
  default:
    error("assign exp unexpected %d", node->token);
  }
}

void assign_homes(ASTNode *node) {
  Table t;
  table_init(&t);
  while (node != 0) {
    switch (node->token) {
    case MOVQ:
    case ADDQ:
      node->value = assign_loc(node->value, &t);
      assign_homes_exp(node->lhs, &t);
    }
    node = node->rhs;
  }
}
