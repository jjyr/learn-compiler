#include "parser.h"
#include "error.h"
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

static char next_char(Parser *p) {
  if (p->cur == p->len)
    return '\0';
  while (p->source[p->cur] == ' ') {
    p->cur++;
  }
  return p->source[p->cur];
}

static int try_consume(Parser *p, char *str, int must_consume) {
  next_char(p);
  int len = strlen(str);
  int ret = memcmp(&p->source[p->cur], str, len);
  if (ret == 0) {
    p->cur += len;
  } else if (must_consume) {
    parse_error(p, str);
  }
  return ret == 0;
}

static Token read_token(Parser *p) {
  char c = next_char(p);
  switch (c) {
  case 'p':
    if (try_consume(p, "program", 0))
      return Program;
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
    if (try_consume(p, "read", 0))
      return Read;
  }
  parse_error(p, "token");
}

static int read_fixnum(Parser *p) {
  char buf[32];
  int cur = p->cur;
  int i = 0;
  while (1) {
    int c = p->source[p->cur++];
    if (c > '9' || c < '0') {
      break;
    }
    buf[i++] = c - '0';
  }
  p->cur--;
  if (i == 0) {
    parse_error(p, "fixnum");
  }
  buf[i] = '\0';
  return atoi(buf);
}

static ASTNode *read_exp(Parser *p) {
  int in_paren = try_consume(p, "(", 0);
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
  default:
    parse_error(p, "token");
  }
  if (in_paren)
    try_consume(p, ")", 1);
  return n;
}

ASTNode *parse_program(Parser *p) { return read_exp(p); }
