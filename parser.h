#ifndef _PARSER_H
#include "ast.h"

typedef struct parser {
  char *source;
  int cur;
  int len;
} Parser;

void init_parser(Parser *, char *source);
ASTNode *parse_program(Parser *);

#define _PARSER_H
#endif
