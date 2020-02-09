/* translate to native opcode */

#include "ast.h"
#include "defs.h"
#include "error.h"
#include <stdio.h>
#include <string.h>

void select_inst(ASTNode *node) {
  // used for rewrite Add node
  ASTNode *assign;
  ASTNode *other;
  ASTNode *t;
  while (node != 0) {
    switch (node->lhs->token) {
    case Add:
      if (node->lhs->lhs->token == Var || node->lhs->lhs->token == Fixnum) {
        assign = node->lhs->lhs;
        other = node->lhs->rhs;
      } else {
        assign = node->lhs->rhs;
        other = node->lhs->lhs;
      }
      node->token = MOVQ;
      node->lhs = assign;
      t = alloc_node();
      t->token = ADDQ;
      t->value = node->value;
      t->lhs = other;
      t->rhs = node->rhs;
      node->rhs = t;
      // skip next
      node = node->rhs;
      break;
    case Read:
      t = alloc_node();
      t->token = REG;
      t->value = RAX;
      assign = alloc_node();
      assign->token = MOVQ;
      assign->lhs = t;
      assign->value = node->value;
      node->token = CALLQ;
      char * s = "read_int";
      node->value = (size_t)s;
      if (node->rhs != 0)
        assign->rhs = node->rhs;
      node->rhs = assign;
      // skip next
      node = node->rhs;
      break;
    case Var:
    case Fixnum:
      node->token = MOVQ;
      break;
    default:
      error("select_inst unexpected\n");
    }
    node = node->rhs;
  }
}
