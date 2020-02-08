#include "ast.h"
#include "defs.h"
#include "error.h"
#include <stdio.h>
#include <string.h>

ASTNode *flattern2(ASTNode *node, ASTNode *prev);

ASTNode *flattern(ASTNode *node) {
  ASTNode *root = alloc_node();
  flattern2(node, root);
  return root->rhs;
}

static int cnt = 0;

char *alloc_var() {
  cnt++;
  char buf[256];
  sprintf(buf, "tmp%d", cnt);
  char *s = malloc(strlen(buf) + 1);
  strcpy(s, buf);
  return s;
}

ASTNode *create_assign_node(ASTNode *value_node, ASTNode *prev) {
  ASTNode *node = alloc_node();
  node->token = Assign;
  node->lhs = value_node;
  // use rhs to construct a linked list
  if (prev != 0) {
    prev->rhs = node;
  }
  return node;
}

/* return a Var node */
ASTNode *flattern2(ASTNode *node, ASTNode *prev) {
  ASTNode *stmt;
  ASTNode *t;
  switch (node->token) {
  case Neg:
    stmt = create_assign_node(node, prev);
    node->lhs = flattern2(node->lhs, stmt);
    stmt->value = (size_t)alloc_var();
    t = alloc_node();
    t->token = Var;
    t->value = stmt->value;
    return t;
  case Read:
    stmt = create_assign_node(node, prev);
    stmt->value = (size_t)alloc_var();
    t = alloc_node();
    t->token = Var;
    t->value = stmt->value;
    return t;
  case Add:
    stmt = create_assign_node(node, 0);
    // lhs is the next stmt after prev
    node->lhs = flattern2(node->lhs, prev);
    // lhs is the next next stmt after prev
    node->rhs = flattern2(node->rhs, prev->rhs);
    // stmt is the last stmt
    while (prev->rhs != 0)
      prev = prev->rhs;
    prev->rhs = stmt;
    stmt->value = (size_t)alloc_var();
    t = alloc_node();
    t->token = Var;
    t->value = stmt->value;
    return t;
  case Fixnum:
  case Var:
    return node;
  case Let:
    t = alloc_node();
    t->token = Fixnum;
    t->value = node->value;
    stmt = create_assign_node(t, prev);
    stmt->value = node->lhs->value;
    // increase suffix
    return flattern2(node->rhs, stmt);
  default:
    error("flattern unexpected\n");
  }
}
