#include "ast.h"
#include "defs.h"
#include "error.h"
#include <stdio.h>

int alloc_cur = 0;

ASTNode *alloc_node() {
  if (alloc_cur == MAX_NODE) {
    error("can't allocate ASTNode, max: %d", MAX_NODE);
  }
  ASTNode *n = &ast_node_pool[alloc_cur++];
  n->lhs = 0;
  n->rhs = 0;
  n->token = 0;
  n->value = 0;
  return n;
}
