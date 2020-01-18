#ifndef _PARSER_H
#include "ast.h"
#define MAX_NODE 1024

ASTNode ast_node_pool[MAX_NODE];

typedef struct parser {
  char *source;
  int cur;
  int len;
} Parser;

void init_parser(Parser *, char *source);
ASTNode *parse_program(Parser *);

#define _PARSER_H
#endif
