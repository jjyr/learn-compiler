#include "parser.h"
#include "error.h"
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MSG_LEN 8

int alloc_cur = 0;

ASTNode *alloc_node() {
  if (alloc_cur == MAX_NODE) {
    error("can't allocate ASTNode, max: %d", MAX_NODE);
  }
  return &ast_node_pool[alloc_cur++];
}

void init_parser(Parser *p, char *source) {
  p->source = source;
  p->len = strlen(source);
  p->cur = 0;
}

static void __attribute__((noreturn)) parse_error(Parser *p, char *expected) {
  char err_msg[MSG_LEN + 1];
  memcpy(err_msg, &p->source[p->cur], MSG_LEN);
  err_msg[MSG_LEN] = '\0';
  error(
      "parse token error at %d, expected %s, but got unexpected token: '%s'\n",
      p->cur, expected, err_msg);
}

static char next_c(Parser *p) {
  if (p->cur == p->len)
    return '\0';
  while (p->source[p->cur] == ' ') {
    p->cur++;
  }
  return p->source[p->cur];
}

static int match_str(Parser *p, char *str, int consume) {
  next_c(p);
  int len = strlen(str);
  int ret = memcmp(&p->source[p->cur], str, len);
  if (ret == 0) {
    p->cur += len;
  } else if (consume) {
    parse_error(p, str);
  }
  return ret == 0;
}

static Token read_token(Parser *p) {
  char c = next_c(p);
  switch (c) {
  case 'p':
    if (match_str(p, "program", 0))
      return Program;
  case 'l':
    if (match_str(p, "let", 0))
      return Let;
  case '+':
    p->cur++;
    return Add;
  case '-':
    p->cur++;
    return Neg;
  case '0':
  case '1':
  case '2':
  case '3':
  case '4':
  case '5':
  case '6':
  case '7':
  case '8':
  case '9':
    return Fixnum;
  case 'r':
    if (match_str(p, "read", 0))
      return Read;
  default:
    return Var;
  }
}

static int read_fixnum(Parser *p) {
  int cur = p->cur;
  int i = 0;
  int n = 0;
  while (1) {
    int c = p->source[p->cur++];
    if (c > '9' || c < '0') {
      break;
    }
    n = n * 10 + i * (c - '0');
    i++;
  }
  p->cur--;
  if (i == 0) {
    parse_error(p, "fixnum");
  }
  return n;
}

static char *read_var(Parser *p) {
  int cur = p->cur;
  int i = 0;
  char *var = malloc(10);
  while (i < 9) {
    int c = p->source[p->cur++];
    if (c > 'z' || c < 'a') {
      break;
    }
    var[i] = c;
    i++;
  }
  var[i] = '\0';
  p->cur--;
  if (i == 0) {
    parse_error(p, "var");
  }
  return var;
}

static ASTNode *read_exp(Parser *p) {
  int in_paren = match_str(p, "(", 0);
  Token token = read_token(p);
  ASTNode *n = alloc_node();
  n->token = token;
  switch (token) {
  case Add:
    n->lhs = read_exp(p);
    n->rhs = read_exp(p);
    break;
  case Neg:
    n->lhs = read_exp(p);
    break;
  case Fixnum:
    n->value = read_fixnum(p);
    break;
  case Read:
    break;
  case Let:
    match_str(p, "(", 1);
    match_str(p, "[", 1);
    ASTNode *val_n = alloc_node();
    val_n->value = (size_t)read_var(p);
    next_c(p);
    n->value = read_fixnum(p);
    n->lhs = val_n;
    match_str(p, "]", 1);
    match_str(p, ")", 1);
    n->rhs = read_exp(p);
    break;
  case Var:
    n->value = (size_t)read_var(p);
    break;
  default:
    parse_error(p, "token");
  }
  if (in_paren)
    match_str(p, ")", 1);
  return n;
}

ASTNode *parse_program(Parser *p) { return read_exp(p); }
