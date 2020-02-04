#include "ast.h"
#include "error.h"
#include "token.h"
#include <stdio.h>
#include <string.h>

ASTNode *flattern_ast(ASTNode *node, ASTNode *prev);

int flattern(ASTNode *node, ASTNode *root) {
  flattern_ast(node, root);
  return 0;
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

ASTNode *create_assign_node(ASTNode *value_node, ASTNode *prev,
                            char *var_name) {
  ASTNode *node = alloc_node();
  node->token = Assign;
  // allocate a tmp name
  if (var_name == 0) {
    node->value = (size_t)alloc_var();
  } else {
    node->value = (size_t)var_name;
  }
  node->lhs = value_node;
  // use rhs to construct a linked list
  if (prev != 0) {
    prev->rhs = node;
  }
  return node;
}

/* return a Var node */
ASTNode *flattern_ast(ASTNode *node, ASTNode *prev) {
  ASTNode *stmt;
  ASTNode *t;
  switch (node->token) {
  case Neg:
    stmt = create_assign_node(node, prev, (char *)0);
    node->lhs = flattern_ast(node->lhs, stmt);
    t = alloc_node();
    t->token = Var;
    t->value = stmt->value;
    return t;
  case Add:
    stmt = create_assign_node(node, 0, (char *)0);
    // lhs is the next stmt after prev
    node->lhs = flattern_ast(node->lhs, prev);
    // lhs is the next next stmt after prev
    node->rhs = flattern_ast(node->rhs, prev->rhs);
    // stmt is the next next next stmt after prev
    prev->rhs->rhs = stmt;
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
    stmt = create_assign_node(t, prev, (char *)node->lhs->value);
    // increase suffix
    return flattern_ast(node->rhs, stmt);
  default:
    error("hehe");
  }
}
