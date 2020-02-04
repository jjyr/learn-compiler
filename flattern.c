#include "ast.h"
#include "token.h"
#include <string.h>

ASTNode *flattern_inner(ASTNode *node, ASTNode *prev);

int flattern(ASTNode *node, ASTNode *root) {
  flattern_inner(node, root);
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

ASTNode *flattern_inner(ASTNode *node, ASTNode *prev) {
  ASTNode *assign;
  ASTNode *var_node;
  switch (node->token) {
  case Neg:
    assign = alloc_node();
    assign->token = Assign;
    assign->value = alloc_var();
    assign->lhs = node;
    node->lhs = flattern_inner(node->lhs, assign);
    prev->rhs = assign;
    var_node = alloc_node();
    var_node->token = Var;
    var_node->value = assign->value;
    return var_node;
  case Add:
    assign = alloc_node();
    assign->token = Assign;
    assign->value = alloc_var();
    node->lhs = flattern_inner(node->lhs, prev);
    node->rhs = flattern_inner(node->rhs, prev->rhs);
    assign->lhs = node;
    prev->rhs->rhs = assign;
    var_node = alloc_node();
    var_node->token = Var;
    var_node->value = assign->value;
    return var_node;
  case Fixnum:
  case Var:
    return node;
  case Let:
    assign = alloc_node();
    ASTNode *num = alloc_node();
    num->token = Fixnum;
    num->value = node->value;
    assign->token = Assign;
    assign->value = node->lhs->value;
    assign->lhs = num;
    prev->rhs = assign;
    // increase suffix
    return flattern_inner(node->rhs, assign);
  default:
    error("hehe");
  }
}
