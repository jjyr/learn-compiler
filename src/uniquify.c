#include "ast.h"
#include "defs.h"
#include "table.h"
#include <string.h>

char *rewrite_var(char *p, int num) {
  int len = strlen(p);
  char *buf = malloc(len + 2);
  strcpy(buf, p);
  buf[len] = num + '0';
  buf[len + 1] = '\0';
  free((void *)p);
  return buf;
}

void uniquify2(ASTNode *node, Table *t) {
  int cnt;
  int len;
  char *buf;
  switch (node->token) {
  case Program:
    uniquify2(node->lhs, t);
    break;
  case Neg:
    uniquify2(node->lhs, t);
    break;
  case Add:
    uniquify2(node->lhs, t);
    uniquify2(node->rhs, t);
    break;
  case Var:
    cnt = table_get(t, (char *)node->value);
    node->value = (size_t)rewrite_var((char *)node->value, cnt);
    break;
  case Let:
    cnt = table_get(t, (char *)node->lhs->value);
    // increase suffix
    table_store(t, (char *)node->lhs->value, cnt + 1);
    uniquify2(node->rhs, t);
    // set back suffix
    table_store(t, (char *)node->lhs->value, cnt);
    node->lhs->value = (size_t)rewrite_var((char *)node->lhs->value, cnt + 1);
    break;
  default:
    break;
  }
}

void uniquify(ASTNode *node) {
  Table t;
  table_init(&t);
  uniquify2(node, &t);
}
